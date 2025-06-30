<script lang="ts">
    import {AppBar} from "@skeletonlabs/skeleton";
    import {listen, type UnlistenFn} from "@tauri-apps/api/event";
    import {afterUpdate, onDestroy, onMount} from "svelte";
    import {EventChannel, type Log, type LogType} from "$lib/types";
    import {
        bigErrorStatus, connectedToMainPCB,
        ErrorStatus,
        logsPanelSize,
        logsScrollAreaSize,
        logsVisible,
    } from '$lib/stores/state';
    import {getToastStore} from "@skeletonlabs/skeleton";
    import { View, ViewOff } from 'carbon-icons-svelte';
    import { util, VIEWPORT_HEIGHT_NORMALIZING_VALUE } from '$lib';
    import { invoke } from '@tauri-apps/api/tauri';

    let unlistens: UnlistenFn[] = [];
    let logContainer: HTMLElement;
    let userHasScrolled = false;
    let logs: Log[] = [];

    let logsCountUpperLimit: number = 500;
    let logsCutAmount: number = 250;

    let colours = new Map([
      ['STATUS', 'text-surface-50'],
      ['WARNING', 'text-warning-400'],
      ['INFO', 'text-surface-300'],
      ['ERROR', 'text-error-500']
    ]);

    let filters: Record<string, boolean> = { 'STATUS': true, 'WARNING': true, 'INFO': true, 'ERROR': true }; // filter variable
    const toastStore = getToastStore();

    $: filteredLogs = logs.filter(log => filters[log.log_type]);

    function toggleFilter(type: string) {
        filters[type] = !filters[type];
    }

    function registerChannel(channel: string, log_type: LogType) {
      return listen(channel, (event: {payload: string}) => {
          if (logs.length > logsCountUpperLimit) {
              logs = logs.toSpliced(0, logsCutAmount);
          }
          logs = [{message: event.payload, log_type, timestamp: Date.now().valueOf()}, ...logs]
      });
    }

    function clearLogs() {
        logs = []
        userHasScrolled = false; // Reset scroll state when clearing
    }

    function toggleLogsVisibility() {
        logsVisible.set(!$logsVisible);
        if ($logsVisible) {
            logsPanelSize.set(30);
        } else {
            logsPanelSize.set(5);
        }
    }

    let connectionClosedMessages = [
        "ConnectionClosedByClient",
        "ConnectionClosedByServer",
        "ConnectionDropped",
        "FailedToReadFromConnection",
    ];

    onMount(async () => {
        unlistens[0] = await registerChannel(EventChannel.INFO, 'INFO');

        unlistens[1] = await listen(EventChannel.STATUS, (event: {payload: string}) => {
            if (logs.length > logsCountUpperLimit) {
                logs = logs.toSpliced(0, logsCutAmount);
            }

            logs = [{message: event.payload.split(';')[0], log_type: 'STATUS', timestamp: Date.now().valueOf()}, ...logs]

            console.log("received smth", event.payload)

            const message:string[] = event.payload.split(";");

            console.log(message)
            console.log(`bg-${message[1]}-600`)

            if (message[0].includes("Failed") || message[0].includes("mismatch")) {
                toastStore.trigger({
                    message: message[0],
                    background: 'bg-error-400',
                    autohide: false,
                });
            } else {
                toastStore.trigger({
                    message: message[0],
                    background: 'bg-surface-600',
                });
            }

            switch (message[0]) {
            case "Unsafe":
              bigErrorStatus.set(ErrorStatus.UNSAFE)
              break;
            case "Safe":
              bigErrorStatus.set(ErrorStatus.SAFE)
              break;
            }
        });

        unlistens[2] = await registerChannel(EventChannel.WARNING, 'WARNING');
        unlistens[3] = await registerChannel(EventChannel.ERROR, 'ERROR');

        unlistens[4] = await listen('shortcut_channel', (event: { payload: string }) => {
                if (event.payload === 'ClearLogs') {
                    clearLogs();
                }
            }
        );

        unlistens[5] = await listen(EventChannel.STATUS, async (event: {payload: string}) => {
            const message:string[] = event.payload.split(";");
            if (message[0] === "Status: ConnectionClosedByClient" ||
                message[0] === "Status: ConnectionClosedByServer" ||
                message[0] === "Status: ConnectionDropped" ||
                message[0] === "Status: FailedToReadFromConnection")
            {
                await invoke("disconnect");
                await invoke("connect_to_pod");
                connectedToMainPCB.set(false);
            } else if (message[0].split(":")[1].trim() === "ConnectionEstablished") {
                await invoke('send_command', {cmdName: "ConnectionEstablished", val: 0}).then(() => {
                    console.log(`Command ConnectionEstablished sent`);
                }).catch((e) => {
                    console.error(`Error sending command ConnectionEstablished: ${e}`);
                });
                util.log(`Command ConnectionEstablished sent`, EventChannel.INFO);
                connectedToMainPCB.set(true);
            }
        })

        logContainer.addEventListener('scroll', () => {
            // User has scrolled if they're not at the very top
            userHasScrolled = logContainer.scrollTop > 10; // Small threshold to account for minor scroll variations
        });
    });

    onDestroy(() => {
        unlistens.forEach(u => u());
    });

    afterUpdate(() => {
        // Only auto-scroll to top if user hasn't manually scrolled away from the top
        // This keeps new messages visible at the top
        if (!userHasScrolled) {
            logContainer.scrollTop = 0;
        }
    });
</script>

<div class="h-full grid grid-rows-[auto_1fr]">
    <AppBar padding="p-3" background="bg-surface-700 text-surface-50 ">
        <svelte:fragment slot="lead">
            <button class="text-sm" on:click={toggleLogsVisibility}>
                {#if $logsVisible}
                    <ViewOff size={16}/>
                {:else}
                    <View size={16}/>
                {/if}
            </button>
        </svelte:fragment>
        Logs
        <svelte:fragment slot="trail">
            <button class="btn rounded-lg text-sm" on:click={clearLogs}>
                Clear Logs
            </button>
            {#each Object.keys(filters) as type}
            <button class="line-through" class:active={filters[type]} on:click={() => toggleFilter(type)}>
                {type}
            </button>
            {/each}
        </svelte:fragment>
    </AppBar>

    <div class="p-1 overflow-y-auto" bind:this={logContainer}>
        {#each filteredLogs as log}
            <div class="flex items-center">
                <p class="{colours.get(log.log_type)}"><span class="font-mono font-light">[{log.timestamp}]</span>{log.message}</p>
            </div>
        {/each}
        <hr>
    </div>
</div>

<style>
    .active {
        text-decoration: none;
        color: white;
    }
</style>
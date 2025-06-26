<script lang="ts">
    import {AppBar} from "@skeletonlabs/skeleton";
    import {listen, type UnlistenFn} from "@tauri-apps/api/event";
    import {afterUpdate, onDestroy, onMount} from "svelte";
    import {EventChannel, type Log, type LogType} from "$lib/types";
    import { bigErrorStatus, ErrorStatus, logsPanelSize, logsScrollAreaSize, logsVisible } from '$lib/stores/state';
    import {getToastStore} from "@skeletonlabs/skeleton";
    import { View, ViewOff } from 'carbon-icons-svelte';
    import { VIEWPORT_HEIGHT_NORMALIZING_VALUE } from '$lib';

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
          logs = [...logs, {message: event.payload, log_type, timestamp: Date.now().valueOf()}]
      });
    }

    function clearLogs() {
        logs = []
    }

    function toggleLogsVisibility() {
        logsVisible.set(!$logsVisible);
        if ($logsVisible) {
            logsPanelSize.set(30);
        } else {
            logsPanelSize.set(5);
        }
    }

    onMount(async () => {
        unlistens[0] = await registerChannel(EventChannel.INFO, 'INFO');

        unlistens[1] = await listen(EventChannel.STATUS, (event: {payload: string}) => {
            if (logs.length > logsCountUpperLimit) {
                logs = logs.toSpliced(0, logsCutAmount);
            }

            logs = [...logs, {message: event.payload.split(';')[0], log_type: 'STATUS', timestamp: Date.now().valueOf()}]

            console.log("received smth", event.payload)

            const message:string[] = event.payload.split(";");

            console.log(message)
            console.log(`bg-${message[1]}-600`)

            if (message[0].includes("Failed")) {
                toastStore.trigger({
                    message: message[0],
                    background: 'bg-error-400',
                    autohide: false,
                });
            } else {
                toastStore.trigger({
                    message: message[0],
                    background: `bg-surface-600` || 'bg-surface-600',
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

        logContainer.addEventListener('scroll', () => {
            userHasScrolled = logContainer.scrollTop < logContainer.scrollHeight - logContainer.clientHeight;
        });
    });

    onDestroy(() =>
        unlistens.forEach(u => u())
    );

    afterUpdate(() => {
        if (!userHasScrolled) logContainer.scrollTop = logContainer.scrollHeight;
    });

    function updateLogsPanelHeight() {
        logsScrollAreaSize.set($logsPanelSize - ($logsPanelSize * 0.05 + 4.5) + window.innerHeight / VIEWPORT_HEIGHT_NORMALIZING_VALUE * 10 - 10);
    }

    onMount(() => {
        updateLogsPanelHeight();
        window.addEventListener('resize', updateLogsPanelHeight);
    });

    onDestroy(() => {
        window.removeEventListener('resize', updateLogsPanelHeight);
    });
</script>

<div class="h-full">
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
            <button class="line-through" class:active={filters['STATUS']} on:click={() => toggleFilter('STATUS')}>
                STATUS
            </button>
            <button class="line-through" class:active={filters['INFO']} on:click={() => toggleFilter('INFO')}>
                INFO
            </button>
            <button class="line-through" class:active={filters['WARNING']} on:click={() => toggleFilter('WARNING')}>
                WARNING
            </button>
            <button class="line-through" class:active={filters['ERROR']} on:click={() => toggleFilter('ERROR')}>
                ERROR
            </button>
        </svelte:fragment>
    </AppBar>

    <div class="p-1 overflow-y-auto" bind:this={logContainer} style="height: {$logsScrollAreaSize}vh;">
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
<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri';
    import {EventChannel, type NamedCommand, util} from "$lib";
    import { writable, type Writable } from 'svelte/store';
    import { getModalStore } from '@skeletonlabs/skeleton';
    import { MODAL_SETTINGS } from '$lib/types';
    import { modalBody, modalTitle } from '$lib/stores/data';
    import { connectedToMainPCB, overrideDependencies } from '$lib/stores/state';

    export let className: string = '';
    export let cmd: NamedCommand;
    export let val: number = 0;
    export let callback: (val:number) => void = () => {};
    export let text: string = '';
    export let onClickMethod: () => void = () => {};
    export let dependency: Writable<boolean> = writable<boolean>(true);
    export let dependencyMessage: string = '';
    export let dependencyTitle: string = '';
    export let icon: typeof import("svelte").SvelteComponent | null = null;
    export let iconClass: string = '';
    export let disabled: boolean = false;

    let modalStore = getModalStore();

    let send = async () => {
        if (!$connectedToMainPCB) {
            modalTitle.set("Not connected to the pod!");
            modalBody.set("Can't send commands without being connected to the pod!");
            modalStore.trigger(MODAL_SETTINGS);
            return;
        }

        if (!$overrideDependencies && dependency && !$dependency) {
            modalTitle.set(dependencyTitle);
            modalBody.set(dependencyMessage);
            modalStore.trigger(MODAL_SETTINGS);
            return;
        }

        onClickMethod();

        console.log(`Sending command: ${cmd}, value: ${val}`);
        await invoke('send_command', {cmdName: cmd, val}).then(() => {
            console.log(`Command ${cmd} sent`);
        }).catch((e) => {
            console.error(`Error sending command ${cmd}: ${e}`);
        });
        util.log(`Command ${cmd} sent`, EventChannel.INFO);
        callback(val);
    };
</script>

<button disabled={disabled} class="btn rounded-md font-number font-medium text-wrap flex-wrap {className ? className : 'bg-primary-500 text-surface-900'}"
        on:click={send}>
    {#if icon}
        <div class="mr-1">
            <svelte:component this={icon} size={20} class={iconClass}/>
        </div>
    {/if}
    {text ? text : util.snakeToCamel(cmd)}
</button>

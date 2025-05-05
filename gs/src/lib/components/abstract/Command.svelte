<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri';
    import {EventChannel, type NamedCommand, util} from "$lib";
    import { writable, type Writable } from 'svelte/store';
    import { getModalStore } from '@skeletonlabs/skeleton';
    import { MODAL_SETTINGS } from '$lib/types';
    import { modalBody, modalTitle } from '$lib/stores/data';
    import { overrideDependencies } from '$lib/stores/state';

    export let className: string = '';
    export let cmd: NamedCommand;
    export let val: number = 0;
    export let callback: (val:number) => void = () => {};
    export let text: string = '';
    export let onClickMethod: () => void = () => {};
    export let dependency: Writable<boolean> = writable<boolean>(true);
    export let dependencyMessage: string = '';
    export let dependencyTitle: string = '';

    let modalStore = getModalStore();

    let send = async () => {
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

<button class="btn rounded-md font-number font-medium text-wrap overflow-auto {className ? className : 'py-2 bg-primary-500 text-surface-900'}"
        on:click={send}>
    {text ? text : util.snakeToCamel(cmd)}
</button>

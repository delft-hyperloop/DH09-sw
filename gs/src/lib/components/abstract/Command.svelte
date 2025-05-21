<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { EventChannel, type NamedCommand, util } from "$lib";
    import { writable, type Writable } from 'svelte/store';
    import { MODAL_SETTINGS } from '$lib/types';
    import { modalBody, modalTitle } from '$lib/stores/data';
    import { overrideDependencies } from '$lib/stores/state';

    interface Props {
        className?: string;
        cmd: NamedCommand;
        val?: number;
        callback?: (val:number) => void;
        text?: string;
        onClickMethod?: () => void;
        dependency?: Writable<boolean>;
        dependencyMessage?: string;
        dependencyTitle?: string;
        icon?: typeof import("svelte").SvelteComponent | null;
    }

    let {
        className = '',
        cmd,
        val = 0,
        callback = () => {},
        text = '',
        onClickMethod = () => {},
        dependency = writable<boolean>(true),
        dependencyMessage = '',
        dependencyTitle = '',
        icon = null
    }: Props = $props();

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
        onclick={send}>
    {#if icon}
        {@const SvelteComponent = icon}
        <div class="mr-1">
            <SvelteComponent size={20}/>
        </div>
    {/if}
    {text ? text : util.snakeToCamel(cmd)}
</button>

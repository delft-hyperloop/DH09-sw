<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri';
    import {EventChannel, type NamedCommand, util} from "$lib";
    import { writable, type Writable } from 'svelte/store';
    import { MODAL_SETTINGS } from '$lib/types';

    interface Props {
        className?: string;
        cmd: NamedCommand;
        values?: number[];
        callback?: (val:number[]) => void;
        text?: string;
        onClickMethod?: () => void;
        dependency?: Writable<boolean>;
        dependencyMessage?: string;
        dependencyTitle?: string;
    }

    let {
        className = '',
        cmd,
        values = [0],
        callback = () => {},
        text = '',
        onClickMethod = () => {},
        dependency = writable<boolean>(true),
        dependencyMessage = '',
        dependencyTitle = ''
    }: Props = $props();

    let modalStore = getModalStore();

    let send = async () => {
        if (dependency && !$dependency) {
            MODAL_SETTINGS.body = dependencyMessage;
            MODAL_SETTINGS.title = dependencyTitle;
            modalStore.trigger(MODAL_SETTINGS);
            return;
        }

        onClickMethod();

        console.log(`Sending command: ${cmd}, values: ${values[0]}, ${values[1]}`);
        await invoke('send_command_64_bits', {cmdName: cmd, vals: values}).then(() => {
            console.log(`Command ${cmd} sent`);
        }).catch((e) => {
            console.error(`Error sending command ${cmd}: ${e}`);
        });
        util.log(`Command ${cmd} sent`, EventChannel.INFO);
        callback(values);
    };
</script>

<button class="btn rounded-md font-number font-medium text-wrap overflow-auto {className ? className : 'py-2 bg-primary-500 text-surface-900'}"
        onclick={send}>
    {text ? text : util.snakeToCamel(cmd)}
</button>

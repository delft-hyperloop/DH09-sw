<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri';
    import {EventChannel, util} from "$lib";
    import type {PopupSettings} from "@skeletonlabs/skeleton-svelte";

    interface Props {
        className?: string;
        cmd: 'connect_to_pod' | 'disconnect' | 'procedures' | 'save_logs';
        successCallback?: (r:any) => void;
        errorCallback?: (error:string) => void;
        textOverride?: string;
        hoverContent?: string;
        icon?: typeof import("svelte").SvelteComponent | null;
        send?: any;
    }

    let {
        className = '',
        cmd,
        successCallback = () => {},
        errorCallback = () => {},
        textOverride = '',
        hoverContent = '',
        icon = null,
        send = async () => {
        console.log(`Sending command: ${cmd}`);
        await invoke(cmd).then(r => {
            console.log(`Command ${cmd} sent with response: ` + r);
            util.log(`Command ${cmd} sent`, EventChannel.INFO);
            r ? successCallback(r) : errorCallback(r as string);
        }).catch((e) => {
            console.error(`Error sending command ${cmd}: ${e}`);
            util.log(`Command ${cmd} ERROR sending`, EventChannel.WARNING);
            errorCallback(`Error Sending Command: ${e}`);
        });
    }
    }: Props = $props();

    const popupHover: PopupSettings = {
      event: 'hover',
      target: 'popupHover',
      placement: 'top'
    };
</script>

{#if hoverContent !== ''}
    <div class="card p-4 preset-filled-secondary-500" data-popup="popupHover">
        <p>{hoverContent}</p>
        <div class="arrow preset-filled-secondary-500"></div>
    </div>
{/if}

<button class="btn *:pointer-events-none rounded-md font-number font-medium
               {className ? className : 'py-2 bg-primary-500 text-surface-900'}"
        onclick={send}
        use:popup={popupHover}>
    {#if icon}
        {@const SvelteComponent = icon}
        <SvelteComponent size={20} class="mr-1 items-center"/>
    {/if}
    {textOverride === '' ? util.snakeToCamel(cmd) : textOverride}
</button>

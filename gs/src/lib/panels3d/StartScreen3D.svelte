<script lang="ts">
    import { enteringScene, rendering } from '$lib/stores/state';
    import { fade } from 'svelte/transition';
    import { invoke } from '@tauri-apps/api/tauri';
    import { EventChannel, util } from '$lib';
    import { Circle } from 'svelte-loading-spinners';

    let transitionDuration: number = 1000;
    let waitingForPod: boolean = false;

    const connectToPod = async () => {
        waitingForPod = true;

        setTimeout(() => {
            $rendering = true;
        }, 2000);

        console.log(`Sending command: connect_to_pod`);
        await invoke("connect_to_pod").then(r => {
            console.log(`Command connect_to_pod sent with response: ` + r);
            util.log(`Command connect_to_pod sent`, EventChannel.INFO);
        }).catch((e) => {
            console.error(`Error sending command connect_to_pod: ${e}`);
            util.log(`Command connect_to_pod ERROR sending`, EventChannel.WARNING);
        });
    }
</script>

<div
    class="w-full flex flex-col items-center justify-center absolute h-full bg-black"
    transition:fade={{ duration: transitionDuration }}
    on:outroend={() => {enteringScene.set(true)}}
>
    <div
        class="justify-center items-center flex flex-col transition-all"
    >
        <img
            src="/images/logo-green-new.png"
            alt="Delft Hyperloop logo"
            class="w-[25vh] mb-5"
        />
        <span class="text-7xl text-primary-500 mb-16">
            Delft Hyperloop
        </span>
    </div>
    <button on:click={connectToPod} class="btn mb-5 text-2xl">
        Start
    </button>
    <div class="{waitingForPod ? 'visible' : 'invisible'} flex flex-row gap-4 items-center">
        <Circle color="#008564" size={30}/>
        <span class="text-xl">
            Connecting to pod...
        </span>
    </div>
</div>
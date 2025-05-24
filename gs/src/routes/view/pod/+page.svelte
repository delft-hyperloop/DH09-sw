<script lang="ts">
    import { Canvas } from '@threlte/core';
    import Scene from '$lib/panels3d/components/Scene.svelte';
    import { EventChannel, util } from '$lib';
    import { invoke } from '@tauri-apps/api/core';
    import { fade } from 'svelte/transition';
    import { enteringScene } from './PodState';

    let waitingForPod: boolean = $state(false);
    let rendering: boolean = $state(false);
    let transitionDuration: number = 1000;

    // let stores = GrandDataDistributor.getInstance().stores;
    // const podState = stores.getWritable("FSMState");

    const connectToPod = async () => {
        waitingForPod = true;

        setTimeout(() => {
            rendering = true;
        }, 2000);

        console.log(`Sending command: connect_to_pod`);
        await invoke("connect_to_pod").then(r => {
            console.log(`Command connect_to_pod sent with response: ` + r);
            util.log(`Command connect_to_pod sent`, EventChannel.INFO);
            if (r) {
                rendering = true;
            }
        }).catch((e) => {
            console.error(`Error sending command connect_to_pod: ${e}`);
            util.log(`Command connect_to_pod ERROR sending`, EventChannel.WARNING);
        });
    }
</script>


<div class="justify-center h-full w-full {rendering ? 'visible' : 'invisible'}">
    <Canvas>
        <Scene/>
    </Canvas>
</div>
{#if !rendering}
    <div
        class="w-full flex flex-col h-full items-center justify-center absolute my-10"
        transition:fade={{ duration: transitionDuration }}
        onoutroend={() => {enteringScene.set(true)}}
    >
        <div
            class="justify-center items-center flex flex-col transition-all duration-1000 ease-in-out
                    {rendering ? 'mt-10 scale-75' : 'mt-10 scale-100'}"
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
        <button onclick={connectToPod} class="btn">
            Start
        </button>
        <div class={waitingForPod ? 'visible' : 'invisible'}>
            <span>
                Connecting to pod...
            </span>
        </div>
    </div>
{/if}
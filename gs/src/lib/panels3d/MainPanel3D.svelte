<script lang="ts">
    import { Canvas } from '@threlte/core';
    import Scene from '$lib/panels3d/components/Scene.svelte';
    import { menuOpen, serverStatus } from '$lib/stores/state';
    import { Menu } from 'carbon-icons-svelte';
    import BurgerMenu from '$lib/panels3d/components/BurgerMenu.svelte';
    import { fade, fly } from 'svelte/transition';
</script>

<div class="absolute">
    {#if $menuOpen}
        <div
            in:fly={{ duration: 400, x: -500 }}
            out:fly={{ duration: 800, x: -500 }}
            class="bg-surface-900 bg-opacity-75"
        >
            <BurgerMenu/>
        </div>
    {/if}
</div>
<div class="justify-center h-full w-full {$serverStatus ? 'visible' : 'invisible'} bg-black">
    {#if !$menuOpen}
        <button
            onclick={() => {menuOpen.set(true)}}
            class="rounded-xl bg-surface-800 p-2 m-5 absolute hover:bg-surface-900"
            in:fade={{ duration: 100, delay: 200 }}
            out:fade={{ duration: 100 }}
        >
            <Menu size={20}/>
        </button>
    {/if}
<!--    <div class="absolute top-10 right-0 bg-surface-800 p-1" class:rotate-90={true}>-->
<!--        <button on:click={toggleLogs} class:rotate-180={true}>-->
<!--            Logs-->
<!--        </button>-->
<!--    </div>-->
    <Canvas>
        <Scene/>
    </Canvas>
</div>

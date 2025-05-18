<script lang="ts">
    import { Canvas } from '@threlte/core';
    import Scene from '$lib/panels3d/components/Scene.svelte';
    import { menuOpen, rendering } from '$lib/stores/state';
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
<div class="justify-center h-full w-full {$rendering ? 'visible' : 'invisible'}">
    {#if !$menuOpen}
        <button
            on:click={() => {menuOpen.set(true)}}
            class="rounded-xl bg-surface-800 p-2 m-5 absolute"
            in:fade={{ duration: 100, delay: 200 }}
            out:fade={{ duration: 100 }}
        >
            <Menu size={20}/>
        </button>
    {/if}
    <Canvas>
        <Scene/>
    </Canvas>
</div>

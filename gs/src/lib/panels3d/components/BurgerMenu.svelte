<script lang="ts">
    import { debugModeActive, menuOpen, threeDModeActive } from '$lib/stores/state';
    import { ChartLineSmooth, WatsonHealth3DMprToggle, Close } from 'carbon-icons-svelte';
    import { ViewWindow } from '$lib/util/WindowControl';
    import Icon from '@iconify/svelte';

    let btnClass: string = "w-full p-2 flex flex-row gap-2 items-center hover:bg-surface-800"
</script>

<div class="flex flex-col gap-4 p-5 h-[95vh] flex-grow">
    <div class="flex flex-row w-full justify-end">
        <button
            on:click={() => {menuOpen.set(false)}}
        >
            <Close size={20}/>
        </button>
    </div>

    <span class="text-primary-500 text-3xl mt-5 p-2 mr-16">
        Delft Hyperloop
    </span>
    <div class="w-full border-primary-500 mx-2" style="border-width: 0.5px;"/>

    {#if !$debugModeActive}
        <button
            class={btnClass}
            on:click={() => {debugModeActive.set(true)}}
        >
            <Icon icon="mdi:bug-outline" class="w-6 h-6"/>
            <span>
                Enable Debug Mode
            </span>
        </button>
    {:else}
        <button
            class={btnClass}
            on:click={() => {debugModeActive.set(false)}}
        >
            <Icon icon="mdi:bug-outline" class="w-6 h-6"/>
            <span>
                Disable Debug Mode
            </span>
        </button>
    {/if}
    <button
        class={btnClass}
        on:click={() => new ViewWindow("Charts", `/view/charts`)}
    >
        <ChartLineSmooth size={20}/>
        Graph Visualizer
    </button>
    <button
        class={btnClass}
        on:click={() => {threeDModeActive.set(false)}}
    >
        <WatsonHealth3DMprToggle size={20}/>
        <span>
            Exit 3D Mode
        </span>
    </button>
</div>
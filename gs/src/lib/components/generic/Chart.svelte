<script lang="ts">
    import {onDestroy, onMount} from 'svelte';
    import 'uplot/dist/uPlot.min.css';
    import {PlotBuffer} from "$lib";
    import {chartStore} from "$lib/stores/state";
    import { ViewWindow } from "$lib/util/WindowControl"
    import { displayedCharts, pinnedCharts } from '$lib/stores/data';
    import Pin from 'carbon-icons-svelte/lib/Pin.svelte';
    import { ShrinkScreen, TrashCan } from 'carbon-icons-svelte';

    export let title: string;
    export let background: string = "bg-surface-800";
    export let height: number = 200;
    export let chart: PlotBuffer|undefined = $chartStore.get(title);
    export let pop_up: boolean = true;
    export let pinnable: boolean = true;
    export let removable: boolean = false;
    export let display_title: boolean = true;

    let iconClass: string = "active:scale-90 hover:bg-surface-700 transition rounded-lg";

    let width: number;
    let resize = (width:number, height:number) => {
        chart?.setSize(plotContainer, width-15, height);
    }

    $: resize(width, height);
    let plotContainer: HTMLDivElement;

    onMount(async () => {
        chart?.draw(plotContainer);
        resize(width, height)
    })

    onDestroy(() => {
        chart?.destroy(plotContainer);
    });

    function pinToHomePage() {
        if ($pinnedCharts.includes(title)) {
            let index = $pinnedCharts.indexOf(title);
            let temp = $pinnedCharts;
            temp.splice(index, 1);
            pinnedCharts.set(temp);
        } else {
            let temp = $pinnedCharts;
            temp.push(title);
            pinnedCharts.set(temp);
        }
    }

    function removeFromGraphList() {
        let index = $displayedCharts.indexOf(title);
        let temp = $displayedCharts;
        temp.splice(index, 1);
        displayedCharts.set(temp);
    }
</script>

{#if chart}
    <div bind:clientWidth={width} class="flex flex-col {background} rounded-md pt-2 {width < 550 ? 'text-sm' : ''}">
        <div class="flex ml-6 mr-10 justify-between">
            <div class="flex flex-row gap-2">
                {#if display_title}
                    <h3 class="text-lg text-primary-100">{title}</h3>
                {/if}
                {#if pinnable}
                    <button
                        on:click={pinToHomePage}
                        class={iconClass}
                    >
                        <Pin size={16}/>
                    </button>
                {/if}
            </div>
            {#if pop_up}
                <button
                    on:click={() => new ViewWindow(title.replaceAll(/[^a-zA-Z0-9]/g, ""), `/view/chart/${title}`)}
                    class={iconClass}
                >
                    <ShrinkScreen size={16}/>
                </button>
            {/if}
            {#if removable}
                <button
                    on:click={removeFromGraphList}
                    class={iconClass}
                >
                        <TrashCan size={16}/>
                </button>
            {/if}
        </div>
        <div class="flex flex-col justify-center items-center w-full">
            <div class="rounded-md" bind:this={plotContainer} />
        </div>
    </div>
{:else}
    <p>CHART {title} NOT FOUND</p>
{/if}



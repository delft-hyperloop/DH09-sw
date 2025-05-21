<script lang="ts">
    import { run } from 'svelte/legacy';

    import {onDestroy, onMount} from 'svelte';
    import 'uplot/dist/uPlot.min.css';
    import {PlotBuffer} from "$lib";
    import {chartStore} from "$lib/stores/state";
    import { ViewWindow } from "$lib/util/WindowControl"
    import { displayedCharts, pinnedCharts } from '$lib/stores/data';
    import Pin from 'carbon-icons-svelte/lib/Pin.svelte';
    import { ShrinkScreen, TrashCan } from 'carbon-icons-svelte';

    interface Props {
        title: string;
        background?: string;
        height?: number;
        chart?: PlotBuffer|undefined;
        pop_up?: boolean;
        pinnable?: boolean;
        removable?: boolean;
        display_title?: boolean;
    }

    let {
        title,
        background = "bg-surface-800",
        height = 200,
        chart = $chartStore.get(title),
        pop_up = true,
        pinnable = true,
        removable = false,
        display_title = true
    }: Props = $props();

    let iconClass: string = "active:scale-90 hover:bg-surface-700 transition rounded-lg";

    let width: number = $state();
    let resize = (width:number, height:number) => {
        chart?.setSize(plotContainer, width-15, height);
    }

    run(() => {
        resize(width, height);
    });
    let plotContainer: HTMLDivElement = $state();

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
                        onclick={pinToHomePage}
                        class={iconClass}
                    >
                        <Pin size={16}/>
                    </button>
                {/if}
            </div>
            {#if pop_up}
                <button
                    onclick={() => new ViewWindow(title.replaceAll(/[^a-zA-Z0-9]/g, ""), `/view/chart/${title}`)}
                    class={iconClass}
                >
                    <ShrinkScreen size={16}/>
                </button>
            {/if}
            {#if removable}
                <button
                    onclick={removeFromGraphList}
                    class={iconClass}
                >
                        <TrashCan size={16}/>
                </button>
            {/if}
        </div>
        <div class="flex flex-col justify-center items-center w-full">
            <div class="rounded-md" bind:this={plotContainer}></div>
        </div>
    </div>
{:else}
    <p>CHART {title} NOT FOUND</p>
{/if}



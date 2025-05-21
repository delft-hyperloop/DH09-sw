<script lang="ts">
    import { Chart, Tile, TileGrid } from '$lib';
    import { chartStore, leviChartStore, powertrainChartStore, propChartStore } from '$lib/stores/state';
    import { displayedCharts } from '$lib/stores/data';
    import { writable } from 'svelte/store';
    import { onDestroy, onMount } from 'svelte';
    import GraphListCategory from '$lib/components/generic/GraphListCategory.svelte';

    let chartList = $chartStore.keys();
    let others: string[] = [];

    let tileGridColumnCount = $state(2);

    onMount(() => {
        for (const title of chartList) {
            if (!$propChartStore.includes(title) && !$leviChartStore.includes(title) && !$powertrainChartStore.includes(title)) {
                others.push(title);
            }
        }
    });

    let height = writable<number>(window.innerHeight-100);

    function updateHeight() {
        height.set(window.innerHeight-27);
    }

    onMount(() => {
        updateHeight();
        window.addEventListener('resize', updateHeight);
    });

    onDestroy(() => {
        window.removeEventListener('resize', updateHeight);
    });

    let tilegridStyle = $derived($displayedCharts.length >= tileGridColumnCount ? "1fr ".repeat(tileGridColumnCount) : "1fr ".repeat($displayedCharts.length));
</script>

<div class="flex flex-row w-full overflow-y-hidden">
    <div class="bg-surface-900 overflow-y-auto flex flex-col border-r-2 border-surface-700 w-1/5 overflow-x-hidden" style="height: {$height}px;">
        <span class="my-4 text-xl mx-6">Select Charts:</span>
        <hr class="w-full border-0 h-px"/>
        <span class="mx-6 text-lg mt-4 mb-3">Number of charts per column:</span>
        <input type="number" max="4" min="1" placeholder="2" bind:value={tileGridColumnCount}
            class="input p-1 rounded-md mx-6 mb-4 w-4/5 text-lg"
        >
        <div class="pl-5 w-full flex flex-col mb-10">
            <GraphListCategory title="Propulsion" chartList={$propChartStore}/>
            <GraphListCategory title="Levitation" chartList={$leviChartStore}/>
            <GraphListCategory title="Powertrain" chartList={$powertrainChartStore}/>
            <GraphListCategory title="Others" chartList={others}/>
        </div>
    </div>
    <TileGrid columns={tilegridStyle} rows=""
              className="w-full overflow-y-scroll mb-10 bg-surface-900 rounded-lg">
        {#if $displayedCharts.length === 0}
            <span class="text-xl m-4 font-light">Select a chart from the list to display</span>
        {/if}
        {#each $displayedCharts as title}
            <Tile containerClass="w-full">
                <Chart
                    title={title}
                    background="bg-surface-900"
                    height={400}
                    pop_up={false}
                    pinnable={false}
                    removable={true}
                />
            </Tile>
        {/each}
    </TileGrid>
</div>
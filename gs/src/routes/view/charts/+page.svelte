<script lang="ts">
    import { Chart, Tile, TileGrid } from '$lib';
    import { chartStore } from '$lib/stores/state';
    import { displayedCharts } from '$lib/stores/data';
    import { writable } from 'svelte/store';
    import { onDestroy, onMount } from 'svelte';
    import GraphListButton from '$lib/components/generic/GraphListButton.svelte';

    let chartList = $chartStore.keys();

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

    let time = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: true });

    onMount(() => {
        const interval = setInterval(() => {
            time = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: true });
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    });
</script>

<div class="flex flex-row w-full overflow-y-hidden">
    <div class="bg-surface-900 overflow-y-auto flex flex-col border-r-2 border-surface-700" style="height: {$height}px;">
        <span class="my-4 text-xl font-semibold px-7">Select Charts:</span>
        <hr class="w-full border-0 h-px"/>
        <div class="px-5 w-full flex flex-col mb-10">
            {#each chartList as title}
                    <GraphListButton title={title}/>
            {/each}
        </div>
    </div>
    <TileGrid columns="1fr {$displayedCharts.length > 1 ? '1fr' : ''}" rows=""
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
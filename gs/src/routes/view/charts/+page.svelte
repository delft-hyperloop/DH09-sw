<script lang="ts">
    import { Chart, Tile, TileGrid } from '$lib';
    import { chartStore } from '$lib/stores/state';
    import { displayedCharts } from '$lib/stores/data';
    import { writable } from 'svelte/store';
    import { onDestroy, onMount } from 'svelte';

    let chartList = $chartStore.keys();

    function updateChartList(title: string) {
        if ($displayedCharts.includes(title)) {
            let index = $displayedCharts.indexOf(title);
            let temp = $displayedCharts;
            temp.splice(index, 1);
            displayedCharts.set(temp);
        } else {
            let temp = $displayedCharts;
            temp.push(title);
            displayedCharts.set(temp);
        }
    }

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
</script>

<div class="h-full flex flex-row">
    <div class="bg-surface-600 overflow-y-auto flex flex-col gap-2 items-center" style="height: {$height}px;">
        <span>Charts:</span>
        {#each chartList as title}
            <button on:click={() => {updateChartList(title)}}>
                {title}
            </button>
        {/each}
    </div>
    <TileGrid columns="1fr {$displayedCharts.length > 1 ? '1fr' : ''}" rows="" className="w-full overflow-y-scroll mb-10">
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
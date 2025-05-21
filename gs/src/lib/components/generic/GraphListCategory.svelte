<script lang="ts">
    import GraphListButton from '$lib/components/generic/GraphListButton.svelte';
    import { displayedCharts } from '$lib/stores/data';
    import { ArrowUp } from 'carbon-icons-svelte';
    import { slide } from 'svelte/transition';

    interface Props {
        title?: string;
        chartList?: string[];
    }

    let { title = '', chartList = [] }: Props = $props();

    let showingItems: boolean = $state(false);

    function displayAllGraphs() {
        let temp = $displayedCharts;

        for (const title of chartList) {
            if (!temp.includes(title)) {
                temp = temp.filter((title) => !chartList.includes(title));
                temp = chartList.concat((temp));
                displayedCharts.set(temp);
                return;
            }
        }

        temp = temp.filter((title) => !chartList.includes(title));
        displayedCharts.set(temp);
    }
</script>

<div>
    <div class="flex flex-row mb-2">
        <div class="transition-transform duration-300 flex items-center text-center py-2" class:rotate-180={!showingItems}>
            <button onclick={() => {showingItems = !showingItems}}>
                <ArrowUp size={16}/>
            </button>
        </div>
        <button
            class="text-lg text-left font-light rounded-l-md font-number w-full text-primary-500
            p-2 hover:bg-surface-700 active:bg-surface-800 active:scale-95 transition"
            onclick={displayAllGraphs}
        >
            {title}
        </button>
    </div>
    {#if showingItems}
        <div
            class="flex flex-col ml-6 gap-2"
            transition:slide={{ duration: 250 }}
        >
            {#each chartList as title}
                <GraphListButton title={title}/>
            {/each}
        </div>
    {/if}
</div>
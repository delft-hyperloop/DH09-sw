<script lang="ts">
    import GraphListButton from '$lib/components/generic/GraphListButton.svelte';
    import { displayedCharts } from '$lib/stores/data';

    export let title: string = '';
    export let chartList: string[] = [];

    let showingItems: boolean = true;
    // TODO: Localization not showing in others

    function displayAllGraphs() {
        let temp = $displayedCharts;

        for (const title of temp) {
            if (chartList.includes(title)) {
                let index = temp.indexOf(title);
                // let temp2 = temp;
                // temp2.splice(index, 1);
                // temp = temp2;
                temp.splice(index, 1);
            }
        }

        if (showingItems) {
            temp = chartList.concat(temp);
        }
        showingItems = !showingItems;
        displayedCharts.set(temp);
    }
</script>

<div>
    <button
        class="mt-2 text-lg text-left font-light rounded-md font-number w-full
            p-2 hover:bg-surface-700 active:bg-surface-800 active:scale-95 transition"
        on:click={displayAllGraphs}
    >
        {title}
    </button>
    <div class="flex flex-col ml-5">
        {#each chartList as title}
            <GraphListButton title={title}/>
        {/each}
    </div>
</div>
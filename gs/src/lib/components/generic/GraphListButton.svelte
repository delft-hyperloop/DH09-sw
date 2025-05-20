<script lang="ts">
    import { displayedCharts } from '$lib/stores/data';

    export let title: string = ""

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
</script>

<button
    on:click={() => {updateChartList(title)}}
    class="mt-2 text-lg text-left font-light rounded-md font-number
            p-2 hover:bg-surface-700 active:bg-surface-800 active:scale-95 transition
            {$displayedCharts.includes(title) ? 'bg-surface-800' : 'bg-surface-900'}"
>
    {title}
</button>
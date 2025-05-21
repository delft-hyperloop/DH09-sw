<script lang="ts">
    import { displayedCharts } from '$lib/stores/data';

    interface Props {
        title?: string;
    }

    let { title = "" }: Props = $props();

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
    onclick={() => {updateChartList(title)}}
    class="text-lg text-left font-light rounded-l-md font-number
           p-2 hover:bg-surface-700 active:bg-surface-800 active:scale-95 transition
           {$displayedCharts.includes(title) ? 'bg-surface-800' : 'bg-surface-900'}"
>
    {title}
</button>
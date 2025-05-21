<script lang="ts">
    import {Store} from "$lib";
    import {NamedDatatypeValues} from "$lib/types";

    interface Props {
        tableArr?: any[][];
        titles?: any;
        background?: string;
    }

    let { tableArr = [], titles = ["Module", "Temp C°"], background = "bg-surface-800" }: Props = $props();
</script>


<div class="pt-2 rounded-lg1 {background}">
    <table class="text-sm w-full">
        <thead>
        <tr style="grid-template-columns: repeat({titles.length}, 1fr);" class="{background} grid text-center border-b border-b-surface-700">
            {#each titles as title, i}
                <th class="text-base text-left {i === 0 ? 'px-2' : 'px-4'}" style="font-weight: 550;">{title}</th>
            {/each}
        </tr>
        </thead>
        <tbody>
        {#each tableArr as row}
            <tr style="grid-template-columns: repeat({row.length}, 1fr);" class="grid {background} border-b border-b-surface-700">
                {#each row as cell, i}
                    <td class="{i === 0 ? 'px-2' : 'px-4'}
                           {i === row.length-1 ? '' : 'border-r border-r-primary-700'}">
                        {#if typeof cell === "number"}
                            <span class="font-mono">{cell.toFixed(2)}</span>
                        {:else if NamedDatatypeValues.includes(cell)}
                            <Store datatype={cell}/>
                        {:else}
                            {cell}
                        {/if}
                    </td>
                {/each}
            </tr>
        {/each}
        </tbody>
    </table>
</div>
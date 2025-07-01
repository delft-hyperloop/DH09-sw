<script lang="ts">
    import type {NamedDatatype} from "$lib/types";
    import {GrandDataDistributor} from "$lib";

    export let datatype: NamedDatatype;
    const store = GrandDataDistributor.getInstance().stores.getWritable(datatype);

    const STALE_THRESHOLD_MS = 5000;

    $: store;
</script>

<div class="flex flex-col">
    {#each $store as item, i}
        {#if Date.now() - item.timestamp > STALE_THRESHOLD_MS}
            <span class="text-stale">{typeof item.value === "number" ? item.value.toFixed(2) : item.value} {item.units}</span>
        {:else}
            <span class="text-{i === 0 ? 'success' : 'warning'}-400">
                {typeof item.value === "number" ? item.value.toFixed(2) : item.value} {item.units}
            </span>
        {/if}
    {/each}
</div>

<style>
.text-stale {
    color: orange;
    font-weight: bold;
}
</style>



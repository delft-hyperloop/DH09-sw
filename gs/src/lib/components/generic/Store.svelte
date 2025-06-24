<script lang="ts">
    import type {NamedDatatype} from "$lib/types";
    import {GrandDataDistributor} from "$lib";

    export let datatype: NamedDatatype;
    const store = GrandDataDistributor.getInstance().stores.getWritable(datatype);

    $: store;
</script>

<div class="flex flex-col">
    {#each $store as item, i}
        <span class="text-{i === 0 ? 'success' : 'warning'}-400">
            {typeof item.value === "number" ? item.value.toFixed(2) : item.value} {item.units}
        </span>
    {/each}
</div>



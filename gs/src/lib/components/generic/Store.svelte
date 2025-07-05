<script lang="ts">
    import type {NamedDatatype} from "$lib/types";
    import {GrandDataDistributor} from "$lib";
    import {latestTimestamp} from "$lib/stores/state";

    const STALE_DATA_TICKS = 500;
    export let datatype: NamedDatatype;
    const store = GrandDataDistributor.getInstance().stores.getWritable(datatype);
    export let dataModifier: number = 1;

    $: store;
</script>

<span class="{$latestTimestamp - $store.timestamp > STALE_DATA_TICKS ? 'text-surface-400' : ''}">
    {typeof $store.value === "number" ?
        ($store.value * dataModifier).toFixed(2) : $store.value}
</span>



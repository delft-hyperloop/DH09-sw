<script lang="ts">
    import type {NamedDatatype} from "$lib/types";
    import {GrandDataDistributor} from "$lib";
    import {latestTimestamp} from "$lib/stores/state";
    import { imdWarnings } from '$lib/types';

    const STALE_DATA_TICKS = 10_000;
    export let datatype: NamedDatatype;
    export let warningRange: [number, number] | undefined = undefined;
    export let safeRange: [number, number] | undefined = undefined;
    const store = GrandDataDistributor.getInstance().stores.getWritable(datatype);

    $: store;
    $: value = $store.value;
    $: color = '';
    if (typeof value === 'number') {
        if (safeRange && (value < safeRange[0] || value > safeRange[1])) {
            color = 'text-red-500';
        } else if (warningRange && (value < warningRange[0] || value > warningRange[1])) {
            color = 'text-orange-400';
        } else {
            color = 'text-surface-400';
        }
    } else {
        color = 'text-surface-400';
    }
</script>

<span class={color}>
    {#if datatype === 'IMDWarnings'}
        {imdWarnings[$store.value] ?? $store.value}
    {:else}
        {typeof $store.value === "number" ?
            $store.value.toFixed(2) : $store.value} {$store.units}
    {/if}
</span>



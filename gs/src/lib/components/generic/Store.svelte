<script lang="ts">
    import { run } from 'svelte/legacy';

    import type {NamedDatatype} from "$lib/types";
    import {GrandDataDistributor} from "$lib";
    import {latestTimestamp} from "$lib/stores/state";

    const STALE_DATA_TICKS = 10_000;
    interface Props {
        datatype: NamedDatatype;
    }

    let { datatype }: Props = $props();
    const store = GrandDataDistributor.getInstance().stores.getWritable(datatype);

    run(() => {
        store;
    });
</script>

<span class="text-{$latestTimestamp - $store.timestamp > STALE_DATA_TICKS ? 'surface' : $store.style === 'critical' ? 'error' : $store.style}-400">
    {typeof $store.value === "number" ?
        $store.value.toFixed(2) : $store.value} {$store.units}
</span>



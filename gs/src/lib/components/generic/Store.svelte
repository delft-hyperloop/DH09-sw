<script lang="ts">
    import type {NamedDatatype} from "$lib/types";
    import {GrandDataDistributor} from "$lib";
    import {latestTimestamp} from "$lib/stores/state";
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    const STALE_DATA_TICKS = 500;
    export let datatype: NamedDatatype;
    export let name: string;
    export let displayName: boolean = true;
    export let rangesNearValue: boolean = false;

    const store = GrandDataDistributor.getInstance().stores.getWritable(datatype);
    let range: unknown = "";
    let unit: unknown = "";

    onMount(async () => {
        unit = await invoke('get_unit_by_datatype', { datatype: datatype }).catch((e) => {
            console.error(`Couldn't get unit for ${datatype}: ${e}`);
        });
        range = await invoke('get_ranges_by_datatype_id', { datatype: datatype }).catch((e) => {
            console.error(`Couldn't get ranges for ${datatype}: ${e}`);
        });
    })

    $: store;
</script>

<div class="flex flex-col">
    <div class="flex flex-row gap-1">
        {#if displayName}
            <span>{name}: </span>
        {/if}
        <span class="{$latestTimestamp - $store.timestamp > STALE_DATA_TICKS ? 'text-surface-400' : $store.style}">
        {typeof $store.value === "number" ?
            ($store.value).toFixed(2) : $store.value}
            {#if rangesNearValue}
                {range}
            {/if}
            {unit}
    </span>
    </div>
    {#if range !== "" && !rangesNearValue}
        <span class="text-surface-400">
            Safety Range: {range}
        </span>
    {/if}
</div>


<script lang="ts">
    import {TileGrid, Tile, Chart } from "$lib"
    import Localization from '$lib/components/Localization.svelte';

    let tableArr2:any[][];
    $: tableArr2 = [
        ["Some", 0],
        ["Important", 0],
        ["Value", 0],
        ["That", 0],
        ["Could", 0],
        ["stay", 0],
        ["as a", 0],
        ["table entry", 0],
        ["here", 0]
    ]

    export const pop_up: boolean = true;
</script>

<div class="p-4">
    <h2 class="text-2xl font-semibold mb-4">Location & IMU</h2>

    <TileGrid columns="1fr 1fr" rows="">
        <Tile containerClass="col-span-2">
            <Localization showLabels={true} />
        </Tile>
        <Tile>
            <Chart title="Velocity" background="bg-surface-900" />
        </Tile>
        <Tile>
            <Chart title="Acceleration" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-start-1 col-span-2">
            <Chart title="Localisation" background="bg-surface-900" />
        </Tile>
    </TileGrid>
</div>

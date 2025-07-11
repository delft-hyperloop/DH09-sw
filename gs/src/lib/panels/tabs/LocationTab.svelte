<script lang="ts">
    import {TileGrid, Tile, Chart } from "$lib"
    import Localization from '$lib/components/Localization.svelte';
</script>

<div class="p-4">
    <h2 class="text-2xl font-semibold mb-4">Location</h2>

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
            <Chart title="Localization" background="bg-surface-900" />
        </Tile>
    </TileGrid>
</div>

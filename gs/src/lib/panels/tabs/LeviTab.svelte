<script lang="ts">
  import {Chart, TileGrid, Tile, Command} from "$lib";
  import StartLevitating from '$lib/components/StartLevitating.svelte';
  import StopLevitating from '$lib/components/StopLevitating.svelte';
  import type { SvelteComponent } from 'svelte';

  const StartLevitatingIcon = StartLevitating as unknown as typeof SvelteComponent;
  const StopLevitatingIcon = StopLevitating as unknown as typeof SvelteComponent;
</script>

<div class="p-4">
    <h2 class="text-2xl font-semibold mb-4">Levitation</h2>

    <TileGrid columns="1fr 1fr" rows="">
        <div class="flex gap-x-4 col-span-full mb-2">
            <Command cmd="LevitationOn" icon={StartLevitatingIcon}/>
            <Command cmd="LevitationOff" icon={StopLevitatingIcon}/>
            <Command cmd="vertical_0_current" text="Vertical 0 Current"/>
            <Command cmd="vert_0_current_reset" text="Reset Vertical 0 Current"/>
        </div>
        <Tile containerClass="col-span-1">
            <Chart title="HEMS Current" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="EMS Current" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Air Gaps" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Angles" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Requested Force 1" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Requested Force 2" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Temperatures HEMS" background="bg-surface-900"/>
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Temperatures EMS" background="bg-surface-900"/>
        </Tile>
    </TileGrid>
    <br>
</div>


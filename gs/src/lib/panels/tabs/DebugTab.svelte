<script lang="ts">
    import { Chart, Command, GrandDataDistributor, Tile, TileGrid } from '$lib';
    import { NamedCommandValues } from "$lib/types";
    import { propControlWord1, propControlWord2, propulsionConfigSent } from '$lib/stores/state';
    import CollapsibleTile from '$lib/components/generic/CollapsibleTile.svelte';
    import Command64Bits from '$lib/components/abstract/Command64Bits.svelte';
    import BinaryInput1 from '$lib/components/BinaryInput1.svelte';
    import BinaryInput2 from '$lib/components/BinaryInput2.svelte';

    const values: number[] = new Array(NamedCommandValues.length).fill(0);
    const propLabels: string[] = [
        // "Motor On",
        "Reset",
        "Phase Enable 1",
        "Phase Enable 2",
        "Phase Enable 3",
        "Logic Enable 1",
        "Logic Enable 2",
        "Logic Enable 3",
        // "Direction",
        // "General Fault"
    ].reverse();

    const propReadLabels: string[] = [
        "Direction",
        "General Fault",
        "State",
    ].reverse().concat(propLabels);

    const storeManager = GrandDataDistributor.getInstance().stores;
    const ppControlWordStore1 = storeManager.getWritable("Word1");
    const ppControlWordStore2 = storeManager.getWritable("Word2");

    // Propulsion debug stuff
    let modulation_factor: number = 0;
    let maximum_velocity: number = 0;
    let ppControlParams: number = 0;
    let direction: number = 0;

    let kpq: number = 0;
    let kiq: number = 0;
    let kpd: number = 0;
    let kid: number = 0;
    let ppDebugParams11: number = 0;
    let ppDebugParams12: number = 0;

    let pos_offset: number = 0;
    let alpha: number = 0;
    let ppDebugParams2: number = 0;

    let iq_ref: number = 0;
    let id_ref: number = 0;
    let vq_ref: number = 0;
    let vd_ref: number = 0;
    let testControlParams1: number = 0;
    let testControlParams2: number = 0;

    let calculatePPControlParams = () => {
        ppControlParams = (modulation_factor * 1000 << 16) | maximum_velocity * 10;
    }

    let calculatePPDebugParams1 = () => {
        ppDebugParams11 = (kpq << 16) | kiq;
        ppDebugParams12 = (kpd << 16) | kid;
    }

    let calculatePPDebugParams2 = () => {
        ppDebugParams2 = (pos_offset * 1000 << 16) | alpha * 1000;
    }

    let calculateTestControlParams = () => {
        testControlParams1 = (iq_ref * 10 << 16) | id_ref * 10;
        testControlParams2 = (vq_ref * 10 << 16) | vd_ref * 10;
    }

</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4">Debug Commands</h2>
    <TileGrid className="mb-5" columns="1fr 1fr" rows="">
        <CollapsibleTile title="Propulsion Commands">
            <div slot="content">
                <div class="border-surface-600 border-[1px] rounded-lg m-4 p-2">
                    <div class="grid grid-cols-9 gap-2 items-center mt-2 mb-3 ">
                        <span class="text-center">Propulsion Control Word 1:</span>
                        {#each Array.from({ length: propLabels.length }, (_, i) => propLabels.length - 1 - i) as i}
                            <BinaryInput1 index={i} />
                        {/each}
                        <Command cmd="SendPropulsionControlWord1" val={$propControlWord1} text={"Send"}/>

                        <span/>
                        {#each propLabels as l}
                            <span class="text-center">{l}</span>
                        {/each}
                        <span/>
                    </div>
                    <div class="grid grid-cols-11 gap-2 items-center mt-2 mb-3 ">
                        <span class="text-center">Received Control Word 1:</span>
                        <span class="text-center">{($ppControlWordStore1.value >> 2 & 1) + ($ppControlWordStore1.value >> 3 & 1) * 2}</span>
                        <span class="text-center">{$ppControlWordStore1.value >> 1 & 1}</span>
                        <span class="text-center">{$ppControlWordStore1.value & 1}</span>

                        {#each Array.from({ length: 7 }, (_, i) => propReadLabels.length - i) as i}
                            <span class="text-center">{$ppControlWordStore1.value >> (i + 4) & 1}</span>
                        {/each}

                        <span/>
                        {#each propReadLabels as l}
                            <span class="text-center">{l}</span>
                        {/each}
                        <span/>
                    </div>
                </div>
                <div class="border-surface-600 border-[1px] rounded-lg m-4 p-2">
                    <div class="grid grid-cols-9 gap-2 items-center mt-2 mb-3 ">
                        <span class="text-center">Propulsion Control Word 2:</span>
                        {#each Array.from({ length: propLabels.length }, (_, i) => propLabels.length - 1 - i) as i}
                            <BinaryInput2 index={i} />
                        {/each}
                        <Command cmd="SendPropulsionControlWord2" val={$propControlWord2} text={"Send"}/>

                        <span/>
                        {#each propLabels as l}
                            <span class="text-center">{l}</span>
                        {/each}
                        <span/>
                    </div>
                    <div class="grid grid-cols-11 gap-2 items-center mt-2 mb-3 ">
                        <span class="text-center">Received Control Word 2:</span>
                        <span class="text-center">{($ppControlWordStore2.value >> 2 & 1) + ($ppControlWordStore2.value >> 3 & 1) * 2}</span>
                        <span class="text-center">{$ppControlWordStore2.value >> 1 & 1}</span>
                        <span class="text-center">{$ppControlWordStore2.value & 1}</span>
                        {#each Array.from({ length: 7 }, (_, i) => propReadLabels.length - i) as i}
                            <span class="text-center">{$ppControlWordStore2.value >> (i + 4) & 1}</span>
                        {/each}
                        <span/>
                        {#each propReadLabels as l}
                            <span class="text-center">{l}</span>
                        {/each}
                        <span/>
                    </div>
                </div>
                <div class="grid grid-cols-2 gap-4 m-4 items-center ">
                    <div class="border-surface-600 border-[1px] flex flex-row gap-4 items-center p-4 rounded-lg ">
                        <Command64Bits
                            cmd="PPControlParams"
                            text="Submit PP Control Params"
                            values={[ppControlParams, direction << 16]}
                            onClickMethod={() => {propulsionConfigSent.set(true)}}
                        />
                        <div class="grid grid-cols-2 gap-2 ">
                            <div class="text-center content-center">Modulation Factor</div>
                            <input bind:value={modulation_factor} type="number" max={1} min={0} class="input p-4 rounded-md" on:change={calculatePPControlParams}>
                            <div class="text-center content-center">Maximum Velocity</div>
                            <input bind:value={maximum_velocity} type="number" class="input p-4 rounded-md " on:change={calculatePPControlParams}>
                            <div class="text-center content-center">Direction</div>
                            <input bind:value={direction} type="number" min="0" max="1" class="input p-4 rounded-md ">
                        </div>
                    </div>
                    <div class="border-surface-600 border-[1px] flex flex-row gap-4 items-center p-4 rounded-lg h-full">
                        <Command64Bits cmd="PPDebugParams2" text="Submit PP Debug Params 2" values={[0, ppDebugParams2]}/>
                        <div class="grid grid-cols-2 gap-2">
                            <div class="text-center content-center">Position Offset</div>
                            <input bind:value={pos_offset} type="number" class="input p-4 rounded-md " on:change={calculatePPDebugParams2}>
                            <div class="text-center content-center">Alpha</div>
                            <input bind:value={alpha} type="number" class="input p-4 rounded-md " on:change={calculatePPDebugParams2}>
                        </div>
                    </div>
                    <div class="border-surface-600 border-[1px] flex flex-row gap-4 items-center p-4 rounded-lg">
                        <Command64Bits cmd="PPDebugParams1" text="Submit PP Debug Params 1" values={[ppDebugParams11, ppDebugParams12]} />
                        <div class="grid grid-cols-2 gap-2">
                            <div class="text-center content-center">kpq</div>
                            <input bind:value={kpq} type="number" class="input p-4 rounded-md " on:change={calculatePPDebugParams1}>
                            <div class="text-center content-center">kiq</div>
                            <input bind:value={kiq} type="number" class="input p-4 rounded-md " on:change={calculatePPDebugParams1}>
                            <div class="text-center content-center">kpd</div>
                            <input bind:value={kpd} type="number" class="input p-4 rounded-md " on:change={calculatePPDebugParams1}>
                            <div class="text-center content-center">kid</div>
                            <input bind:value={kid} type="number" class="input p-4 rounded-md " on:change={calculatePPDebugParams1}>
                        </div>
                    </div>
                    <div class="border-surface-600 border-[1px] flex flex-row gap-4 items-center p-4 rounded-lg">
                        <Command64Bits cmd="PPTestControlParams" values={[testControlParams1, testControlParams2]} text="Submit PP Test Control Params" />
                        <div class="gap-2 grid grid-cols-2">
                            <div class="text-center content-center">iq_ref</div>
                            <input bind:value={iq_ref} type="number" class="input p-4 rounded-md " on:change={calculateTestControlParams}>
                            <div class="text-center content-center">id_ref</div>
                            <input bind:value={id_ref} type="number" class="input p-4 rounded-md " on:change={calculateTestControlParams}>
                            <div class="text-center content-center">vq_ref</div>
                            <input bind:value={vq_ref} type="number" class="input p-4 rounded-md " on:change={calculateTestControlParams}>
                            <div class="text-center content-center">vd_ref</div>
                            <input bind:value={vd_ref} type="number" class="input p-4 rounded-md " on:change={calculateTestControlParams}>
                        </div>
                    </div>
                </div>
                <TileGrid columns="1fr 1fr" rows="auto" className="mt-2">
                    <Tile containerClass="col-span-1" heading="Propulsion Log 1 - MD1">
                        <Chart title="Propulsion Log 1 - MD1" background="bg-surface-900" />
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 1 - MD2">
                        <Chart title="Propulsion Log 1 - MD2" background="bg-surface-900" />
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 2 - MD1">
                        <Chart title="Propulsion Log 2 - MD1" background="bg-surface-900" />
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 2 - MD2">
                        <Chart title="Propulsion Log 2 - MD2" background="bg-surface-900" />
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 3 - MD1">
                        <Chart title="Propulsion Log 3 - MD1" background="bg-surface-900" />
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 3 - MD2">
                        <Chart title="Propulsion Log 3 - MD2" background="bg-surface-900" />
                    </Tile>
                </TileGrid>
            </div>
        </CollapsibleTile>
        <CollapsibleTile title="Levi Commands">
            <div slot="content">
                <h1>To be added...</h1>
            </div>
        </CollapsibleTile>
        <CollapsibleTile title="General Commands">
            <div slot="content">
                <div class="grid grid-cols-4 w-full gap-4">
                    {#each NamedCommandValues as cmd, i}
                        <Command {cmd} val={Number(values[i])}/>
                        <input bind:value={values[i]} type="number" class="input p-4 rounded-md">
                    {/each}
                </div>
            </div>
        </CollapsibleTile>
    </TileGrid>
</div>

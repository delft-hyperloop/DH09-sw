<script lang="ts">
    import { Chart, Command, GrandDataDistributor, Tile, TileGrid } from '$lib';
    import { NamedCommandValues } from "$lib/types";
    import { overrideDependencies, propControlWord1, propControlWord2, propulsionConfigSent, showcasingStates, RedHVALTurnedOn, GreenHVALTurnedOn } from '$lib/stores/state';
    import CollapsibleTile from '$lib/components/generic/CollapsibleTile.svelte';
    import Command64Bits from '$lib/components/abstract/Command64Bits.svelte';
    import BinaryInput1 from '$lib/components/BinaryInput1.svelte';
    import BinaryInput2 from '$lib/components/BinaryInput2.svelte';
    import PropulsionHeartbeat from '$lib/components/PropulsionHeartbeat.svelte';
    import PropulsionInitFault from '$lib/components/PropulsionInitFault.svelte';

    const values: number[] = new Array(NamedCommandValues.length).fill(0);
    const propLabels: string[] = [
        "Reset",
        "Phase Enable 1",
        "Phase Enable 2",
        "Phase Enable 3",
        "Logic Enable 1",
        "Logic Enable 2",
        "Logic Enable 3",
    ].reverse();

    const propReadLabels: string[] = [
        "Direction",
        "General Fault",
        "State",
    ].reverse().concat(propLabels);

    const storeManager = GrandDataDistributor.getInstance().stores;
    const ppEmergency1 = storeManager.getWritable("PPEmergency1");
    const ppEmergency2 = storeManager.getWritable("PPEmergency2");
    // const propInitFault1 = storeManager.getWritable("PPInitFault1");
    // const propInitFault2 = storeManager.getWritable("PPInitFault2");

    // Propulsion debug stuff
    let modulation_factor: number = 0;
    let maximum_velocity: number = 0;
    let direction: number = 0;
    let maximumPower: number = 0;
    let ppControlParams1: number = 0;
    let ppControlParams2: number = 0;

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
        ppControlParams1 = (((modulation_factor * 1000) & 0xFFFF) << 16) | ((maximum_velocity * 10) & 0xFFFF);
        ppControlParams2 = ((direction & 0xFFFF) << 16) | (maximumPower & 0xFFFF);
    }

    let calculatePPDebugParams1 = () => {
        ppDebugParams11 = ((kpq & 0xFFFF) << 16) | (kiq & 0xFFFF);
        ppDebugParams12 = ((kpd & 0xFFFF) << 16) | (kid & 0xFFFF);
    }

    let calculatePPDebugParams2 = () => {
        ppDebugParams2 = (((pos_offset * 1000) & 0xFFFF) << 16) | ((alpha * 1000) & 0xFFFF);
    }

    let calculateTestControlParams = () => {
        testControlParams1 = (((iq_ref * 10) & 0xFFFF) << 16) | ((id_ref * 10) & 0xFFFF);
        testControlParams2 = (((vq_ref * 10) & 0xFFFF) << 16) | ((vd_ref * 10) & 0xFFFF);
    }

</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4">Debug Commands</h2>
    <TileGrid className="mb-5" columns="1fr 1fr" rows="">
        <CollapsibleTile title="Propulsion Commands">
            <div slot="content">
                <div class="border-surface-600 border-[1px] rounded-lg m-4 p-2">
                    <div class="m-4">
                        <PropulsionHeartbeat storeName="Word1" labels={propReadLabels}/>
                    </div>
                    <div class="grid grid-cols-9 gap-2 items-center m-4">
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
                    <div class="m-4 mt-8">
                        <PropulsionInitFault storeName="PPInitFault1"/>
                    </div>
                    <div class="m-4 text-center">
                        <span>Emergency 1: {$ppEmergency1.value}</span>
                    </div>
<!--                    <div class="m-4">-->
<!--                        <span>Init fault 1: {$propInitFault1.value}</span>-->
<!--                    </div>-->
                </div>
                <div class="border-surface-600 border-[1px] rounded-lg m-4 p-2">
                    <div class="m-4 mt-8">
                        <PropulsionHeartbeat storeName="Word2" labels={propReadLabels}/>
                    </div>
                    <div class="grid grid-cols-9 gap-2 items-center m-4">
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
                    <div class="m-4 mt-8">
                        <PropulsionInitFault storeName="PPInitFault2"/>
                    </div>
                    <div class="m-4 text-center">
                        <span>Emergency 2: {$ppEmergency2.value}</span>
                    </div>
<!--                    <div class="m-4 mt-8">-->
<!--                        <span>Init fault 2: {$propInitFault2.value}</span>-->
<!--                    </div>-->
                </div>
                <div class="grid grid-cols-2 gap-4 m-4 items-center">
                    <div class="border-surface-600 border-[1px] flex flex-row gap-4 items-center p-4 rounded-lg ">
                        <Command64Bits
                            cmd="PPControlParams"
                            text="Submit PP Control Params"
                            values={[ppControlParams1, ppControlParams2]}
                            onClickMethod={() => {propulsionConfigSent.set(true)}}
                        />
                        <div class="grid grid-cols-2 gap-2 ">
                            <div class="text-center content-center">Modulation Factor</div>
                            <input bind:value={modulation_factor} type="number" max={1} min={0} class="input p-4 rounded-md" on:change={calculatePPControlParams}>
                            <div class="text-center content-center">Maximum Velocity</div>
                            <input bind:value={maximum_velocity} type="number" class="input p-4 rounded-md " on:change={calculatePPControlParams}>
                            <div class="text-center content-center">Direction</div>
                            <input bind:value={direction} type="number" min="0" max="1" class="input p-4 rounded-md " on:change={calculatePPControlParams}>
                            <div class="text-center content-center">Maximum power</div>
                            <input bind:value={maximumPower} type="number" min="0" max="50000" class="input p-4 rounded-md " on:change={calculatePPControlParams}>
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
                    <Tile containerClass="col-span-1" heading="Motor Temperatures Left">
                        <Chart title="Motor Temperatures Left" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Motor Temperatures Right">
                        <Chart title="Motor Temperatures Right" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 1 - Left Motor">
                        <Chart title="Propulsion Log 1 - Left Motor" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 1 - Right Motor">
                        <Chart title="Propulsion Log 1 - Right Motor" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 2 - Left Motor">
                        <Chart title="Propulsion Log 2 - Left Motor" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 2 - Right Motor">
                        <Chart title="Propulsion Log 2 - Right Motor" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 3 - Left Motor">
                        <Chart title="Propulsion Log 3 - Left Motor" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-1" heading="Propulsion Log 3 - Right Motor">
                        <Chart title="Propulsion Log 3 - Right Motor" display_title={false} pop_up={false}/>
                    </Tile>
                    <Tile containerClass="col-span-2" heading="Offset">
                        <Chart title="Offset" display_title={false} pop_up={false}/>
                    </Tile>
                </TileGrid>
            </div>
        </CollapsibleTile>
        <CollapsibleTile title="Reset Commands">
            <div slot="content" class="flex gap-4">
                <Command cmd="SystemReset"/>
                <Command cmd="ResetSenseCon"/>
                <Command cmd="ResetPowertrain"/>
                <Command cmd="ResetLevitation"/>
                <Command cmd="ResetPropulsion"/>
                <Command cmd="ResetLocalization"/>
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
        <CollapsibleTile title="Override commands">
            <div slot="content" class="flex gap-4">
                {#if $overrideDependencies}
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium"
                            on:click={() => {overrideDependencies.set(false)}}>
                        Enable Command Guards
                    </button>
                {:else}
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium"
                            on:click={() => {overrideDependencies.set(true)}}>
                        Disable Command Guards
                    </button>
                {/if}
                <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium"
                        on:click={() => {GreenHVALTurnedOn.set(!$GreenHVALTurnedOn)}}>
                    Toggle Green HVAL
                </button>
                <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium"
                        on:click={() => {RedHVALTurnedOn.set(!$RedHVALTurnedOn)}}>
                    Toggle Red HVAL
                </button>
                {#if !$showcasingStates}
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium"
                            on:click={() => {showcasingStates.set(true)}}>
                        Showcase FSM States
                    </button>
                {:else}
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium"
                            on:click={() => {showcasingStates.set(false)}}>
                        Stop Showcasing States
                    </button>
                {/if}
            </div>
        </CollapsibleTile>
    </TileGrid>
</div>

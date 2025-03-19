<script lang="ts">
    import { Command, GrandDataDistributor, Tile, TileGrid } from '$lib';
    import {NamedCommandValues} from "$lib/types";
    import BinaryInput from '$lib/components/BinaryInput.svelte';
    import { propControlWord } from '$lib/stores/state';

    const values:number[] = new Array(NamedCommandValues.length).fill(0);

    const storeManager = GrandDataDistributor.getInstance().stores;
    const direction = storeManager.getWritable("GoingForward");

    // Propulsion debug stuff
    // 2
    let modulation_factor: number = 0;
    let maximum_velocity: number = 0

    // 3
    let kpq: number = 0;
    let kiq: number = 0;
    let kpd: number = 0;
    let kid: number = 0;

    // 4
    let pos_offset: number = 0;
    let alpha: number = 0;

    // 5
    let iq_ref: number = 0;
    let id_ref: number = 0;
    let vq_ref: number = 0;
    let vd_ref: number = 0;

    let submitPPControlParams = () => {
        let data = (modulation_factor << 16) | maximum_velocity;
        values[2] = data;
    }

    let submitPPDebugParams1 = () => {
        let data1 = kiq | (kpq << 16);
        let data2 = kid | (kpd << 16);

    }

    let submitPPDebugParams2 = () => {
        let data = (pos_offset << 16) | alpha;
        values[4] = data;
        console.log(data);
    }

    let submitTestControlParams = () => {
        let data1 = (iq_ref << 16) | id_ref;
        let data2 = (vq_ref << 16) | vd_ref;

    }

    export const pop_up: boolean = true;
</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4">Debug Commands</h2>

    <TileGrid columns="1fr 1fr" rows="">
        <Tile containerClass="col-span-full">
            <div class="grid grid-cols-12 gap-2 items-center mb-2">
                <span class="text-center">Propulsion Control Word:</span>
                <BinaryInput index={9} />
                <BinaryInput index={8} />
                <BinaryInput index={7} />
                <BinaryInput index={6} />
                <BinaryInput index={5} />
                <BinaryInput index={4} />
                <BinaryInput index={3} />
                <BinaryInput index={2} />
                <BinaryInput index={1} />
                <BinaryInput index={0} />
                <!--        <BinaryInput associatedStore={direction} />-->
                <Command cmd="SendPropulsionControlWord" val={$propControlWord} text={"Send"}/>

                <span/>
                <span class="text-center">Motor On</span>
                <span class="text-center">Phase Enable 1</span>
                <span class="text-center">Phase Enable 2</span>
                <span class="text-center">Phase Enable 3</span>
                <span class="text-center">Logic Enable 1</span>
                <span class="text-center">Logic Enable 2</span>
                <span class="text-center">Logic Enable 3</span>
                <span class="text-center">Reset</span>
                <span class="text-center">Direction</span>
                <span class="text-center">General Fault</span>
                <span/>
            </div>
            <div class="flex flex-col">
                <div class="flex flex-row gap-4 m-4 items-center">
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium text-wrap"
                            on:click={submitPPControlParams}>
                        Submit PP
                        Control Params
                    </button>
                    <div class="grid grid-cols-2 gap-2 ">
                        <div class="text-center content-center">Modulation Factor</div>
                        <input bind:value={modulation_factor} type="number" class="input p-4">
                        <div class="text-center content-center">Maximum Velocity</div>
                        <input bind:value={maximum_velocity} type="number" class="input p-4">
                    </div>
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium text-wrap "
                            on:click={submitPPDebugParams2}>
                        Submit PP Debug Params 2
                    </button>
                    <div class="grid grid-cols-2 gap-2">
                        <div class="text-center content-center">Position Offset</div>
                        <input bind:value={pos_offset} type="number" class="input p-4">
                        <div class="text-center content-center">Position Alpha</div>
                        <input bind:value={alpha} type="number" class="input p-4">
                    </div>
                </div>
                <div class="flex flex-row gap-4 m-4 items-center">
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium text-wrap"
                            on:click={submitPPDebugParams1}>
                        Submit PP
                        Debug Params 1
                    </button>
                    <div class="grid grid-cols-2 gap-2">
                        <div class="text-center content-center">kpq</div>
                        <input bind:value={kpq} type="number" class="input p-4">
                        <div class="text-center content-center">kiq</div>
                        <input bind:value={kiq} type="number" class="input p-4">
                        <div class="text-center content-center">kpd</div>
                        <input bind:value={kpd} type="number" class="input p-4">
                        <div class="text-center content-center">kid</div>
                        <input bind:value={kid} type="number" class="input p-4">
                    </div>
                    <button class="btn rounded-md bg-primary-500 text-surface-900 overflow-auto font-medium text-wrap "
                            on:click={submitTestControlParams}>
                        Submit PP Test Control Params
                    </button>
                    <div class="gap-2 grid grid-cols-2">
                        <div class="text-center content-center">iq_ref</div>
                        <input bind:value={iq_ref} type="number" class="input p-4">
                        <div class="text-center content-center">id_ref</div>
                        <input bind:value={id_ref} type="number" class="input p-4">
                        <div class="text-center content-center">vq_ref</div>
                        <input bind:value={vq_ref} type="number" class="input p-4">
                        <div class="text-center content-center">vd_ref</div>
                        <input bind:value={vd_ref} type="number" class="input p-4">
                    </div>
                </div>
            </div>
        </Tile>
        <Tile containerClass="col-span-2">
            <div class="grid grid-cols-4 w-full gap-4 pb-20 ">
                {#each NamedCommandValues as cmd, i}
                    <Command {cmd} val={Number(values[i])}/>
                    <input bind:value={values[i]} type="number" class="input p-4">
                {/each}
            </div>
        </Tile>
    </TileGrid>
</div>

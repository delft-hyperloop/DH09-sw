<script lang="ts">
    import { Command, GrandDataDistributor } from '$lib';
    import {NamedCommandValues} from "$lib/types";
    import BinaryInput from '$lib/components/BinaryInput.svelte';
    import { propControlWord } from '$lib/stores/state';

    const values:number[] = new Array(NamedCommandValues.length).fill(0);

    const storeManager = GrandDataDistributor.getInstance().stores;
    const direction = storeManager.getWritable("GoingForward");

    export const pop_up: boolean = true;
</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4">Debug Commands</h2>

    <div class="grid grid-cols-12 gap-2 mb-4 items-center">
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

    <div class="grid grid-cols-4 w-full gap-4 pb-20">
        {#each NamedCommandValues as cmd, i}
            <Command {cmd} val={Number(values[i])}/>
            <input bind:value={values[i]} type="number" class="input p-4">
        {/each}
    </div>
</div>

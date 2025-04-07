<script lang="ts">
    import {
        Table,
        Status,
        Command,
        Tile,
        TileGrid,
        GrandDataDistributor,
        Chart,
    } from '$lib';
    import {DatatypeEnum} from "$lib/namedDatatypeEnum";
    import {invoke} from "@tauri-apps/api/tauri";
    import {STATUS} from "$lib/types";
    import { podSpeed, propModulationFactor } from '$lib/stores/data';
    import { debugModeActive, goingForwardState } from '$lib/stores/state';
    import RangeSlider from 'svelte-range-slider-pips';

    const storeManager = GrandDataDistributor.getInstance().stores;
    // const statuses = storeManager.getWritable("ConnectionStatus")

    export let pop_up: boolean = true;

    let tableArr2:any[][];
    // $: tableArr2 = [
    //     ["Acceleration X", DatatypeEnum.ACCELERATIONX],
    //     ["Acceleration Y", DatatypeEnum.ACCELERATIONY],
    //     ["Acceleration Z", DatatypeEnum.ACCELERATIONZ],
    //     ["Gyroscope X", DatatypeEnum.GYROSCOPEX],
    //     ["Gyroscope Y", DatatypeEnum.GYROSCOPEY],
    //     ["Gyroscope Z", DatatypeEnum.GYROSCOPEZ],
    // ]

    let currentDirectionForward: boolean = $goingForwardState;
    let currentSpeed: number = $podSpeed;
    // Value bound to the modulation factor slider.
    // Didn't work with a different name for some reason.
    let values = [100];

    // const modalStore = getModalStore();

    // const input:ModalComponent = {ref: SpeedsInput};
    // let inputModal = () => {
    //     modalStore.trigger({
    //         type: "component",
    //         component: input,
    //         title: "Run Configuration",
    //     })
    // }
    // let finishRunConfig = () => {
    //     invoke('send_command', {cmdName: "FinishRunConfig", val: 0}).then(() => {
    //         console.log(`Command FinishRunConfig sent`);
    //         modalStore.close();
    //     });
    // }

    function submitRun() {
        goingForwardState.set(currentDirectionForward);
        podSpeed.set(currentSpeed);
        propModulationFactor.set(values[0]);

        invoke('send_command', {cmdName: "SubmitDirection", val: currentDirectionForward}).then(() => {
            console.log(`Direction goingForward = ${currentDirectionForward} sent to the pod`);
        });
        invoke('send_command', {cmdName: "SubmitSpeed", val: currentSpeed}).then(() => {
            console.log(`Speed of ${currentSpeed} sent to the pod`);
        });
        // TODO: send config command
    }

</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4 ">Initialization</h2>

    <TileGrid columns="1fr 1fr 1.5fr" rows="auto 1fr">
        <Tile containerClass="row-span-2" insideClass="flex flex-col gap-2" heading="Run Initialization">
            <div class="grid grid-cols-2 gap-2">
                <Command cmd="EnablePropulsion" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto"/>
                <Command cmd="DisablePropulsion" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <Command cmd="SystemReset" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <Command cmd="ArmBrakes" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <p class="col-span-2">
                    Choose Direction:
                </p>
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-1 flex-grow overflow-auto font-medium"
                        on:click={() => {currentDirectionForward = true}}
                        disabled={currentDirectionForward}>
                    Forward
                </button>
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-1 flex-grow overflow-auto font-medium"
                        on:click={() => {currentDirectionForward = false}}
                        disabled={!currentDirectionForward}>
                    Backward
                </button>
                <p class="col-span-full">
                    Change Speed:
                </p>
                <input class="input rounded-lg px-1 col-span-2 min-h-10"
                       type="number"
                       max="500"
                       min="0"
                       bind:value={currentSpeed}
                />
                <p class="col-span-full">
                    Modulation factor:
                </p>
                <input class="input rounded-lg px-1 col-span-2 min-h-10"
                       type="number"
                       max="100"
                       min="0"
                       bind:value={values[0]}
                />
                <div class="col-span-full mb-4">
                    <RangeSlider value={100} bind:values min={0} max={100} pips all="label" suffix="%" pipstep={25}

                    />
                </div>
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-full flex-grow overflow-auto font-medium"
                        on:click={submitRun} disabled={false}>
                    Submit New Run Parameters
                </button>
                <hr class="col-span-full">
                <p class="col-span-full font-normal text-xl justify-center text-center pb-3 ">Current Values:</p>
                <p>Desired Speed:</p>
                <p>{$podSpeed} m/s</p>
                <p>Run Direction:</p>
                {#if $goingForwardState}
                    <p>Forward</p>
                {:else}
                    <p>Backward</p>
                {/if}
                <p>Modulation Factor:</p>
                <p>{$propModulationFactor}%</p>
            </div>
        </Tile>
        <Tile insideClass="grid grid-cols-2 gap-y-2 auto-rows-min" heading="Statuses" >
<!--            <p>Main PCB</p>-->
<!--            <Status status={$statuses.value[STATUS.MAIN_PCB]} />-->
<!--            <p>Propulsion</p>-->
<!--            <Status on="Active" off="Off" status={$statuses.value[STATUS.PROPULSION]} />-->
<!--            <p>Levitation</p>-->
<!--            <Status status={$statuses.value[STATUS.LEVITATION]} />-->
<!--            <p>Sensor Hub</p>-->
<!--            <Status status={$statuses.value[STATUS.SENSOR_HUB]} />-->
<!--            <p>LV Batteries</p>-->
<!--            <Status status={$statuses.value[STATUS.LV_BATTERIES]} />-->
<!--            <p>HV Batteries</p>-->
<!--            <Status status={$statuses.value[STATUS.HV_BATTERIES]} />-->
<!--            <p>Braking PCB</p>-->
<!--            <Status on="Armed" off="Extended" status={$statuses.value[STATUS.BRAKING_PCB]} />-->
<!--            <p>Voltage Over 50</p>-->
<!--            <Status offColor="text-primary-400" off="Safe"-->
<!--                    onColor="text-error-400" on="UNSAFE"-->
<!--                    status={$statuses.value[STATUS.VOLTAGE_OVER]} />-->
        </Tile>
        <Tile heading="Data">
            <Table tableArr={tableArr2} background="bg-surface-900" titles={["important", "variable"]}/>
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart height={250} background="bg-surface-900" title="Velocity" />
        </Tile>
    </TileGrid>

    {#if $debugModeActive}
        <Tile containerClass="col-span-4 mt-2" heading="Propulsion Log 1">
            <Chart title="Propulsion Log 1" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-4 mt-2" heading="Propulsion Log 2">
            <Chart title="Propulsion Log 2" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-4 mt-2" heading="Propulsion Log 3">
            <Chart title="Propulsion Log 3" background="bg-surface-900" />
        </Tile>
    {/if}
</div>

<style>
    :root {
        --range-slider:            hsl(165.6, 33.9%, 45.7%);
        --range-handle-inactive:   hsl(165.6, 33.9%, 45.7%);
        --range-handle:            hsl(165.6, 33.9%, 45.7%);
        --range-handle-focus:      hsl(165.6, 33.9%, 45.7%);
        --range-handle-border:     hsl(180, 5.2%, 33.9%);
        --range-range-inactive:    hsl(180, 5.2%, 33.9%);
        --range-range:             hsl(180, 5.2%, 33.9%);
        --range-range-limit:       hsl(180, 5.2%, 33.9%);
        --range-inactive:          hsl(180, 5.2%, 33.9%);

        --range-pip:               hsl(180, 5.2%, 33.9%);
        --range-pip-text:          hsl(180, 5.2%, 33.9%);
        --range-pip-active:        hsl(165.6, 33.9%, 45.7%);
        --range-pip-active-text:   hsl(165.6, 33.9%, 45.7%);
        --range-pip-hover:         hsl(165.6, 33.9%, 45.7%);
        --range-pip-hover-text:    hsl(165.6, 33.9%, 45.7%);
        --range-pip-in-range:      hsl(180, 5.2%, 33.9%);
        --range-pip-in-range-text: hsl(180, 5.2%, 33.9%);
        --range-pip-out-of-limit:      hsl(180, 5.2%, 33.9%);
        --range-pip-out-of-limit-text: hsl(180, 5.2%, 33.9%);
    }
</style>

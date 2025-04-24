<script lang="ts">
    import {
        Table,
        Command,
        Tile,
        TileGrid,
        Chart, util, EventChannel,
    } from '$lib';
    import { podSpeed, propMaxPower, propModulationFactor } from '$lib/stores/data';
    import { debugModeActive, goingForward, propulsionConfigSent } from '$lib/stores/state';
    import RangeSlider from 'svelte-range-slider-pips';
    import { invoke } from '@tauri-apps/api/tauri';

    // const storeManager = GrandDataDistributor.getInstance().stores;
    // const statuses = storeManager.getWritable("ConnectionStatus")

    // export let pop_up: boolean = true;

    let tableArr2:any[][];
    // $: tableArr2 = [
    //     ["Acceleration X", DatatypeEnum.ACCELERATIONX],
    //     ["Acceleration Y", DatatypeEnum.ACCELERATIONY],
    //     ["Acceleration Z", DatatypeEnum.ACCELERATIONZ],
    //     ["Gyroscope X", DatatypeEnum.GYROSCOPEX],
    //     ["Gyroscope Y", DatatypeEnum.GYROSCOPEY],
    //     ["Gyroscope Z", DatatypeEnum.GYROSCOPEZ],
    // ]

    let currentDirectionForward: boolean = $goingForward;
    let currentSpeed: number = $podSpeed;
    let currentMaxPower: number = $propMaxPower;

    // Value bound to the modulation factor slider.
    // Didn't work with a different name for some reason.
    let values = [1];

    async function submitRun() {
        goingForward.set(currentDirectionForward);
        podSpeed.set(currentSpeed);
        propModulationFactor.set(values[0]);
        propMaxPower.set(currentMaxPower); // TODO: Add here

        let direction: number = 1;
        if (!$goingForward) {
            direction = 0;
        }

        const value1 = ($propModulationFactor * 1000 << 16) | $podSpeed * 10;
        const value2 = (direction << 16) | $propMaxPower;

        await invoke('send_command_64_bits', {cmdName: "PPControlParams", vals: [value1, value2]}).then(() => {
            console.log(`Sending command: PPControlParams, value: ${value1} ${value2}`);
        }).catch((e) => {
            console.error(`Error sending command PPControlParams: ${e}`);
        });
        util.log(`Command PPControlParams sent`, EventChannel.INFO);

        propulsionConfigSent.set(true);
    }

</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4 ">Initialization</h2>

    <TileGrid columns="1fr 1fr 1.5fr" rows="auto 1fr">
        <Tile containerClass="row-span-2" insideClass="flex flex-col gap-2" heading="Run Initialization">
            <div class="grid grid-cols-2 gap-2">
                <Command
                    cmd="EnablePropulsion"
                    className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto"
                    dependency={propulsionConfigSent}
                    dependencyMessage="Run configuration was not sent! Can't start propulsion without specifying
                    a direction, top speed and modulation factor!"
                    dependencyTitle="Configuration not sent!"
                />
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
                    Target Speed:
                </p>
                <input class="input rounded-lg px-1 col-span-2 min-h-10"
                       type="number"
                       max="500"
                       min="0"
                       bind:value={currentSpeed}
                />
                <p class="col-span-full">
                    Maximum power:
                </p>
                <input class="input rounded-lg px-1 col-span-2 min-h-10"
                       type="number"
                       max="50000"
                       min="0"
                       bind:value={currentMaxPower}
                />
                <p class="col-span-full">
                    Modulation factor:
                </p>
                <input class="input rounded-lg px-1 col-span-2 min-h-10"
                       type="number"
                       max="1"
                       min="0"
                       bind:value={values[0]}
                       step="0.01"
                />
                <div class="col-span-full mb-4">
                    <RangeSlider
                        value={1}
                        bind:values
                        min={0}
                        max={1}
                        pips
                        all="label"
                        step={0.01}
                        pipstep={25}
                    />
                </div>
                <button class="btn text-wrap rounded-md bg-primary-500 text-surface-900 col-span-full flex-grow overflow-auto font-medium"
                        on:click={submitRun} disabled={false}>
                    Submit New Run Parameters
                </button>
                <hr class="col-span-full">
                <p class="col-span-full font-normal text-xl justify-center text-center pb-3 ">Current Values:</p>
                <p>Run Direction:</p>
                {#if $propulsionConfigSent}
                    {#if $goingForward}
                        <p>Forward</p>
                    {:else}
                        <p>Backward</p>
                    {/if}
                {:else}
                    <p>Not set</p>
                {/if}
                <p>Desired Speed:</p>
                {#if $propulsionConfigSent}
                    <p>{$podSpeed} m/s</p>
                {:else}
                    <p>Not set</p>
                {/if}
                <p>Maximum power:</p>
                {#if $propulsionConfigSent}
                    <p>{$propMaxPower} W</p>
                {:else}
                    <p>Not set</p>
                {/if}
                <p>Modulation Factor:</p>
                {#if $propulsionConfigSent}
                    <p>{$propModulationFactor}</p>
                {:else}
                    <p>Not set</p>
                {/if}
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

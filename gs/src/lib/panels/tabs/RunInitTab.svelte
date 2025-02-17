<script lang="ts">
    import {
        Table,
        Status,
        Command,
        Tile,
        TileGrid,
        GrandDataDistributor,
        Chart
    } from "$lib";
    import {DatatypeEnum} from "$lib/namedDatatypeEnum";
    import {invoke} from "@tauri-apps/api/tauri";
    import {STATUS} from "$lib/types";
    import { podSpeed } from '$lib/stores/data';
    import { goingForward } from '$lib/stores/state';

    const storeManager = GrandDataDistributor.getInstance().stores;
    const statuses = storeManager.getWritable("ConnectionStatus")

    export let pop_up: boolean = true;

    let tableArr2:any[][];
    $: tableArr2 = [
        ["Acceleration X", DatatypeEnum.ACCELERATIONX],
        ["Acceleration Y", DatatypeEnum.ACCELERATIONY],
        ["Acceleration Z", DatatypeEnum.ACCELERATIONZ],
        ["Gyroscope X", DatatypeEnum.GYROSCOPEX],
        ["Gyroscope Y", DatatypeEnum.GYROSCOPEY],
        ["Gyroscope Z", DatatypeEnum.GYROSCOPEZ],
    ]

    let currentDirectionForward: boolean = $goingForward;
    let currentSpeed: number = $podSpeed;

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
        goingForward.set(currentDirectionForward);
        podSpeed.set(currentSpeed);
        invoke('send_command', {cmdName: "SubmitRun", val: 0}).then(() => {
            console.log("Speed sent to pod");
        }) // TODO: see how command is sent
    }

</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4">Initialization</h2>

    <TileGrid columns="1fr 1fr 1.5fr" rows="auto 1fr">
        <Tile containerClass="row-span-2" insideClass="flex flex-col gap-2" heading="Run Initialisation">
            <div class="grid grid-cols-2 gap-2">
                <Command cmd="EnablePropulsion" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto"/>
                <Command cmd="DisablePropulsion" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <Command cmd="SystemReset" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <Command cmd="ArmBrakes" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <p class="col-span-2">Choose Direction:</p>
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
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-full flex-grow overflow-auto font-medium" on:click={submitRun} disabled={false}>
                    Submit Run
                </button>
                <hr class="col-span-full">
                <p class="col-span-full font-normal text-xl justify-center text-center pb-3 ">Current Values:</p>
                <p>Desired Speed:</p>
                <p>{$podSpeed} m/s</p>
                <p>Run Direction:</p>
                {#if $goingForward}
                    <p>Forward</p>
                {:else}
                    <p>Backward</p>
                {/if}
            </div>
        </Tile>
        <Tile insideClass="grid grid-cols-2 gap-y-2 auto-rows-min" heading="Statuses" >
            <p>Main PCB</p>
            <Status status={$statuses.value[STATUS.MAIN_PCB]} />
            <p>Propulsion</p>
            <Status on="Active" off="Off" status={$statuses.value[STATUS.PROPULSION]} />
            <p>Levitation</p>
            <Status status={$statuses.value[STATUS.LEVITATION]} />
            <p>Sensor Hub</p>
            <Status status={$statuses.value[STATUS.SENSOR_HUB]} />
            <p>LV Batteries</p>
            <Status status={$statuses.value[STATUS.LV_BATTERIES]} />
            <p>HV Batteries</p>
            <Status status={$statuses.value[STATUS.HV_BATTERIES]} />
            <p>Braking PCB</p>
            <Status on="Armed" off="Extended" status={$statuses.value[STATUS.BRAKING_PCB]} />
            <p>Voltage Over 50</p>
            <Status offColor="text-primary-400" off="Safe"
                    onColor="text-error-400" on="UNSAFE"
                    status={$statuses.value[STATUS.VOLTAGE_OVER]} />
        </Tile>
        <Tile heading="Data">
            <Table tableArr={tableArr2} background="bg-surface-900" titles={["important", "variable"]}/>
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart height={250} background="bg-surface-900" title="Velocity" />
        </Tile>
    </TileGrid>
</div>

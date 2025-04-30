<script lang="ts">
    import {Battery, TileGrid, Tile, Command, GrandDataDistributor} from "$lib";
    import { AppBar, getToastStore } from '@skeletonlabs/skeleton';
    import Icon from "@iconify/svelte";
    import {invoke} from "@tauri-apps/api/tauri";
    import {DatatypeEnum as DE} from "$lib/namedDatatypeEnum";
    import Localization from '$lib/components/Localization.svelte';
    import Light from '$lib/components/Light.svelte';
    import MainFSM from '$lib/components/MainFSM.svelte';
    import { showcaseStateCounter, showcasingStates } from '$lib/stores/state';

    let width: number;

    const storeManager = GrandDataDistributor.getInstance().stores;
    const lvBattery = storeManager.getWritable("BMSVoltageLow");
    const hvBattery = storeManager.getWritable("BMSVoltageHigh");
    const fsmState = storeManager.getWritable("FSMState");
    const location1 = storeManager.getWritable("Loc1");
    const location2 = storeManager.getWritable("Loc2");

    let tableTempsArr: any[][];
    let tableArr2: any[][];

    let tableBatteryTitles = ["", "HV Voltages", "HV Temp", "LV Voltages", "LV Temp"]

    $: tableBatteryVitals = [
        ["Min", DE.Alpha1, DE.Alpha1, DE.Alpha1, DE.Alpha1],
        ["Max", DE.Alpha1, DE.Alpha1, DE.Alpha1, DE.Alpha1],
        ["Avg", DE.Alpha1, DE.Alpha1, DE.Alpha1, DE.Alpha1],
        ["Safe Range", "[360, 420] V", "[15,50] °C", "[280,360] V", "[15,50] °C"]
    ]

    $: tableTempsArr = [
        ["Up VB", DE.Alpha1, "[0,70] °C", "HEMS 1", DE.Alpha1, "[0,80] °C"],
        ["Low VB", DE.Alpha1, "[0,70] °C", "HEMS 2", DE.Alpha1, "[0,80] °C"],
        ["Ambient", DE.Temp0, "[0,50] °C", "HEMS 3", DE.Alpha1, "[0,80] °C"],
        ["Motor Front", "Temp_Motor_1", "[0,80] °C", "HEMS 4", DE.Alpha1, "[0,80] °C"],
        ["Motor Back", "Temp_Motor_2", "[0,80] °C", "EMS 1", DE.Alpha1, "[0,80] °C"],
        ["", "", "", "EMS 2", DE.Alpha1, "[0,80] °C"],
    ]

    $: tableArr2 = [
        ["HEMS A1", DE.Alpha1, "[-10,10] A", "HEMS A2", DE.Alpha1, "[-10,10] A"],
        ["HEMS B1", DE.Alpha1, "[-10,10] A", "HEMS B2", DE.Alpha1, "[-10,10] A"],
        ["HEMS C1", DE.Alpha1, "[-10,10] A", "HEMS C2", DE.Alpha1, "[-10,10] A"],
        ["HEMS D1", DE.Alpha1, "[-10,10] A", "HEMS D2", DE.Alpha1, "[-10,10] A"],
        ["EMS AB", DE.Alpha1, "[-10,10] A", "EMS CD", DE.Alpha1, "[-10,10] A"],
    ]

    const toastStore = getToastStore();
</script>

<div bind:clientWidth={width} class="h-full bg-surface-700 text-surface-50">
    <AppBar padding="pl-8 pr-8 pt-3 pb-3" border="border-b border-b-surface-900" background="bg-surface-700" slotDefault="place-self-center">
        <svelte:fragment slot="lead">
            <div class="gap-2 flex flex-row items-center">
                <Icon icon="codicon:graph-line"/>
                <span>Vitals</span>
            </div>
        </svelte:fragment>

        <div class="flex gap-5 items-center justify-center">
            <span>IMD: &ltstatus&gt</span>
            <div class="flex flex-row gap-2 items-center">
                <span>HVAL:</span>
                <Light isGreen={true}/>
                <Light isGreen={false}/>
            </div>
        </div>

        <svelte:fragment slot="trail">
            <Command callback={() => {
                toastStore.trigger({
                    //@ts-ignore
                    message: "Emergency Brake triggered!",
                    background: 'variant-filled-error'
                });
            }} className="bg-error-500 text-surface-100 btn py-0 border border-error-500 rounded-sm" cmd="EmergencyBrake"/>
        </svelte:fragment>
    </AppBar>

    {#if width < 200}
        <div class="flex flex-col h-full pb-20 justify-between items-center">
            <button on:click={() => {
                invoke('send_command', {cmdName: "EmergencyBrake", val: 0}).then(() => {
                    console.log(`Triggered EmergencyBrake!!`);
                })
            }} class="btn border border-error-500 bg-error-500 rounded-sm">
                <span style="writing-mode: vertical-lr">EMERGENCY BRAKE</span>
            </button>
            <span style="writing-mode: vertical-lr" class="font-medium">Vitals Panel</span>
            <div class="flex flex-col gap-4">
                <Battery fill="#3b669c" orientation="vertical" height={55} perc={Number($lvBattery.value)}/>
                <Battery fill="#723f9c" orientation="vertical" height={55} perc={Number($hvBattery.value)}/>
            </div>
        </div>
    {:else}
        <div class="snap-x scroll-px-0.5 snap-mandatory overflow-x-auto h-[90vh]">
            <TileGrid className="p-4 w-full" columns="1fr 1fr" rows="">
                <Tile bgToken={800} containerClass="col-span-2">
                    <Localization showLabels={true}/>
                </Tile>
                <Tile bgToken={700} containerClass="col-span-2">
                    <div class="flex flex-wrap justify-between gap-4">
                        <div class="flex gap-4">
                            <p>
<!--                                Velocity: <Store datatype="Velocity" /> m/s-->
                                <br>
                                Position: {($location1.value + $location2.value) / 2} mm
                                <br>
<!--                                Acceleration: <Store datatype="Acceleration" /> m/s²-->
                            </p>
<!--                            <p>-->
<!--                                HV Current: <Store datatype="BatteryCurrentHigh" /> - [0, 25] A-->
<!--                                <br>-->
<!--                                LV Current: <Store datatype="BatteryCurrentLow" /> - [0, 10] A-->
<!--                            </p>-->
<!--                            <p>-->
<!--                                Low Pressure: <Store datatype="LowPressureSensor" /> - [40, 52] bar-->
<!--                                <br>-->
<!--                                High Pressure: <Store datatype="HighPressureSensor" /> - [80, 180] bar-->
<!--                            </p>-->
                        </div>
                        <div style="grid-template-columns: 1fr 2fr 3fr;" class="grid gap-2">
                            <span>LV: </span>
                            <Battery fill="#3b669c" orientation="horizontal" perc={Number($lvBattery.value)}/>
<!--                            <span>Total: <Store datatype="TotalBatteryVoltageLow" /></span>-->

                            <span>HV: </span>
                            <Battery fill="#723f9c" orientation="horizontal" perc={Number($hvBattery.value)}/>
<!--                            <span>Total: <Store datatype="TotalBatteryVoltageHigh" /></span>-->
                        </div>
                        <div class="flex flex-col gap-4">
                            <span>High Voltage BMS: &ltstatus&gt</span>
                            <span>Emergency Breaking System: &ltstatus&gt</span>
                        </div>
                        <div class="flex flex-col gap-4">
                            <span>LV Total Safe: -Insert values- V</span>
                            <span>HV Total Safe: -Insert values- V</span>
                        </div>
                    </div>
                    <div class="flex flex-wrap justify-between mt-4">
                        <div class="flex gap-4 flex-wrap">
                            <Command cmd="StopHV" className="py-1 text-error-400 border-error-400 border-2" />
                            <Command cmd="ArmBrakes" className="py-1 bg-primary-500 text-surface-900 " />
<!--                            <Command cmd="StartRun" className="py-1 bg-primary-500 text-surface-900" />-->
<!--                            <Command cmd="ContinueRun" className="py-1 bg-primary-500 text-surface-900" />-->
                        </div>
                    </div>
                </Tile>
<!--                <Tile containerClass="pt-2 pb-1 col-span-2" bgToken={800}>-->
<!--                    <Table titles={tableBatteryTitles} tableArr={tableBatteryVitals}/>-->
<!--                </Tile>-->
<!--                <Tile containerClass="pt-2 pb-1 col-span-2" bgToken={800}>-->
<!--                    <Table tableArr={tableTempsArr} titles={["Module", "Temp °C", "Safe range", "Module", "Temp °C", "Safe range"]}/>-->
<!--                </Tile>-->
<!--                <Tile containerClass="pt-2 pb-1 col-span-2" bgToken={800}>-->
<!--                    <Table titles={["Datatype", "Value", "Safe range", "Datatype", "Value", "Safe range"]} tableArr={tableArr2}/>-->
<!--                </Tile>-->
                <Tile
                    bgToken={800}
                    containerClass="col-span-2 {$fsmState.value === 13 || $showcaseStateCounter === 13 && $showcasingStates ? 'shadow-[inset_0_0_10px_5px_rgba(214,17,17,1)]' : ''}">
                    <MainFSM/>
                </Tile>
            </TileGrid>
        </div>
    {/if}
</div>

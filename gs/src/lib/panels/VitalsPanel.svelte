<script lang="ts">
    import { Battery, TileGrid, Tile, Command, GrandDataDistributor, Store, TauriCommand, serverStatus } from '$lib';
    import { AppBar, getToastStore } from '@skeletonlabs/skeleton';
    import {invoke} from "@tauri-apps/api/tauri";
    import {DatatypeEnum as DE} from "$lib/namedDatatypeEnum";
    import Localization from '$lib/components/Localization.svelte';
    import Light from '$lib/components/Light.svelte';
    import MainFSM from '$lib/components/MainFSM.svelte';
    import {
        connectedToMainPCB,
        debugModeActive,
        inStateAccelerating,
        inStateCruising,
        inStateHVOn,
        inStateLevitating,
        inStateSystemCheck,
        showcaseStateCounter,
        showcasingStates,
    } from '$lib/stores/state';
    import {
        Activity,
        Wifi,
        WifiOff,
        Flash,
        FlashOff,
        QComposerEdit,
        Reset,
        SettingsCheck,
        Meter,
        RightPanelClose,
        ConnectionSignal,
        ConnectionSignalOff,
        SendAlt,
        StopFilled,
        Password
    } from 'carbon-icons-svelte';
    import type { SvelteComponent } from 'svelte';
    import StartLevitating from '$lib/components/StartLevitating.svelte';
    import StopLevitating from '$lib/components/StopLevitating.svelte';
    import { PTCErrorCode } from '$lib/types';

    let width: number;

    const storeManager = GrandDataDistributor.getInstance().stores;
    const lvBattery = storeManager.getWritable("BMSVoltageLow");
    const hvBattery = storeManager.getWritable("BMSVoltageHigh");
    const fsmStateStore = storeManager.getWritable("FSMState");
    const ptcStateStore = storeManager.getWritable("PTCState");
    const ptcFaultStore = storeManager.getWritable("PTCNonCriticalFault");
    const localization = storeManager.getWritable("Localization");
    const velocity = storeManager.getWritable("Velocity");

    const StartLevitatingIcon = StartLevitating as unknown as typeof SvelteComponent;
    const StopLevitatingIcon = StopLevitating as unknown as typeof SvelteComponent;

    $: ptcFaultMessages = Object.entries(PTCErrorCode)
        .filter(([key, value]) => typeof value === 'number' && ($ptcFaultStore[0]?.value & value) !== 0)
        .map(([key]) => key.replace(/_/g, ' '));

    const toastStore = getToastStore();
    const handleSuccess = () => {
        toastStore.trigger({
            message: "Server started successfully",
            background: "bg-primary-400",
            timeout: 1500
        });
        serverStatus.set(true);
    };

    const handleFailure = (error:string) => {
        toastStore.trigger({
            message: `Server did not start successfully: ${error}`,
            background: "bg-error-400"
        });
    };

    let tableTempsArr: any[][];
    let tableArr2: any[][];

    let tableBatteryTitles = ["", "HV Voltages", "HV Temp", "LV Voltages", "LV Temp"]

    let ptcStates = [
        "Idle",
        "Precharge",
        "HV On",
        "Failure",
        "Discharge",
    ]

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
</script>

<div bind:clientWidth={width} class="h-full bg-surface-700 text-surface-50">
    <AppBar padding="px-5 pt-3 pb-3" border="border-b border-b-surface-900" background="bg-surface-700" slotDefault="place-self-center">
        <svelte:fragment slot="lead">
            <div class="gap-2 flex flex-row items-center">
                <Activity size={16}/>
                <span>Vitals</span>
            </div>
        </svelte:fragment>

        <div class="flex gap-5 items-center justify-center">
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
                <Battery fill="#3b669c" orientation="vertical" height={55} perc={Number($lvBattery.values)}/>
                <Battery fill="#723f9c" orientation="vertical" height={55} perc={Number($hvBattery.values)}/>
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
                        <div class="flex justify-between flex-col">
                            <p>Velocity: <Store datatype="Velocity" /></p>
                            <p>Position: <Store datatype="Localization" /></p>
                            <p>LV Temp High: <Store datatype="BMSTemperatureLow" /></p>
                            <p>HV Temp High: <Store datatype="BMSTemperatureHigh" /></p>
                        </div>
                        <div style="grid-template-columns: 1fr 2fr 3fr;" class="grid gap-2 items-center">
                            <span>LV:</span>
                            <Battery fill="#3b669c" orientation="horizontal" perc={Number($lvBattery.values)}/>
                            <span>Total: <Store datatype="BMSVoltageLow" /></span>

                            <span>HV:</span>
                            <Battery fill="#723f9c" orientation="horizontal" perc={Number($hvBattery.values)}/>
                            <span>Total: <Store datatype="BMSVoltageHigh" /></span>
                        </div>
                        <div class="flex flex-col gap-4">
                            <span>PT Controller State: {ptcStates[$ptcStateStore[0]?.value]}</span>
                            <span>PT Controller Fault: {ptcFaultMessages.length > 0 ? ptcFaultMessages.join(', ') : 'None'}</span>
                            <span>IMD: &ltstatus&gt</span>
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
                    containerClass="col-span-2 {($fsmStateStore[0]?.value === 13 || $showcaseStateCounter === 13) && $showcasingStates ? 'shadow-[inset_0_0_10px_5px_rgba(214,17,17,1)]' : ''}">
                    <MainFSM/>
                </Tile>
                <Tile
                    bgToken={700}
                    containerClass={"col-span-2"}
                >
                    <div class="gap-4 justify-center grid grid-cols-4">
                        {#if !$serverStatus}
                            <TauriCommand
                                cmd="connect_to_pod"
                                successCallback={handleSuccess}
                                errorCallback={handleFailure}
                                icon={Wifi}
                            />
                        {:else}
                            <TauriCommand
                                cmd="disconnect"
                                on:click={() => serverStatus.set(false)}
                                successCallback={() => serverStatus.set(false)}
                                errorCallback={(error) => {
                                    toastStore.trigger({
                                        message: `Server is not running: ${error}`,
                                        background: "bg-error-400"
                                    });
                                }}
                                icon={WifiOff}
                            />
                        {/if}
                        <Command
                            cmd="SystemCheck"
                            icon={SettingsCheck}
                        />
                        {#if !$inStateHVOn}
                            <Command cmd="StartHV" text="Start HV" icon={Flash}/>
                        {:else}
                            <Command cmd="StopHV" text="Stop HV" className="text-error-400 border-error-400 border-2" icon={FlashOff}/>
                        {/if}
                        <Command cmd="RearmSDC" text="Rearm SDC" icon={QComposerEdit}/>
                        <!-- <Command cmd="RetractBrakes" icon={RightPanelClose}/> -->
                        {#if !$inStateLevitating}
                            <Command cmd="LevitationOn" icon={StartLevitatingIcon}/>
                        {:else}
                            <Command cmd="LevitationOff" icon={StopLevitatingIcon}/>
                        {/if}
                        {#if !$inStateAccelerating}
                            <Command cmd="PropulsionOn" icon={Meter}/>
                        {:else}
                            <Command cmd="PropulsionOff" icon={Meter} iconClass="scale-x-[-1]"/>
                        {/if}
                        <Command cmd="SystemReset" icon={Reset}/>
                    </div>
                </Tile>
                <Tile
                    bgToken={700}
                >
                    <div class="flex gap-2 items-center">
                        <span>Connection Status:</span>
                        <div class="flex flex-row items-center gap-1">
                            {#if !$connectedToMainPCB}
                                <ConnectionSignalOff size={20}/>
                                <span>Not Connected</span>
                            {:else}
                                <ConnectionSignal size={20}/>
                                <span>Connected</span>
                            {/if}
                        </div>
                    </div>
                </Tile>
                {#if $debugModeActive}
                    <Tile bgToken={700} containerClass="col-span-2 bg-surface-800" heading="Propulsion FSM Update Commands" headingOnLeft={true}>
                        <div class="flex gap-4 flex-wrap">
                            <Command cmd="FSMUpdate" text="Initialization" val={200} className="py-3 bg-primary-500 text-surface-900"/>
                            <Command cmd="FSMUpdate" text="Idle" val={201}/>
                            <Command cmd="FSMUpdate" text="Running" val={202}/>
                            <Command cmd="FSMUpdate" text="Braking" val={203}/>
                        </div>
                    </Tile>
                    <Tile bgToken={700} containerClass="col-span-2 bg-surface-800" heading="Powertrain FSM Update Commands" headingOnLeft={true}>
                        <div class="flex gap-4 flex-wrap">
                            <Command cmd="FSMUpdate" text="HV On" val={4} className="py-3 bg-primary-500 text-surface-900"/>
                            <Command cmd="FSMUpdate" text="Idle" val={11}/>
                            <Command cmd="FSMUpdate" text="Reset Fault from Precharge Fault" val={3}/>
                        </div>
                    </Tile>
                {/if}
            </TileGrid>
        </div>
    {/if}
</div>

<script lang="ts">
    import {
        Battery,
        TileGrid,
        Tile,
        Command,
        GrandDataDistributor,
        Store,
        TauriCommand,
        serverStatus,
        Table,
    } from '$lib';
    import { AppBar, getToastStore } from '@skeletonlabs/skeleton';
    import {invoke} from "@tauri-apps/api/tauri";
    import Localization from '$lib/components/Localization.svelte';
    import Light from '$lib/components/Light.svelte';
    import MainFSM from '$lib/components/MainFSM.svelte';
    import {
        connectedToMainPCB,
        debugModeActive,
        inStateAccelerating, inStateBraking,
        inStateCharging,
        inStateLevitating,
        showcaseStateCounter,
        showcasingStates,
    } from '$lib/stores/state';
    import {
        Activity,
        Wifi,
        WifiOff,
        Flash,
        FlashOff,
        ChargingStation,
        Reset,
        SettingsCheck,
        Meter,
        RightPanelClose,
        ConnectionSignal,
        ConnectionSignalOff,
        StopOutline,
        Tools
    } from 'carbon-icons-svelte';
    import type { SvelteComponent } from 'svelte';
    import StartLevitating from '$lib/components/StartLevitating.svelte';
    import StopLevitating from '$lib/components/StopLevitating.svelte';
    import { inStateDemo, inStateIdle } from '$lib/stores/state.js';
    import { imdWarnings, ptcErrorCodes, ptcStates } from '$lib/types';

    let width: number;

    const storeManager = GrandDataDistributor.getInstance().stores;
    const lvBattery = storeManager.getWritable("BMSVoltageLow");
    const hvBattery = storeManager.getWritable("BMSVoltageHigh");
    const fsmState = storeManager.getWritable("FSMState");
    const ptcState = storeManager.getWritable("PTCState");
    const localization = storeManager.getWritable("Localization");
    const velocity = storeManager.getWritable("Velocity");
    const ptcFaultStore = storeManager.getWritable("PTCErrors");
    const imdWarningStore1 = storeManager.getWritable("IMDWarnings1");
    const imdWarningStore2 = storeManager.getWritable("IMDWarnings2");

    const StartLevitatingIcon = StartLevitating as unknown as typeof SvelteComponent;
    const StopLevitatingIcon = StopLevitating as unknown as typeof SvelteComponent;

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

    $: ptcFaultMessage = ptcErrorCodes.filter((x, index) =>
        ((($ptcFaultStore.value >> index) & 1) == 1)
    );

    $: imdWarningMessage = imdWarnings.filter((x, index) =>
        ((((($imdWarningStore1.value >> 16) & $imdWarningStore2.value) >> index) & 1) == 1)
    );

    imdWarningStore1.subscribe((store) => {
        console.log(`imd warning store 1: ${store.value}`)
    })

    imdWarningStore2.subscribe((store) => {
        console.log(`imd warning store 2: ${store.value}`)
    })

    async function sendSystemCheckMocks() {
        await invoke('send_command', {cmdName: "MockPtAck", val: 0}).then(() => {
            console.log(`Command MockPtAck sent`);
        }).catch((e) => {
            console.error(`Error sending command MockPtAck: ${e}`);
        });
        await invoke('send_command', {cmdName: "MockLeviAck", val: 0}).then(() => {
            console.log(`Command MockLeviAck sent`);
        }).catch((e) => {
            console.error(`Error sending command MockLeviAck: ${e}`);
        });
        await invoke('send_command', {cmdName: "MockProp1Ack", val: 0}).then(() => {
            console.log(`Command MockProp1Ack sent`);
        }).catch((e) => {
            console.error(`Error sending command MockProp1Ack: ${e}`);
        });
        await invoke('send_command', {cmdName: "MockProp2Ack", val: 0}).then(() => {
            console.log(`Command MockProp2Ack sent`);
        }).catch((e) => {
            console.error(`Error sending command MockProp2Ack: ${e}`);
        });
    }
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
                        <div class="flex justify-between flex-col">
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
                            <span>Velocity: {$velocity.value} m/s</span>
                            <span>Acceleration: // m/s²</span>
                            <span>Position: {$localization.value / 100} mm</span>
                        </div>
                        <div style="grid-template-columns: 1fr 2fr 3fr;" class="grid gap-2 items-center">
                            <span>LV:</span>
                            <Battery fill="#3b669c" orientation="horizontal" perc={Number($lvBattery.value)}/>
                            <span>Total: <Store datatype="BMSVoltageLow" /></span>

                            <span>HV:</span>
                            <Battery fill="#723f9c" orientation="horizontal" perc={Number($hvBattery.value)}/>
                            <span>Total: <Store datatype="BMSVoltageHigh" /></span>
                        </div>
                        <div class="flex flex-col gap-4">
                            <span>PT Controller State: {ptcStates[$ptcState.value]}</span>
                            <span>PT Controller Fault: {ptcFaultMessage.length === 0 ? "None" : ptcFaultMessage.join(", ")}</span>
                            <span>IMD Status??: &ltstatus&gt</span>
                            <span>IMD Warning: {imdWarnings.length === 0 ? "None" : imdWarningMessage.join(", ")}</span>
                        </div>
                    </div>
                </Tile>
                <Tile
                    bgToken={800}
                    containerClass="col-span-2 {$fsmState.value === 13 || $showcaseStateCounter === 13 && $showcasingStates ? 'shadow-[inset_0_0_10px_5px_rgba(214,17,17,1)]' : $connectedToMainPCB ? '' : ''}">
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
                        {#if $fsmState.value < 5}
                            <Command
                                cmd="StartHV"
                                text="Start HV"
                                icon={Flash}
                                dependency={inStateIdle}
                                dependencyTitle="Wrong State!"
                                dependencyMessage="The pod should be in the Idle state to turn on high voltage!"
                            />
                        {:else}
                            <Command cmd="StopHV" text="Stop HV" className="text-error-400 border-error-400 border-2" icon={FlashOff}/>
                        {/if}
                        <Command cmd="RearmSDC" text="Rearm SDC" icon={RightPanelClose}/>
                        {#if $inStateAccelerating || $inStateBraking || $inStateLevitating}
                            <Command cmd="LevitationOff" icon={StopLevitatingIcon}/>
                        {:else}
                            <Command
                                cmd="LevitationOn"
                                icon={StartLevitatingIcon}
                                dependency={inStateDemo}
                                dependencyMessage="The pod should be in the Demo state to start levitating!"
                                dependencyTitle="Wrong State!"
                            />
                        {/if}
                        {#if !$inStateAccelerating}
                            <Command
                                cmd="PropulsionOn"
                                icon={Meter}
                                dependency={inStateLevitating}
                                dependencyTitle="Wrong State!"
                                dependencyMessage="The pod should be in the Levitating state to turn on the motor!"
                            />
                        {:else}
                            <Command cmd="MotorBrake" icon={Meter} iconClass="scale-x-[-1]"/>
                        {/if}
                        {#if !$inStateCharging}
                            <Command cmd="Charge" icon={ChargingStation}/>
                        {:else}
                            <Command cmd="StopCharge" icon={StopOutline}/>
                        {/if}
                        <Command cmd="SystemReset" icon={Reset}/>
                        <Command cmd="FaultFixed" icon={Tools}/>
                    </div>
                </Tile>
                {#if $debugModeActive}
                    <Tile containerClass="col-span-full bg-surface-800" bgToken={700}>
                        <button on:click={sendSystemCheckMocks} class="btn rounded-md font-number font-medium text-wrap flex-wrap bg-primary-500 text-surface-900">
                            Pass System Check
                        </button>
                    </Tile>
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
<!--                    <Tile bgToken={700} containerClass="col-span-full">-->
<!--                        <Table titles={tableBatteryTitles} tableArr={tableBatteryVitals}/>-->
<!--                    </Tile>-->
                {/if}
            </TileGrid>
        </div>
    {/if}
</div>

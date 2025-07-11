<script lang="ts">
    import { Chart, GrandDataDistributor, Pneumatics, Table, Tile, TileGrid } from "$lib"
    import {DatatypeEnum as DE} from "$lib/namedDatatypeEnum";
    import { util } from "$lib";
    import { onMount, onDestroy } from "svelte";

    const storeManager = GrandDataDistributor.getInstance().stores;
    // const lowPressure = storeManager.getWritable("LowPressureSensor");
    // const highPressure = storeManager.getWritable("HighPressureSensor");

    // $: pressureTable = [
    //     ["Low Pressure", DE.LOWPRESSURESENSOR],
    //     ["High Pressure", DE.HIGHPRESSURESENSOR],
    //     ["Braking Comm", DE.BRAKINGCOMMDEBUG],
    //     ["Braking Signal", DE.BRAKINGSIGNALDEBUG],
    //     ["Braking Rearm", DE.BRAKINGREARMDEBUG],
    // ]

    export const pop_up: boolean = true;

    export let pressure_left: number;
    export let pressure_right: number;
    export let max_pressure_left: number;
    export let max_pressure_right: number;
    export let background = "bg-surface-900";

    let colour_left: string;
    let colour_right: string;
    $: {
        colour_left = util.colourCode(pressure_left, max_pressure_left);
        colour_right = util.colourCode(pressure_right, max_pressure_right);
    }

    // --- Brake Pressure and FSM State Logic ---
    const BRAKES_DEPLOYED_STATES = [0, 1, 2, 3, 4, 9, 10, 11, 12]; // boot, connected to gs, system check, idle, precharge, discharge, braking, charging, fault
    let pressureBrakesValue = 0;
    let fsmStateValue = 0;
    let storesReady = false;
    let unsubscribePressure: (() => void) | undefined;
    let unsubscribeFSM: (() => void) | undefined;

    let pressureBrakes, fsmState;

    onMount(() => {
        try {
            pressureBrakes = storeManager.getWritable("PressureBrakes1");
            fsmState = storeManager.getWritable("FSMState");
            if (pressureBrakes && fsmState) {
                unsubscribePressure = pressureBrakes.subscribe(val => pressureBrakesValue = val.value ?? val);
                unsubscribeFSM = fsmState.subscribe(val => fsmStateValue = val.value ?? val);
                storesReady = true;
            } else {
                console.error("PressureBrakes1 or FSMState store not found");
            }
        } catch (e) {
            console.error("Error accessing stores:", e);
        }
    });

    onDestroy(() => {
        unsubscribePressure && unsubscribePressure();
        unsubscribeFSM && unsubscribeFSM();
    });

    let brakeShouldBeDeployedErrorAcknowledged = true;
    let brakePressureWarningActive = false;
    let lastPressure = 1000;
    $: if (storesReady) {
        const brakesShouldBeDeployed = BRAKES_DEPLOYED_STATES.includes(fsmStateValue);
        const brakesDeployed = pressureBrakesValue < 1;
        if (brakesShouldBeDeployed && !brakesDeployed && brakeShouldBeDeployedErrorAcknowledged) {
            brakeShouldBeDeployedErrorAcknowledged = false;
            // Show red error popup
            window?.gdd?.toastStore?.trigger?.({
                message: 'ERROR: Brakes should be deployed in this state, but pressure indicates they are not! Check brake system.',
                background: 'bg-error-400',
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        if (brakesDeployed || !brakesShouldBeDeployed) brakeShouldBeDeployedErrorAcknowledged = true;
                    }
                },
            });
        } else if (brakesDeployed || !brakesShouldBeDeployed) {
            brakeShouldBeDeployedErrorAcknowledged = true;
        }
        if (lastPressure >= 30 && pressureBrakesValue < 30 && !brakePressureWarningActive) {
            brakePressureWarningActive = true;
            window?.gdd?.toastStore?.trigger?.({
                message: 'WARNING: Brake pressure has dropped below 30 bar. Possible leak detected.',
                background: 'bg-warning-400',
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        if (pressureBrakesValue > 32) brakePressureWarningActive = false;
                    }
                },
            });
        } else if (pressureBrakesValue > 32) {
            brakePressureWarningActive = false;
        }
        lastPressure = pressureBrakesValue;
    }
    $: brakesShouldBeDeployed = BRAKES_DEPLOYED_STATES.includes(fsmStateValue);
    $: brakesDeployed = pressureBrakesValue < 1;
    $: brakesRetracted = pressureBrakesValue > 50;
    $: brakesInTransition = !brakesDeployed && !brakesRetracted;
    $: brakesStatus = brakesDeployed ? "Deployed" : brakesRetracted ? "Retracted" : "Transitioning";
    $: statusColor = brakesDeployed ? "bg-green-600" : brakesRetracted ? "bg-orange-500" : "bg-gray-500";
    $: showWarning = (brakesShouldBeDeployed && !brakesDeployed) || (!brakesShouldBeDeployed && !brakesRetracted);
    $: warningMsg = brakesShouldBeDeployed
        ? "Brakes should be DEPLOYED, but pressure indicates otherwise!"
        : "Brakes should be RETRACTED, but pressure indicates otherwise!";
    $: isNormal = !showWarning;
    $: statusMsg = isNormal ? 'Brakes status: NORMAL' : 'Brakes status: NOT NORMAL';
    $: statusMsgColor = isNormal ? 'text-green-500' : 'text-red-500';
    $: indicatorColor = isNormal ? 'bg-green-600' : 'bg-red-600';

    let fakeDataInterval: ReturnType<typeof setInterval> | null = null;

    function startFakeData() {
        let t = 0;
        stopFakeData(); // Ensure no duplicate intervals
        fakeDataInterval = setInterval(() => {
            // Generate a fake pressure value (e.g., sine wave between 0 and 60)
            const value = 30 + 30 * Math.sin(t / 10);
            pressureBrakes.set({ value });
            // Optionally, update FSM state as well (e.g., alternate between 10 and 3)
            fsmState.set({ value: t % 20 < 10 ? 10 : 3 });
            // Push to chart if available
            if (window.gdd && window.gdd.$chartStore) {
                window.gdd.$chartStore.get("Breaking Comms")?.addEntry(1, value);
            }
            t++;
        }, 500); // every 0.5 seconds
    }

    function stopFakeData() {
        if (fakeDataInterval) {
            clearInterval(fakeDataInterval);
            fakeDataInterval = null;
        }
    }
</script>

{#if storesReady}
<div class="{background} p-4 rounded-lg flex justify-center">
    <div class="w-full">
        <!-- Brake Status Indicator -->
        <div class="flex items-center mb-2">
            <div class="w-4 h-4 rounded-full mr-2 {indicatorColor}"></div>
            <span class="font-semibold">Brakes: {brakesStatus} (Pressure: {pressureBrakesValue.toFixed(1)} bar)</span>
        </div>
        <div class="{statusMsgColor} font-semibold mb-2">{statusMsg}</div>
        {#if showWarning}
            <div class="text-red-500 font-bold mb-2">{warningMsg}</div>
        {/if}
        <!-- Existing SVG and content below -->
        <TileGrid columns="3fr 1fr" rows="">
            <Tile>
<!--                <Pneumatics pressure_left={$highPressure.value} max_pressure_left={250}-->
<!--                            pressure_right={$lowPressure.value} max_pressure_right={100}/>-->
            </Tile>
            <Tile>
<!--                <Table background="bg-surface-900" titles={["Section", "Bars"]} tableArr={pressureTable}/>-->
            </Tile>
            <Tile containerClass="col-span-full">
                <Chart title="Breaking Comms"/>
            </Tile>
        </TileGrid>
    </div>
</div>
{:else}
<div class="p-4 text-red-500">Pneumatics data not available. Check backend and store registration.</div>
{/if}
<script lang="ts">
    import '../app.postcss';
    import {
        BottomBar,
        GrandDataDistributor,
        PlotBuffer,
        StrokePresets,
        TitleBar,
    } from "$lib";
    import { initializeStores, Modal, type ModalComponent, Toast } from '@skeletonlabs/skeleton';
    import {
        chartStore,
        debugModeActive,
        latestTimestamp, memeModeActive,
        showcaseStateCounter,
        showcasingStates,
    } from '$lib/stores/state';
    import {initProcedures} from "$lib/stores/data";
    import { onDestroy, onMount } from 'svelte';
    import {listen} from "@tauri-apps/api/event";
    import {parseShortCut} from "$lib/util/parsers";
    import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
    import { storePopup } from '@skeletonlabs/skeleton';
    import { LOCALISATION_NAME } from '$lib/types';
    import AlertModal from '$lib/components/AlertModal.svelte';

    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    const modalRegistry: Record<string, ModalComponent> = {
        alertModal: { ref: AlertModal },
    };

    initProcedures();

    const unlisten = listen("shortcut_channel", (event: {payload: string}) => parseShortCut(event.payload, $debugModeActive, $memeModeActive));

    //////////////////////////////
    /////////// CHARTS ///////////
    //////////////////////////////

    let breakingCommsChart = new PlotBuffer(500, 60000, [0, 120], true, "Breaking Comms");
    $chartStore.set("Breaking Comms", breakingCommsChart);

    let emsTempChart = new PlotBuffer(500, 60000, [0, 120], true, "EMS 1");
    emsTempChart.addSeries(StrokePresets.theoretical("EMS 2"))
    $chartStore.set("EMS Temperatures", emsTempChart);

    let airGapChart = new PlotBuffer(500, 60000, [0, 30], true, "Vertical Air Gap");
    airGapChart.addSeries(StrokePresets.theoretical("Lateral Air Gap"));
    $chartStore.set("Air Gaps", airGapChart);
    // max vertical: 27.5
    // max lateral: 16

    let rotationChart = new PlotBuffer(500, 60000, [0, 120], true, "Pitch");
    rotationChart.addSeries(StrokePresets.theoretical("Roll"));
    $chartStore.set("Rotations", rotationChart);
    // TODO: max rotations?

    let hemsTempChart = new PlotBuffer(500, 60000, [0, 120], true, "HEMS 1");

    hemsTempChart.addSeries(StrokePresets.theoretical("HEMS 2"))
    hemsTempChart.addSeries(StrokePresets.yellow("HEMS 3"))
    hemsTempChart.addSeries(StrokePresets.blue("HEMS 4"))
    $chartStore.set("HEMS Temperatures", hemsTempChart);

    let hemsCurrentChart = new PlotBuffer(500, 3*60000, [-11.3, 11.3], true, "a1");
    hemsCurrentChart.addSeries(StrokePresets.hyperloopGreenDashed("a2"))
    hemsCurrentChart.addSeries(StrokePresets.theoretical("b1"))
    hemsCurrentChart.addSeries(StrokePresets.theoreticalDashed("b2"))
    hemsCurrentChart.addSeries(StrokePresets.yellow("c1"))
    hemsCurrentChart.addSeries(StrokePresets.yellowDashed("c2"))
    hemsCurrentChart.addSeries(StrokePresets.blue("d1"))
    hemsCurrentChart.addSeries(StrokePresets.blueDashed("d2"))
    $chartStore.set("HEMS Current", hemsCurrentChart);

    let emsCurrentChart = new PlotBuffer(500, 3*60000, [-11.3, 11.3], true);
    emsCurrentChart.addSeries(StrokePresets.theoretical("cd"))
    $chartStore.set("EMS Current", emsCurrentChart);

    let voffChart = new PlotBuffer(500, 60000, [8, 25], false)
    $chartStore.set('Offset Vertical', voffChart);

    let accelChart = new PlotBuffer(500, 60000, [0, 25], false)
    $chartStore.set('Acceleration', accelChart);

    let rolPitchChart = new PlotBuffer(500, 60000, [-0.8, 0.8], true, "roll")
    rolPitchChart.addSeries(StrokePresets.theoretical("pitch"))
    $chartStore.set("Roll Pitch", rolPitchChart)

    let hoffChart = new PlotBuffer(500, 60000, [-8, 8], true, "ab")
    hoffChart.addSeries(StrokePresets.theoretical("cd"))
    $chartStore.set('Offset Horizontal', hoffChart);

    let velChart = new PlotBuffer(500, 60000, [0, 100], false)
    $chartStore.set('Velocity', velChart);

    let localisationChart = new PlotBuffer(500, 60000, [0, 13000], false);
    $chartStore.set(LOCALISATION_NAME, localisationChart);

    let trr = new PlotBuffer(500, 60000, [0, 50], false)
    trr.addSeries(StrokePresets.theoretical())
    $chartStore.set('Theoretical vs Real run', trr)

    let lvCurrent = new PlotBuffer(500, 60000, [-15, 15], false)
    $chartStore.set('LV Current', lvCurrent)

    let hvCurrent = new PlotBuffer(500, 60000, [-15, 15], false)
    $chartStore.set('HV Current', hvCurrent)

    let lvTotal = new PlotBuffer(500, 2*60000, [-1, 30], false)
    $chartStore.set('LV Total', lvTotal)

    let hvTotal = new PlotBuffer(500, 2*60000, [-10, 450], false)
    $chartStore.set('HV Total', hvTotal)

    ///////////////////////////////////////////////////////
    //////////////// BMS REGISTER /////////////////////////
    ///////////////////////////////////////////////////////

    let gdd = GrandDataDistributor.getInstance();

    ////////////////////////////////////////////////////////////////
    ///////////////// PROPULSION REGISTER //////////////////////////
    ////////////////////////////////////////////////////////////////

    // Prop log 1 chart for test runs
    let propLog1Chart = new PlotBuffer(500, 3*60000, [0, 20], true, "Id measured");
    propLog1Chart.addSeries(StrokePresets.yellow("Id reference"))
    propLog1Chart.addSeries(StrokePresets.blue("Iq measured"))
    propLog1Chart.addSeries(StrokePresets.theoretical("Iq reference"))
    $chartStore.set("Propulsion Log 1", propLog1Chart);

    // Prop log 2 chart for test runs
    let propLog2Chart = new PlotBuffer(500, 3*60000, [0, 20], true, "VQ");
    propLog2Chart.addSeries(StrokePresets.yellow("VD"))
    propLog2Chart.addSeries(StrokePresets.blue("Vbus"))
    propLog2Chart.addSeries(StrokePresets.theoretical("Ibus"))
    $chartStore.set("Propulsion Log 2", propLog2Chart);

    // Prop log 3 chart for test runs
    let propLog3Chart = new PlotBuffer(500, 3*60000, [0, 20], true, "Ta");
    propLog3Chart.addSeries(StrokePresets.yellow("Tb"))
    propLog3Chart.addSeries(StrokePresets.blue("Tc"))
    propLog3Chart.addSeries(StrokePresets.theoretical("TCASE"))
    $chartStore.set("Propulsion Log 3", propLog3Chart);

    // generated

    gdd.stores.registerStore<number>("TempAmbient0", 0);

    gdd.stores.registerStore<number>("TempAmbient1", 0);

    gdd.stores.registerStore<number>("TempAmbient2", 0);

    gdd.stores.registerStore<number>("BMSVoltageHigh", 0);

    gdd.stores.registerStore<number>("BMSVoltageLow", 0);

    gdd.stores.registerStore<number>("BMSTemperatureHigh", 0);

    gdd.stores.registerStore<number>("BMSTemperatureLow", 0);

    gdd.stores.registerStore<number>("TempRangeStart", 0);

    gdd.stores.registerStore<number>("TempRangeEnd", 0);

    gdd.stores.registerStore<number>("Loc1", 0);

    gdd.stores.registerStore<number>("Loc2", 0);

    gdd.stores.registerStore<number>("Temp0", 0);

    gdd.stores.registerStore<number>("Temp1", 0);

    gdd.stores.registerStore<number>("Temp2", 0);

    gdd.stores.registerStore<number>("Temp3", 0);

    gdd.stores.registerStore<number>("Temp4", 0);

    gdd.stores.registerStore<number>("Temp5", 0);

    gdd.stores.registerStore<number>("Temp6", 0);

    gdd.stores.registerStore<number>("Temp7", 0);

    gdd.stores.registerStore<number>("ResetSenseCon", 0);

    gdd.stores.registerStore<number>("ResetPropulsion", 0);

    gdd.stores.registerStore<number>("ModulationFactor1", 0);

    gdd.stores.registerStore<number>("ModulationFactor2", 0);

    gdd.stores.registerStore<number>("MaximumVelocity1", 0);

    gdd.stores.registerStore<number>("MaximumVelocity2", 0);

    gdd.stores.registerStore<number>("Kpq1", 0);

    gdd.stores.registerStore<number>("Kpq2", 0);

    gdd.stores.registerStore<number>("Kiq1", 0);

    gdd.stores.registerStore<number>("Kiq2", 0);

    gdd.stores.registerStore<number>("Kpd1", 0);

    gdd.stores.registerStore<number>("Kpd2", 0);

    gdd.stores.registerStore<number>("Kid1", 0);

    gdd.stores.registerStore<number>("Kid2", 0);

    gdd.stores.registerStore<number>("PositionOffset1", 0);

    gdd.stores.registerStore<number>("PositionOffset2", 0);

    gdd.stores.registerStore<number>("Alpha1", 0);

    gdd.stores.registerStore<number>("Alpha2", 0);

    gdd.stores.registerStore<number>("Iq1", 0);

    gdd.stores.registerStore<number>("Iq2", 0);

    gdd.stores.registerStore<number>("Id1", 0);

    gdd.stores.registerStore<number>("Id2", 0);

    gdd.stores.registerStore<number>("Vq1_C", 0);

    gdd.stores.registerStore<number>("Vq2_C", 0);

    gdd.stores.registerStore<number>("Vd1_C", 0);

    gdd.stores.registerStore<number>("Vd2_C", 0);

    gdd.stores.registerStore<number>("PPInitFault", 0);

    gdd.stores.registerStore<number>("PPEmergency", 0);

    gdd.stores.registerStore<number>("Word", 0);

    gdd.stores.registerStore<number>("IqMeasured", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IqReference", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdMeasured", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdReference", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vq_Log", 0);

    gdd.stores.registerStore<number>("Vd_Log", 0);

    gdd.stores.registerStore<number>("Vbus", 0);

    gdd.stores.registerStore<number>("Ibus", 0);

    gdd.stores.registerStore<number>("Ta", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tb", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tc", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TCASE", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("PropulsionCurrent", 0);

    gdd.stores.registerStore<number>("Localisation", 0, data => {
            const curr = Number(data);
            $chartStore.get("Localisation")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("FSMState", 0);

    // End of generated

    gdd.start(50);

    initializeStores();

    setInterval(() => {
       latestTimestamp.set(Date.now());
    }, 1000)

    onMount(() => {
        setInterval(() => {
            if ($showcasingStates) {
                showcaseStateCounter.set(($showcaseStateCounter + 1) % 14);
            }
        }, 1000)
    })

    onDestroy(async () => {
      GrandDataDistributor.getInstance().kill();
      (await unlisten)();
    })
</script>


<div class="flex flex-col w-screen h-screen max-h-screen overflow-hidden">
    <Toast/>
    <Modal components={modalRegistry}/>
    <TitleBar/>
    <slot/>
    <BottomBar/>
</div>


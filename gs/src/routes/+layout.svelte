<script lang="ts">
    import '../app.postcss';
    import {
        BottomBar,
        GrandDataDistributor,
        PlotBuffer,
        StrokePresets,
        TitleBar,
    } from '$lib';
    import {
        initializeStores,
        Modal,
        type ModalComponent,
        Toast,
    } from '@skeletonlabs/skeleton';
    import {
        chartStore,
        debugModeActive,
        latestTimestamp,
        logsPanelSize,
        logsVisible,
        showcaseStateCounter,
        showcasingStates,
    } from '$lib/stores/state';
    import { initProcedures } from '$lib/stores/data';
    import { onDestroy, onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import { parseShortCut } from '$lib/util/parsers';
    import {
        computePosition,
        autoUpdate,
        offset,
        shift,
        flip,
        arrow,
    } from '@floating-ui/dom';
    import { storePopup } from '@skeletonlabs/skeleton';
    import AlertModal from '$lib/components/AlertModal.svelte';

    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    const modalRegistry: Record<string, ModalComponent> = {
        alertModal: { ref: AlertModal },
    };

    initProcedures();

    logsVisible.subscribe(() => {
        if ($logsVisible) {
            logsPanelSize.set(30);
        } else {
            logsPanelSize.set(5);
        }
    });

    const unlisten = listen('shortcut_channel', (event: { payload: string }) =>
        parseShortCut(event.payload, $debugModeActive, $logsVisible)
    );

    //////////////////////////////
    /////////// CHARTS ///////////
    //////////////////////////////

    let breakingCommsChart = new PlotBuffer(
        500,
        60000,
        [0, 120],
        true,
        'Breaking Comms'
    );
    $chartStore.set('Breaking Comms', breakingCommsChart);

    let emsTempChart = new PlotBuffer(500, 60000, [0, 120], true, 'EMS 1');
    emsTempChart.addSeries(StrokePresets.theoretical('EMS 2'));
    $chartStore.set('EMS Temperatures', emsTempChart);

    let airGapChart = new PlotBuffer(
        500,
        60000,
        [0, 30],
        true,
        'Vertical Air Gap'
    );
    airGapChart.addSeries(StrokePresets.theoretical('Lateral Air Gap'));
    $chartStore.set('Air Gaps', airGapChart);
    // max vertical: 27.5
    // max lateral: 16

    let rotationChart = new PlotBuffer(500, 60000, [0, 120], true, 'Pitch');
    rotationChart.addSeries(StrokePresets.theoretical('Roll'));
    $chartStore.set('Rotations', rotationChart);
    // TODO: max rotations?

    let hemsTempChart = new PlotBuffer(500, 60000, [0, 120], true, 'HEMS 1');

    hemsTempChart.addSeries(StrokePresets.theoretical('HEMS 2'));
    hemsTempChart.addSeries(StrokePresets.yellow('HEMS 3'));
    hemsTempChart.addSeries(StrokePresets.blue('HEMS 4'));
    $chartStore.set('HEMS Temperatures', hemsTempChart);

    let hemsCurrentChart = new PlotBuffer(
        500,
        3 * 60000,
        [-11.3, 11.3],
        true,
        'a1'
    );
    hemsCurrentChart.addSeries(StrokePresets.hyperloopGreenDashed('a2'));
    hemsCurrentChart.addSeries(StrokePresets.theoretical('b1'));
    hemsCurrentChart.addSeries(StrokePresets.theoreticalDashed('b2'));
    hemsCurrentChart.addSeries(StrokePresets.yellow('c1'));
    hemsCurrentChart.addSeries(StrokePresets.yellowDashed('c2'));
    hemsCurrentChart.addSeries(StrokePresets.blue('d1'));
    hemsCurrentChart.addSeries(StrokePresets.blueDashed('d2'));
    $chartStore.set('HEMS Current', hemsCurrentChart);

    let emsCurrentChart = new PlotBuffer(500, 3 * 60000, [-11.3, 11.3], true);
    emsCurrentChart.addSeries(StrokePresets.theoretical('cd'));
    $chartStore.set('EMS Current', emsCurrentChart);

    let voffChart = new PlotBuffer(500, 60000, [8, 25], false);
    $chartStore.set('Offset Vertical', voffChart);

    let accelChart = new PlotBuffer(500, 60000, [0, 25], false);
    $chartStore.set('Acceleration', accelChart);

    let rolPitchChart = new PlotBuffer(500, 60000, [-0.8, 0.8], true, 'roll');
    rolPitchChart.addSeries(StrokePresets.theoretical('pitch'));
    $chartStore.set('Roll Pitch', rolPitchChart);

    let hoffChart = new PlotBuffer(500, 60000, [-8, 8], true, 'ab');
    hoffChart.addSeries(StrokePresets.theoretical('cd'));
    $chartStore.set('Offset Horizontal', hoffChart);

    let velChart = new PlotBuffer(500, 60000, [0, 100], false);
    $chartStore.set('Velocity', velChart);

    let localizationChart = new PlotBuffer(500, 60000, [0, 13000], false);
    localizationChart.addSeries(StrokePresets.yellow("Localization"))
    $chartStore.set("Localization", localizationChart);

    let lvCurrent = new PlotBuffer(500, 60000, [-15, 15], false);
    $chartStore.set('LV Current', lvCurrent);

    let hvCurrent = new PlotBuffer(500, 60000, [-15, 15], false);
    $chartStore.set('HV Current', hvCurrent);

    let lvTotal = new PlotBuffer(500, 2 * 60000, [-1, 30], false);
    $chartStore.set('LV Total', lvTotal);

    let hvTotal = new PlotBuffer(500, 2 * 60000, [-10, 450], false);
    $chartStore.set('HV Total', hvTotal);

    let motorLeftTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Left 1');
    motorLeftTemp.addSeries(StrokePresets.yellow('Motor Left 2'));
    motorLeftTemp.addSeries(StrokePresets.blue('Motor Left 3'));
    motorLeftTemp.addSeries(StrokePresets.blueDashed('Motor Left 4'));
    motorLeftTemp.addSeries(StrokePresets.hyperLoopGreen('Motor Left 5'));
    motorLeftTemp.addSeries(StrokePresets.hyperloopGreenDashed('Motor Left 6'));
    $chartStore.set('Motor Temperatures Left', motorLeftTemp);

    let motorRightTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Right 1');
    motorRightTemp.addSeries(StrokePresets.yellow('Motor Right 2'));
    motorRightTemp.addSeries(StrokePresets.blue('Motor Right 3'));
    motorRightTemp.addSeries(StrokePresets.blueDashed('Motor Right 4'));
    motorRightTemp.addSeries(StrokePresets.hyperLoopGreen('Motor Right 5'));
    motorRightTemp.addSeries(StrokePresets.hyperloopGreenDashed('Motor Right 6'));
    motorRightTemp.addSeries(StrokePresets.yellowDashed('Motor Right 7'));
    $chartStore.set('Motor Temperatures Right', motorRightTemp);
    

    ////////////////////////////////////////////////////////////////

    let gdd = GrandDataDistributor.getInstance();

    ////////////////////////////////////////////////////////////////

    // Prop log 1 MD1 chart for test runs
    let propLog1MD1Chart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1MD1Chart.addSeries(StrokePresets.yellow('Id reference'));
    propLog1MD1Chart.addSeries(StrokePresets.blue('Iq measured'));
    propLog1MD1Chart.addSeries(StrokePresets.theoretical('Iq reference'));
    $chartStore.set('Propulsion Log 1 - MD1', propLog1MD1Chart);

    // Prop log 2 MD1 chart for test runs
    let propLog2MD1Chart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2MD1Chart.addSeries(StrokePresets.yellow('VD'));
    propLog2MD1Chart.addSeries(StrokePresets.blue('Vbus'));
    propLog2MD1Chart.addSeries(StrokePresets.theoretical('Ibus'));
    $chartStore.set('Propulsion Log 2 - MD1', propLog2MD1Chart);

    // Prop log 3 MD1 chart for test runs
    let propLog3MD1Chart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3MD1Chart.addSeries(StrokePresets.yellow('Tb'));
    propLog3MD1Chart.addSeries(StrokePresets.blue('Tc'));
    propLog3MD1Chart.addSeries(StrokePresets.theoretical('TCASE'));
    $chartStore.set('Propulsion Log 3 - MD1', propLog3MD1Chart);

    // Prop log 1 MD2 chart for test runs
    let propLog1MD2Chart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1MD2Chart.addSeries(StrokePresets.yellow('Id reference'));
    propLog1MD2Chart.addSeries(StrokePresets.blue('Iq measured'));
    propLog1MD2Chart.addSeries(StrokePresets.theoretical('Iq reference'));
    $chartStore.set('Propulsion Log 1 - MD2', propLog1MD2Chart);

    // Prop log 2 MD2 chart for test runs
    let propLog2MD2Chart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2MD2Chart.addSeries(StrokePresets.yellow('VD'));
    propLog2MD2Chart.addSeries(StrokePresets.blue('Vbus'));
    propLog2MD2Chart.addSeries(StrokePresets.theoretical('Ibus'));
    $chartStore.set('Propulsion Log 2 - MD2', propLog2MD2Chart);

    // Prop log 3 MD2 chart for test runs
    let propLog3MD2Chart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3MD2Chart.addSeries(StrokePresets.yellow('Tb'));
    propLog3MD2Chart.addSeries(StrokePresets.blue('Tc'));
    propLog3MD2Chart.addSeries(StrokePresets.theoretical('TCASE'));
    $chartStore.set('Propulsion Log 3 - MD2', propLog3MD2Chart);

    // generated

            // auto-generated with npm run generate:gs


    gdd.stores.registerStore<number>("TempMotorLeft0", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorLeft1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorLeft2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorLeft3", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorLeft4", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(5, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorLeft5", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(6, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorRight0", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorRight1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorRight2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorRight3", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorRight4", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(5, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorRight5", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(6, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorRight6", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(7, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("BMSVoltageHigh", 0);

    gdd.stores.registerStore<number>("BMSVoltageLow", 0);

    gdd.stores.registerStore<number>("BMSTemperatureHigh", 0);

    gdd.stores.registerStore<number>("BMSTemperatureLow", 0);

    gdd.stores.registerStore<number>("TempRangeStart", 0);

    gdd.stores.registerStore<number>("TempRangeEnd", 0);

    gdd.stores.registerStore<number>("Localization", 0, data => {
            const curr = Number(data);
            $chartStore.get("Localization")!.addEntry(1, curr);
            return curr;
        }
    );

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

    gdd.stores.registerStore<number>("PPInitFault1", 0);

    gdd.stores.registerStore<number>("PPInitFault2", 0);

    gdd.stores.registerStore<number>("PPEmergency1", 0);

    gdd.stores.registerStore<number>("PPEmergency2", 0);

    gdd.stores.registerStore<number>("Word1", 0);

    gdd.stores.registerStore<number>("Word2", 0);

    gdd.stores.registerStore<number>("IqMeasured1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD1")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IqReference1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD1")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdMeasured1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD1")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdReference1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD1")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IqMeasured2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD2")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IqReference2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD2")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdMeasured2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD2")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdReference2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - MD2")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vq_Log1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD1")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vd_Log1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD1")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vbus1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD1")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ibus1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD1")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("CANLog", 0);

    gdd.stores.registerStore<number>("Vq_Log2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD2")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vd_Log2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD2")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vbus2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD2")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ibus2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - MD2")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ta1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD1")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tb1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD1")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tc1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD1")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TCASE1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD1")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ta2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD2")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tb2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD2")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tc2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD2")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TCASE2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - MD2")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("FSMState", 0);


    // End of generated

    gdd.start(50);

    initializeStores();

    setInterval(() => {
        latestTimestamp.set(Date.now());
    }, 1000);

    onMount(() => {
        setInterval(() => {
            if ($showcasingStates) {
                showcaseStateCounter.set(($showcaseStateCounter + 1) % 14);
            }
        }, 1000);
    });

    onDestroy(async () => {
        GrandDataDistributor.getInstance().kill();
        (await unlisten)();
    });
</script>

<div class="flex flex-col w-screen h-screen max-h-screen overflow-hidden">
    <Toast />
    <Modal components={modalRegistry} />
    <TitleBar />
    <slot />
    <BottomBar />
</div>

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

    const unlisten = listen('shortcut_channel', (event: { payload: string }) =>
        parseShortCut(event.payload, $debugModeActive, $logsVisible)
    );

    //////////////////////////////
    /////////// CHARTS ///////////
    //////////////////////////////

    let offsetChart = new PlotBuffer(500, 60000, [0, 30], true, 'Offset 1');
    offsetChart.addSeries(StrokePresets.hyperLoopGreen('Offset 2'));
    offsetChart.addSeries(StrokePresets.yellow('Offset 3'));
    offsetChart.addSeries(StrokePresets.blue('Offset 4'));
    $chartStore.set('Offset', offsetChart);

    // Prop log 1 Right Motor chart for test runs
    let propLog1RightMotorChart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1RightMotorChart.addSeries(StrokePresets.yellow('Id reference'));
    propLog1RightMotorChart.addSeries(StrokePresets.blue('Iq measured'));
    propLog1RightMotorChart.addSeries(StrokePresets.theoretical('Iq reference'));
    $chartStore.set('Propulsion Log 1 - Right Motor', propLog1RightMotorChart);

    // Prop log 2 Right Motor chart for test runs
    let propLog2RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2RightMotorChart.addSeries(StrokePresets.yellow('VD'));
    propLog2RightMotorChart.addSeries(StrokePresets.blue('Vbus'));
    propLog2RightMotorChart.addSeries(StrokePresets.theoretical('Ibus'));
    $chartStore.set('Propulsion Log 2 - Right Motor', propLog2RightMotorChart);

    // Prop log 3 Right Motor chart for test runs
    let propLog3RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3RightMotorChart.addSeries(StrokePresets.yellow('Tb'));
    propLog3RightMotorChart.addSeries(StrokePresets.blue('Tc'));
    propLog3RightMotorChart.addSeries(StrokePresets.theoretical('TCASE'));
    $chartStore.set('Propulsion Log 3 - Right Motor', propLog3RightMotorChart);

    // Prop log 1 Left Motor chart for test runs
    let propLog1LeftMotorChart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1LeftMotorChart.addSeries(StrokePresets.yellow('Id reference'));
    propLog1LeftMotorChart.addSeries(StrokePresets.blue('Iq measured'));
    propLog1LeftMotorChart.addSeries(StrokePresets.theoretical('Iq reference'));
    $chartStore.set('Propulsion Log 1 - Left Motor', propLog1LeftMotorChart);

    // Prop log 2 Left Motor chart for test runs
    let propLog2LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2LeftMotorChart.addSeries(StrokePresets.yellow('VD'));
    propLog2LeftMotorChart.addSeries(StrokePresets.blue('Vbus'));
    propLog2LeftMotorChart.addSeries(StrokePresets.theoretical('Ibus'));
    $chartStore.set('Propulsion Log 2 - Left Motor', propLog2LeftMotorChart);

    // Prop log 3 Left Motor chart for test runs
    let propLog3LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3LeftMotorChart.addSeries(StrokePresets.yellow('Tb'));
    propLog3LeftMotorChart.addSeries(StrokePresets.blue('Tc'));
    propLog3LeftMotorChart.addSeries(StrokePresets.theoretical('TCASE'));
    $chartStore.set('Propulsion Log 3 - Left Motor', propLog3LeftMotorChart);

    let motorLeftTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Left 1');
    motorLeftTemp.addSeries(StrokePresets.yellow('Motor Left 2'));
    motorLeftTemp.addSeries(StrokePresets.blue('Motor Left 3'));
    motorLeftTemp.addSeries(StrokePresets.blueDashed('Motor Left 4'));
    motorLeftTemp.addSeries(StrokePresets.hyperLoopGreen('Motor Left 5'));
    motorLeftTemp.addSeries(StrokePresets.hyperloopGreenDashed('Motor Left 6'));
    motorLeftTemp.addSeries(StrokePresets.yellowDashed('Motor Left 7'));
    motorLeftTemp.addSeries(StrokePresets.theoretical('Motor Left 8'))
    $chartStore.set('Motor Temperatures Left', motorLeftTemp);

    let motorRightTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Right 1');
    motorRightTemp.addSeries(StrokePresets.yellow('Motor Right 2'));
    motorRightTemp.addSeries(StrokePresets.blue('Motor Right 3'));
    motorRightTemp.addSeries(StrokePresets.blueDashed('Motor Right 4'));
    motorRightTemp.addSeries(StrokePresets.hyperLoopGreen('Motor Right 5'));
    motorRightTemp.addSeries(StrokePresets.hyperloopGreenDashed('Motor Right 6'));
    motorRightTemp.addSeries(StrokePresets.yellowDashed('Motor Right 7'));
    motorRightTemp.addSeries(StrokePresets.theoretical('Motor Right 8'));
    $chartStore.set('Motor Temperatures Right', motorRightTemp);

    let airGapChart = new PlotBuffer(
        500,
        60000,
        [0, 30],
        true,
        'Vertical Air Gap'
    );
    airGapChart.addSeries(StrokePresets.theoretical('Lateral Air Gap'));
    $chartStore.set('Air Gaps', airGapChart);

    let anglesChart = new PlotBuffer(500, 60000, [0, 120], true, 'Roll');
    anglesChart.addSeries(StrokePresets.theoretical('Pitch'));
    anglesChart.addSeries(StrokePresets.blue('Yaw'));
    $chartStore.set('Angles', anglesChart);

    let hemsCurrentChart = new PlotBuffer(
        500,
        3 * 60000,
        [-11.3, 11.3],
        true,
        'VFL'
    );
    hemsCurrentChart.addSeries(StrokePresets.blue('VFR'));
    hemsCurrentChart.addSeries(StrokePresets.theoretical('VBL'));
    hemsCurrentChart.addSeries(StrokePresets.yellow('VBR'));
    $chartStore.set('HEMS Current', hemsCurrentChart);

    let emsCurrentChart = new PlotBuffer(500, 3 * 60000, [-11.3, 11.3], true, 'LF');
    emsCurrentChart.addSeries(StrokePresets.theoretical('LB'));
    $chartStore.set('EMS Current', emsCurrentChart);

    let accelChart = new PlotBuffer(500, 60000, [0, 25], false);
    $chartStore.set('Acceleration', accelChart);

    let velChart = new PlotBuffer(500, 60000, [0, 100], false);
    $chartStore.set('Velocity', velChart);

    let localizationChart = new PlotBuffer(500, 60000, [0, 13000], false);
    localizationChart.addSeries(StrokePresets.yellow("Localization"))
    $chartStore.set("Localization", localizationChart);

    let leviRequestForce1Chart = new PlotBuffer(500, 60000, [0, 100], true, 'Z');
    leviRequestForce1Chart.addSeries(StrokePresets.yellow('Roll'));
    leviRequestForce1Chart.addSeries(StrokePresets.theoretical('Pitch'));
    $chartStore.set('Requested Force 1', leviRequestForce1Chart);

    let leviRequestForce2Chart = new PlotBuffer(500, 60000, [0, 100], true, 'Y');
    leviRequestForce2Chart.addSeries(StrokePresets.yellow('Yaw'));
    $chartStore.set('Requested Force 2', leviRequestForce2Chart);



    ////////////////////////////////////////////////////////////////

    let gdd = GrandDataDistributor.getInstance();

    ////////////////////////////////////////////////////////////////

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

    gdd.stores.registerStore<number>("PTCState", 0);

    gdd.stores.registerStore<number>("PTCNonCriticalFault", 0);

    gdd.stores.registerStore<number>("BMSVoltageHigh", 0);

    gdd.stores.registerStore<number>("BMSVoltageLow", 0);

    gdd.stores.registerStore<number>("BMSTemperatureHigh", 0);

    gdd.stores.registerStore<number>("BMSTemperatureLow", 0);

    gdd.stores.registerStore<number>("VPack", 0);

    gdd.stores.registerStore<number>("IPack", 0);

    gdd.stores.registerStore<number>("VDCLink", 0);

    gdd.stores.registerStore<number>("TempRangeStart", 0);

    gdd.stores.registerStore<number>("TempRangeEnd", 0);

    gdd.stores.registerStore<number>("Localization", 0, data => {
            const curr = Number(data);
            $chartStore.get("Localization")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Offset1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Offset")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Offset2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Offset")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Offset3", 0, data => {
            const curr = Number(data);
            $chartStore.get("Offset")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Offset4", 0, data => {
            const curr = Number(data);
            $chartStore.get("Offset")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("PPInitFault1", 256);

    gdd.stores.registerStore<number>("PPInitFault2", 256);

    gdd.stores.registerStore<number>("PPEmergency1", 0);

    gdd.stores.registerStore<number>("PPEmergency2", 0);

    gdd.stores.registerStore<number>("Word1", 0);

    gdd.stores.registerStore<number>("Word2", 0);

    gdd.stores.registerStore<number>("IqMeasured1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IqReference1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdMeasured1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdReference1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Right Motor")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IqMeasured2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IqReference2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdMeasured2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IdReference2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 1 - Left Motor")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vq_Log1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vd_Log1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vbus1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ibus1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Right Motor")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("CANLog", 0);

    gdd.stores.registerStore<number>("Vq_Log2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vd_Log2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Vbus2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ibus2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 2 - Left Motor")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ta1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tb1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tc1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TCASE1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Right Motor")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Ta2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tb2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Tc2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TCASE2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Propulsion Log 3 - Left Motor")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("FSMAckProp1", 0);

    gdd.stores.registerStore<number>("FSMAckProp2", 0);

    gdd.stores.registerStore<number>("Velocity", 0, data => {
            const curr = Number(data);
            $chartStore.get("Velocity")!.addEntry(1, curr);
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
        }, 500);
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

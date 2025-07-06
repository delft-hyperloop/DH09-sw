<script lang="ts">
    import '../app.postcss';
    import {
        BottomBar,
        EventChannel,
        GrandDataDistributor,
        PlotBuffer,
        StrokePresets,
        TitleBar,
        util
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
        inDropdown,
        latestTimestamp,
        leviChartStore,
        logsVisible,
        powertrainChartStore,
        propChartStore,
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
    import { invoke } from '@tauri-apps/api/tauri';
    import { registerSubscribers } from '$lib/util/subscribers';

    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    let propCharts: string[] = [];
    let leviCharts: string[] = [];
    let powertrainCharts: string[] = [];

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

    // Prop log 1 Left Motor chart for test runs
    let propLog1LeftMotorChart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1LeftMotorChart.addSeries({ label: 'Id reference', spanGaps: false, stroke: '#0074D9' }); // blue
    propLog1LeftMotorChart.addSeries({ label: 'Iq measured', spanGaps: false, stroke: '#ffa64d' }); 
    propLog1LeftMotorChart.addSeries({ label: 'Iq reference', spanGaps: false, stroke: '#B10DC9' }); // purple
    $chartStore.set('Propulsion Log 1 - Left Motor', propLog1LeftMotorChart);
    propCharts.push('Propulsion Log 1 - Left Motor');

    // Prop log 1 Right Motor chart for test runs
    let propLog1RightMotorChart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1RightMotorChart.addSeries({ label: 'Id reference', spanGaps: false, stroke: '#0074D9' }); // blue
    propLog1RightMotorChart.addSeries({ label: 'Iq measured', spanGaps: false, stroke: '#ffa64d' }); // green
    propLog1RightMotorChart.addSeries({ label: 'Iq reference', spanGaps: false, stroke: '#B10DC9' }); // purple
    $chartStore.set('Propulsion Log 1 - Right Motor', propLog1RightMotorChart);
    propCharts.push('Propulsion Log 1 - Right Motor');

    // Prop log 2 Left Motor chart for test runs
    let propLog2LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2LeftMotorChart.addSeries({ label: 'VD', spanGaps: false, stroke: '#0074D9' }); // blue
    propLog2LeftMotorChart.addSeries({ label: 'Vbus', spanGaps: false, stroke: '#ffa64d' }); // green
    propLog2LeftMotorChart.addSeries({ label: 'Ibus', spanGaps: false, stroke: '#B10DC9' }); // purple
    $chartStore.set('Propulsion Log 2 - Left Motor', propLog2LeftMotorChart);
    propCharts.push('Propulsion Log 2 - Left Motor');

    // Prop log 2 Right Motor chart for test runs
    let propLog2RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2RightMotorChart.addSeries({ label: 'VD', spanGaps: false, stroke: '#0074D9' }); // blue
    propLog2RightMotorChart.addSeries({ label: 'Vbus', spanGaps: false, stroke: '#ffa64d' }); // green
    propLog2RightMotorChart.addSeries({ label: 'Ibus', spanGaps: false, stroke: '#B10DC9' }); // purple
    $chartStore.set('Propulsion Log 2 - Right Motor', propLog2RightMotorChart);
    propCharts.push('Propulsion Log 2 - Right Motor');

    // Prop log 3 Left Motor chart for test runs
    let propLog3LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3LeftMotorChart.addSeries({ label: 'Tb', spanGaps: false, stroke: '#0074D9' }); // blue
    propLog3LeftMotorChart.addSeries({ label: 'Tc', spanGaps: false, stroke: '#ffa64d' }); // green
    propLog3LeftMotorChart.addSeries({ label: 'TCASE', spanGaps: false, stroke: '#B10DC9' }); // purple
    $chartStore.set('Propulsion Log 3 - Left Motor', propLog3LeftMotorChart);
    propCharts.push('Propulsion Log 3 - Left Motor');

    // Prop log 3 Right Motor chart for test runs
    let propLog3RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3RightMotorChart.addSeries({ label: 'Tb', spanGaps: false, stroke: '#0074D9' }); // blue
    propLog3RightMotorChart.addSeries({ label: 'Tc', spanGaps: false, stroke: '#ffa64d' }); // green
    propLog3RightMotorChart.addSeries({ label: 'TCASE', spanGaps: false, stroke: '#B10DC9' }); // purple
    $chartStore.set('Propulsion Log 3 - Right Motor', propLog3RightMotorChart);
    propCharts.push('Propulsion Log 3 - Right Motor');

    let motorLeftTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Left 1');
    motorLeftTemp.addSeries({ label: 'Motor Left 2', spanGaps: false, stroke: '#0074D9' }); // blue
    motorLeftTemp.addSeries({ label: 'Motor Left 3', spanGaps: false, stroke: '#ffa64d' }); // green
    motorLeftTemp.addSeries({ label: 'Motor Left 4', spanGaps: false, stroke: '#B10DC9' }); // purple
    motorLeftTemp.addSeries({ label: 'Motor Left 5', spanGaps: false, stroke: '#39CCCC' }); // teal
    motorLeftTemp.addSeries({ label: 'Motor Left 6', spanGaps: false, stroke: '#FF851B' }); // orange
    motorLeftTemp.addSeries({ label: 'Motor Left 7', spanGaps: false, stroke: '#F012BE' }); // pink
    motorLeftTemp.addSeries({ label: 'Motor Left 8', spanGaps: false, stroke: '#7FDBFF' }); // cyan

    // Motor Left Temperature safety lines
    const motorLeftTempMin = 0, motorLeftTempMax = 80;
    motorLeftTemp.addSeries({
        label: "Min Safe (0°C)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    motorLeftTemp.addSeries({
        label: "Max Safe (80°C)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    $chartStore.set('Motor Temperatures Left', motorLeftTemp);
    propCharts.push('Motor Temperatures Left');
    const xLenMotorLeft = motorLeftTemp.getSeriesData(0).length;
    motorLeftTemp.updateSeries(9, new Float32Array(xLenMotorLeft).fill(motorLeftTempMin)); // Min Safe (yellow)
    motorLeftTemp.updateSeries(10, new Float32Array(xLenMotorLeft).fill(motorLeftTempMax)); // Max Safe (red)

    let motorRightTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Right 1');
    motorRightTemp.addSeries({ label: 'Motor Right 2', spanGaps: false, stroke: '#0074D9' }); // blue
    motorRightTemp.addSeries({ label: 'Motor Right 3', spanGaps: false, stroke: '#ffa64d' }); // green
    motorRightTemp.addSeries({ label: 'Motor Right 4', spanGaps: false, stroke: '#B10DC9' }); // purple
    motorRightTemp.addSeries({ label: 'Motor Right 5', spanGaps: false, stroke: '#39CCCC' }); // teal
    motorRightTemp.addSeries({ label: 'Motor Right 6', spanGaps: false, stroke: '#FF851B' }); // orange
    motorRightTemp.addSeries({ label: 'Motor Right 7', spanGaps: false, stroke: '#F012BE' }); // pink
    motorRightTemp.addSeries({ label: 'Motor Right 8', spanGaps: false, stroke: '#7FDBFF' }); // cyan

    // Motor Right Temperature safety lines
    const motorRightTempMin = 0, motorRightTempMax = 80;
    motorRightTemp.addSeries({
        label: "Min Safe (0°C)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    motorRightTemp.addSeries({
        label: "Max Safe (80°C)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    $chartStore.set('Motor Temperatures Right', motorRightTemp);
    propCharts.push('Motor Temperatures Right');
    const xLenMotorRight = motorRightTemp.getSeriesData(0).length;
    motorRightTemp.updateSeries(9, new Float32Array(xLenMotorRight).fill(motorRightTempMin)); // Min Safe (yellow)
    motorRightTemp.updateSeries(10, new Float32Array(xLenMotorRight).fill(motorRightTempMax)); // Max Safe (red)

    let offsetChart = new PlotBuffer(500, 60000, [0, 30], true, 'Offset 1');
    offsetChart.addSeries({ label: 'Offset 2', spanGaps: false, stroke: '#0074D9' }); // blue
    offsetChart.addSeries({ label: 'Offset 3', spanGaps: false, stroke: '#ffa64d' }); // green
    offsetChart.addSeries({ label: 'Offset 4', spanGaps: false, stroke: '#B10DC9' }); // purple
    $chartStore.set('Offset', offsetChart);
    propCharts.push('Offset');

    let airGapChart = new PlotBuffer(
        500,
        60000,
        [0, 30],
        true,
        'Vertical Air Gap'
    );
    airGapChart.addSeries({ label: 'Lateral Air Gap', spanGaps: false, stroke: '#0074D9' });
    $chartStore.set('Air Gaps', airGapChart);
    leviCharts.push('Air Gaps');

    // Air Gaps chart safety lines
    const airGapMin = 2, airGapMax = 27;
    airGapChart.addSeries({
        label: "Min Safe (2mm)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    airGapChart.addSeries({
        label: "Max Safe (27mm)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    const xLenAirGaps = airGapChart.getSeriesData(0).length;
    airGapChart.updateSeries(3, new Float32Array(xLenAirGaps).fill(airGapMin)); // Min Safe (yellow)
    airGapChart.updateSeries(4, new Float32Array(xLenAirGaps).fill(airGapMax)); // Max Safe (red)

    // Angles chart: now in degrees, yRange [-0.4, 0.4] deg, with min/max safety lines
    let anglesChart = new PlotBuffer(500, 60000, [-0.4, 0.4], true, 'Roll');
    anglesChart.addSeries({ label: 'Pitch', spanGaps: false, stroke: '#4d4dff' });
    anglesChart.addSeries({ label: 'Yaw', spanGaps: false, stroke: '#B10DC9' });
    $chartStore.set('Angles', anglesChart);
    leviCharts.push('Angles');
    // Add min/max safety lines at +/- 0.4 deg
    anglesChart.addSeries({
        label: 'Min Safe (-0.4°)',
        stroke: '#ffde0a', // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    anglesChart.addSeries({
        label: 'Max Safe (0.4°)',
        stroke: '#ff0a43', // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    const xLenAngles = anglesChart.getSeriesData(0).length;
    anglesChart.updateSeries(4, new Float32Array(xLenAngles).fill(-0.4)); // Min Safe (yellow)
    anglesChart.updateSeries(5, new Float32Array(xLenAngles).fill(0.4)); // Max Safe (red)

    let hemsCurrentChart = new PlotBuffer(
        500,
        3 * 60000,
        [-13, 13],
        true,
        'VFL1'
    );
    hemsCurrentChart.addSeries({ label: 'VFL2', spanGaps: false, stroke: '#0074D9' });
    hemsCurrentChart.addSeries({ label: 'VFR1', spanGaps: false, stroke: '#ffa64d' });
    hemsCurrentChart.addSeries({ label: 'VFR2', spanGaps: false, stroke: '#B10DC9' });
    hemsCurrentChart.addSeries({ label: 'VBL1', spanGaps: false, stroke: '#39CCCC' });
    hemsCurrentChart.addSeries({ label: 'VBL2', spanGaps: false, stroke: '#FF851B' });
    hemsCurrentChart.addSeries({ label: 'VBR1', spanGaps: false, stroke: '#F012BE' });
    hemsCurrentChart.addSeries({ label: 'VBR2', spanGaps: false, stroke: '#7FDBFF' });
    $chartStore.set('HEMS Current', hemsCurrentChart);
    leviCharts.push('HEMS Current');

    // HEMS Current chart safety lines
    const hemsCurrentMin = -13, hemsCurrentMax = 13;
    hemsCurrentChart.addSeries({
        label: "Min Safe (-13A)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    hemsCurrentChart.addSeries({
        label: "Max Safe (13A)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    const xLenHems = hemsCurrentChart.getSeriesData(0).length;
    hemsCurrentChart.updateSeries(9, new Float32Array(xLenHems).fill(hemsCurrentMin)); // Min Safe (yellow)
    hemsCurrentChart.updateSeries(10, new Float32Array(xLenHems).fill(hemsCurrentMax)); // Max Safe (red)

    let emsCurrentChart = new PlotBuffer(500, 3 * 60000, [-13, 13], true, 'LF');
    emsCurrentChart.addSeries({ label: 'LB', spanGaps: false, stroke: '#4d4dff' });
    $chartStore.set('EMS Current', emsCurrentChart);
    leviCharts.push('EMS Current');

    // EMS Current chart safety lines
    const emsCurrentMin = -13, emsCurrentMax = 13;
    emsCurrentChart.addSeries({
        label: "Min Safe (-13A)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    emsCurrentChart.addSeries({
        label: "Max Safe (13A)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    const xLenEms = emsCurrentChart.getSeriesData(0).length;
    emsCurrentChart.updateSeries(3, new Float32Array(xLenEms).fill(emsCurrentMin)); // Min Safe (yellow)
    emsCurrentChart.updateSeries(4, new Float32Array(xLenEms).fill(emsCurrentMax)); // Max Safe (red)

    let accelChart = new PlotBuffer(500, 60000, [0, 25], false);
    $chartStore.set('Acceleration', accelChart);
    propCharts.push('Acceleration');

    let velChart = new PlotBuffer(500, 60000, [0, 100], false);
    $chartStore.set('Velocity', velChart);
    propCharts.push('Velocity');

    let localizationChart = new PlotBuffer(500, 60000, [0, 13000], false);
    localizationChart.addSeries(StrokePresets.yellow("Localization"))
    $chartStore.set("Localization", localizationChart);

    let leviRequestForceVerticalChart = new PlotBuffer(500, 60000, [0, 100], true, 'Z');
    leviRequestForceVerticalChart.addSeries({ label: 'Roll', spanGaps: false, stroke: '#ffa64d' }); // green
    leviRequestForceVerticalChart.addSeries({ label: 'Pitch', spanGaps: false, stroke: '#ffde0a' }); // yellow
    $chartStore.set('Requested Force Vertical', leviRequestForceVerticalChart);
    leviCharts.push('Requested Force Vertical');

    let leviRequestForceHorizontalChart = new PlotBuffer(500, 60000, [0, 100], true, 'Y');
    leviRequestForceHorizontalChart.addSeries({ label: 'Yaw', spanGaps: false, stroke: '#ff0a43' }); // red
    $chartStore.set('Requested Force Horizontal', leviRequestForceHorizontalChart);
    leviCharts.push('Requested Force Horizontal');

    let hemsTempChart = new PlotBuffer(500, 60000, [0, 100], true, 'L1');
    hemsTempChart.addSeries({ label: 'L2', spanGaps: false, stroke: '#0074D9' });
    hemsTempChart.addSeries({ label: 'L3', spanGaps: false, stroke: '#ffa64d' });
    hemsTempChart.addSeries({ label: 'L4', spanGaps: false, stroke: '#B10DC9' });
    hemsTempChart.addSeries({ label: 'R1', spanGaps: false, stroke: '#39CCCC' });
    hemsTempChart.addSeries({ label: 'R2', spanGaps: false, stroke: '#FF851B' });
    hemsTempChart.addSeries({ label: 'R3', spanGaps: false, stroke: '#F012BE' });
    hemsTempChart.addSeries({ label: 'R4', spanGaps: false, stroke: '#7FDBFF' });
    $chartStore.set('Temperatures HEMS', hemsTempChart);
    leviCharts.push('Temperatures HEMS');

    // Temperatures HEMS chart safety lines
    const hemsTempMin = 0, hemsTempMax = 80;
    hemsTempChart.addSeries({
        label: "Min Safe (0°C)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    hemsTempChart.addSeries({
        label: "Max Safe (80°C)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    const xLenHemsTemp = hemsTempChart.getSeriesData(0).length;
    hemsTempChart.updateSeries(9, new Float32Array(xLenHemsTemp).fill(hemsTempMin)); // Min Safe (yellow)
    hemsTempChart.updateSeries(10, new Float32Array(xLenHemsTemp).fill(hemsTempMax)); // Max Safe (red)

    let emsTempChart = new PlotBuffer(500, 60000, [0, 100], true, 'L1');
    emsTempChart.addSeries({ label: 'L2', spanGaps: false, stroke: '#0074D9' });
    emsTempChart.addSeries({ label: 'L3', spanGaps: false, stroke: '#ffa64d' });
    emsTempChart.addSeries({ label: 'L4', spanGaps: false, stroke: '#B10DC9' });
    emsTempChart.addSeries({ label: 'R1', spanGaps: false, stroke: '#39CCCC' });
    emsTempChart.addSeries({ label: 'R2', spanGaps: false, stroke: '#FF851B' });
    emsTempChart.addSeries({ label: 'R3', spanGaps: false, stroke: '#F012BE' });
    emsTempChart.addSeries({ label: 'R4', spanGaps: false, stroke: '#7FDBFF' });
    $chartStore.set('Temperatures EMS', emsTempChart);
    leviCharts.push('Temperatures EMS');

    // Temperatures EMS chart safety lines
    const emsTempMin = 0, emsTempMax = 80;
    emsTempChart.addSeries({
        label: "Min Safe (0°C)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    emsTempChart.addSeries({
        label: "Max Safe (80°C)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    const xLenEmsTemp = emsTempChart.getSeriesData(0).length;
    emsTempChart.updateSeries(9, new Float32Array(xLenEmsTemp).fill(emsTempMin)); // Min Safe (yellow)
    emsTempChart.updateSeries(10, new Float32Array(xLenEmsTemp).fill(emsTempMax)); // Max Safe (red)

    let BMSVoltageHighChart = new PlotBuffer(500, 60000, [0, 700], true, "BMS Voltage High");
    const bmsHighMin = 400, bmsHighMax = 600;
    BMSVoltageHighChart.addSeries({
        label: "Min Safe (400V)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    BMSVoltageHighChart.addSeries({
        label: "Max Safe (600V)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    $chartStore.set("BMS Voltage High", BMSVoltageHighChart);
    powertrainCharts.push("BMS Voltage High");
    const xLenHigh = BMSVoltageHighChart.getSeriesData(0).length;
    BMSVoltageHighChart.updateSeries(2, new Float32Array(xLenHigh).fill(bmsHighMin)); // Min Safe (yellow)
    BMSVoltageHighChart.updateSeries(3, new Float32Array(xLenHigh).fill(bmsHighMax)); // Max Safe (red)

    let BMSVoltageLowChart = new PlotBuffer(500, 60000, [0, 50], true, "BMS Voltage Low");
    const bmsLowMin = 20, bmsLowMax = 30;
    BMSVoltageLowChart.addSeries({
        label: "Min Safe (20V)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    BMSVoltageLowChart.addSeries({
        label: "Max Safe (30V)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    $chartStore.set("BMS Voltage Low", BMSVoltageLowChart);
    powertrainCharts.push("BMS Voltage Low");
    const xLenLow = BMSVoltageLowChart.getSeriesData(0).length;
    BMSVoltageLowChart.updateSeries(2, new Float32Array(xLenLow).fill(bmsLowMin)); // Min Safe (yellow)
    BMSVoltageLowChart.updateSeries(3, new Float32Array(xLenLow).fill(bmsLowMax)); // Max Safe (red)

    let BMSVoltageTempsChart = new PlotBuffer(500, 60000, [0, 100], true, "Temp High");
    const tempMin = 0, tempMax = 80;
    BMSVoltageTempsChart.addSeries(StrokePresets.blue("Temp Low"));
    BMSVoltageTempsChart.addSeries({
        label: "Min Safe (0°C)",
        stroke: "#ffde0a", // yellow
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    BMSVoltageTempsChart.addSeries({
        label: "Max Safe (80°C)",
        stroke: "#ff0a43", // red
        dash: [6, 6],
        spanGaps: true,
        show: true
    });
    $chartStore.set("BMS Temps", BMSVoltageTempsChart);
    powertrainCharts.push("BMS Temps");
    const xLenTemps = BMSVoltageTempsChart.getSeriesData(0).length;
    BMSVoltageTempsChart.updateSeries(3, new Float32Array(xLenTemps).fill(tempMin)); // Min Safe (yellow)
    BMSVoltageTempsChart.updateSeries(4, new Float32Array(xLenTemps).fill(tempMax)); // Max Safe (red)

    leviChartStore.set(leviCharts);
    propChartStore.set(propCharts);
    powertrainChartStore.set(powertrainCharts);

    ////////////////////////////////////////////////////////////////

    let gdd = GrandDataDistributor.getInstance();

    ////////////////////////////////////////////////////////////////

    // gdd stores registration
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

    gdd.stores.registerStore<number>("TempMotorLeft6", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(7, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempMotorLeft7", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Left")!.addEntry(8, curr);
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

    gdd.stores.registerStore<number>("TempMotorRight7", 0, data => {
            const curr = Number(data);
            $chartStore.get("Motor Temperatures Right")!.addEntry(8, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("PTCState", 0);

    gdd.stores.registerStore<number>("HVALState", 0);

    gdd.stores.registerStore<number>("IMDWarnings", 0);

    gdd.stores.registerStore<number>("PTCErrors", 0);

    gdd.stores.registerStore<number>("BMSVoltageHigh", 0, data => {
            const curr = Number(data);
            $chartStore.get("BMS Voltage High")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("BMSVoltageLow", 0, data => {
            const curr = Number(data);
            $chartStore.get("BMS Voltage Low")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("BMSTemperatureHigh", 0, data => {
            const curr = Number(data);
            $chartStore.get("BMS Temps")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("BMSTemperatureLow", 0, data => {
            const curr = Number(data);
            $chartStore.get("BMS Temps")!.addEntry(2, curr);
            return curr;
        }
    );

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

    gdd.stores.registerStore<number>("Velocity", 0, data => {
            const curr = Number(data);
            $chartStore.get("Velocity")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("PPInitFault1", 255);

    gdd.stores.registerStore<number>("PPInitFault2", 255);

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

    gdd.stores.registerStore<number>("FSMAckLevi", 0);

    gdd.stores.registerStore<number>("ClearFaultAckLevi", 0);

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

    gdd.stores.registerStore<number>("LeviSystemCheckResponse", 0);

    gdd.stores.registerStore<number>("VHigh", 0);

    gdd.stores.registerStore<number>("VLow", 0);

    gdd.stores.registerStore<number>("THigh", 0);

    gdd.stores.registerStore<number>("TLow", 0);

    gdd.stores.registerStore<number>("VPackLowVoltage", 0);

    gdd.stores.registerStore<number>("IPackLowVoltage", 0);

    gdd.stores.registerStore<number>("PtSystemCheckResponse", 0);

    gdd.stores.registerStore<number>("LeviFault", 0);

    gdd.stores.registerStore<number>("LeviHeartbeat", 0);

    gdd.stores.registerStore<number>("LeviFSMStateChanged", 0);

    gdd.stores.registerStore<number>("LevitationState", 0);

    gdd.stores.registerStore<number>("NonCriticalLeviError", 0);

    gdd.stores.registerStore<number>("Vertical", 0, data => {
            const curr = Number(data);
            $chartStore.get("Air Gaps")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Lateral", 0, data => {
            const curr = Number(data);
            $chartStore.get("Air Gaps")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Roll", 0, data => {
        const curr = Number(data) * 180 / Math.PI;
        $chartStore.get("Angles")!.addEntry(1, curr);
        return curr;
    });

    gdd.stores.registerStore<number>("Pitch", 0, data => {
        const curr = Number(data) * 180 / Math.PI;
        $chartStore.get("Angles")!.addEntry(2, curr);
        return curr;
    });

    gdd.stores.registerStore<number>("Yaw", 0, data => {
        const curr = Number(data) * 180 / Math.PI;
        $chartStore.get("Angles")!.addEntry(3, curr);
        return curr;
    });

    gdd.stores.registerStore<number>("VFL1", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VFL2", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VFR1", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VFR2", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VBL1", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(5, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VBL2", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(6, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VBR1", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(7, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VBR2", 0, data => {
            const curr = Number(data);
            $chartStore.get("HEMS Current")!.addEntry(8, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("LF1", 0, data => {
            const curr = Number(data);
            $chartStore.get("EMS Current")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("LF2", 0, data => {
            const curr = Number(data);
            $chartStore.get("EMS Current")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("LB1", 0, data => {
            const curr = Number(data);
            $chartStore.get("EMS Current")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("LB2", 0, data => {
            const curr = Number(data);
            $chartStore.get("EMS Current")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("ZRequested", 0, data => {
            const curr = Number(data);
            $chartStore.get("Requested Force Vertical")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("RollRequested", 0, data => {
            const curr = Number(data);
            $chartStore.get("Requested Force Vertical")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("PitchRequested", 0, data => {
            const curr = Number(data);
            $chartStore.get("Requested Force Vertical")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("YRequested", 0, data => {
            const curr = Number(data);
            $chartStore.get("Requested Force Horizontal")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("YawRequested", 0, data => {
            const curr = Number(data);
            $chartStore.get("Requested Force Horizontal")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS3", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS4", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS5", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(5, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS6", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(6, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS7", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(7, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempHEMS8", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures HEMS")!.addEntry(8, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS1", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS2", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS3", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(3, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS4", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(4, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS5", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(5, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS6", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(6, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS7", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(7, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("TempEMS8", 0, data => {
            const curr = Number(data);
            $chartStore.get("Temperatures EMS")!.addEntry(8, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("FSMState", 0);

    gdd.stores.registerStore<number>("FSMTransitionFail", 100);

    gdd.stores.registerStore<number>("Emergency", 0);

    gdd.stores.registerStore<number>("LeviSystemCheckSuccess", 0);

    gdd.stores.registerStore<number>("LeviSystemCheckFailure", 0);

    gdd.stores.registerStore<number>("Prop1SystemCheckSuccess", 0);

    gdd.stores.registerStore<number>("Prop1SystemCheckFailure", 0);

    gdd.stores.registerStore<number>("Prop2SystemCheckSuccess", 0);

    gdd.stores.registerStore<number>("Prop2SystemCheckFailure", 0);

    // End of generated

    gdd.stores.registerStore<number>("FrontendHeartbeating", 0);

    gdd.start(50);

    initializeStores();

    setInterval(() => {
        latestTimestamp.set(Date.now());
    }, 1000);

    registerSubscribers();

    onMount(() => {
        setInterval(async () => {
            if ($showcasingStates) {
                showcaseStateCounter.set(($showcaseStateCounter + 1) % 14);
            }
            if ($inDropdown) {
                let val = 0;
                console.log(`Sending command: LeviDropdown`);
                await invoke('send_command', {cmdName: "LeviDropdown", val}).then(() => {
                    console.log(`Command LeviDropdown sent`);
                }).catch((e) => {
                    console.error(`Error sending command LeviDropdown: ${e}`);
                });
                util.log(`Command LeviDropdown sent`, EventChannel.INFO);
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

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
        GreenHVALTurnedOn,
        RedHVALTurnedOn
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

    // Prop log 1 Left Motor
    let propLog1LeftMotorChart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1LeftMotorChart.addSeries({ label: 'Id reference', spanGaps: false, stroke: '#0a85ff' }); // blue
    propLog1LeftMotorChart.addSeries({ label: 'Iq measured', spanGaps: false, stroke: '#a259f7' }); // purple
    propLog1LeftMotorChart.addSeries({ label: 'Iq reference', spanGaps: false, stroke: '#f78c25' }); // orange
    // Min/Max
    const propLog1Min = 0, propLog1Max = 20;
    propLog1LeftMotorChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => propLog1Min });
    propLog1LeftMotorChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => propLog1Max });
    $chartStore.set('Propulsion Log 1 - Left Motor', propLog1LeftMotorChart);
    propCharts.push('Propulsion Log 1 - Left Motor');
    const xLenPropLog1Left = propLog1LeftMotorChart.getSeriesData(0).length;
    propLog1LeftMotorChart.updateSeries(5, new Float32Array(xLenPropLog1Left).fill(propLog1Min));
    propLog1LeftMotorChart.updateSeries(6, new Float32Array(xLenPropLog1Left).fill(propLog1Max));

    // Prop log 1 Right Motor
    let propLog1RightMotorChart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured'
    );
    propLog1RightMotorChart.addSeries({ label: 'Id reference', spanGaps: false, stroke: '#43bccd' }); // teal
    propLog1RightMotorChart.addSeries({ label: 'Iq measured', spanGaps: false, stroke: '#f72585' }); // pink
    propLog1RightMotorChart.addSeries({ label: 'Iq reference', spanGaps: false, stroke: '#ffd166' }); // yellow
    // Min/Max
    propLog1RightMotorChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => propLog1Min });
    propLog1RightMotorChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => propLog1Max });
    $chartStore.set('Propulsion Log 1 - Right Motor', propLog1RightMotorChart);
    propCharts.push('Propulsion Log 1 - Right Motor');
    const xLenPropLog1Right = propLog1RightMotorChart.getSeriesData(0).length;
    propLog1RightMotorChart.updateSeries(5, new Float32Array(xLenPropLog1Right).fill(propLog1Min));
    propLog1RightMotorChart.updateSeries(6, new Float32Array(xLenPropLog1Right).fill(propLog1Max));

    // Prop log 2 Left Motor
    let propLog2LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2LeftMotorChart.addSeries({ label: 'VD', spanGaps: false, stroke: '#0ea774' }); // green
    propLog2LeftMotorChart.addSeries({ label: 'Vbus', spanGaps: false, stroke: '#0a85ff' }); // blue
    propLog2LeftMotorChart.addSeries({ label: 'Ibus', spanGaps: false, stroke: '#a259f7' }); // purple
    // Min/Max
    const propLog2Min = 0, propLog2Max = 20;
    propLog2LeftMotorChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => propLog2Min });
    propLog2LeftMotorChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => propLog2Max });
    $chartStore.set('Propulsion Log 2 - Left Motor', propLog2LeftMotorChart);
    propCharts.push('Propulsion Log 2 - Left Motor');
    const xLenPropLog2Left = propLog2LeftMotorChart.getSeriesData(0).length;
    propLog2LeftMotorChart.updateSeries(5, new Float32Array(xLenPropLog2Left).fill(propLog2Min));
    propLog2LeftMotorChart.updateSeries(6, new Float32Array(xLenPropLog2Left).fill(propLog2Max));

    // Prop log 2 Right Motor
    let propLog2RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2RightMotorChart.addSeries({ label: 'VD', spanGaps: false, stroke: '#f78c25' }); // orange
    propLog2RightMotorChart.addSeries({ label: 'Vbus', spanGaps: false, stroke: '#43bccd' }); // teal
    propLog2RightMotorChart.addSeries({ label: 'Ibus', spanGaps: false, stroke: '#f72585' }); // pink
    // Min/Max
    propLog2RightMotorChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => propLog2Min });
    propLog2RightMotorChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => propLog2Max });
    $chartStore.set('Propulsion Log 2 - Right Motor', propLog2RightMotorChart);
    propCharts.push('Propulsion Log 2 - Right Motor');
    const xLenPropLog2Right = propLog2RightMotorChart.getSeriesData(0).length;
    propLog2RightMotorChart.updateSeries(5, new Float32Array(xLenPropLog2Right).fill(propLog2Min));
    propLog2RightMotorChart.updateSeries(6, new Float32Array(xLenPropLog2Right).fill(propLog2Max));

    // Prop log 3 Left Motor
    let propLog3LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3LeftMotorChart.addSeries({ label: 'Tb', spanGaps: false, stroke: '#ffd166' }); // yellow
    propLog3LeftMotorChart.addSeries({ label: 'Tc', spanGaps: false, stroke: '#0ea774' }); // green
    propLog3LeftMotorChart.addSeries({ label: 'TCASE', spanGaps: false, stroke: '#0a85ff' }); // blue
    // Min/Max
    const propLog3Min = 0, propLog3Max = 20;
    propLog3LeftMotorChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => propLog3Min });
    propLog3LeftMotorChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => propLog3Max });
    $chartStore.set('Propulsion Log 3 - Left Motor', propLog3LeftMotorChart);
    propCharts.push('Propulsion Log 3 - Left Motor');
    const xLenPropLog3Left = propLog3LeftMotorChart.getSeriesData(0).length;
    propLog3LeftMotorChart.updateSeries(5, new Float32Array(xLenPropLog3Left).fill(propLog3Min));
    propLog3LeftMotorChart.updateSeries(6, new Float32Array(xLenPropLog3Left).fill(propLog3Max));

    // Prop log 3 Right Motor
    let propLog3RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3RightMotorChart.addSeries({ label: 'Tb', spanGaps: false, stroke: '#a259f7' }); // purple
    propLog3RightMotorChart.addSeries({ label: 'Tc', spanGaps: false, stroke: '#f78c25' }); // orange
    propLog3RightMotorChart.addSeries({ label: 'TCASE', spanGaps: false, stroke: '#43bccd' }); // teal
    // Min/Max
    propLog3RightMotorChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => propLog3Min });
    propLog3RightMotorChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => propLog3Max });
    $chartStore.set('Propulsion Log 3 - Right Motor', propLog3RightMotorChart);
    propCharts.push('Propulsion Log 3 - Right Motor');
    const xLenPropLog3Right = propLog3RightMotorChart.getSeriesData(0).length;
    propLog3RightMotorChart.updateSeries(5, new Float32Array(xLenPropLog3Right).fill(propLog3Min));
    propLog3RightMotorChart.updateSeries(6, new Float32Array(xLenPropLog3Right).fill(propLog3Max));

    // Motor Temperatures Left
    let motorLeftTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Left 1');
    motorLeftTemp.addSeries({ label: 'Motor Left 2', spanGaps: false, stroke: '#0a85ff' }); // blue
    motorLeftTemp.addSeries({ label: 'Motor Left 3', spanGaps: false, stroke: '#a259f7' }); // purple
    motorLeftTemp.addSeries({ label: 'Motor Left 4', spanGaps: false, stroke: '#f78c25' }); // orange
    motorLeftTemp.addSeries({ label: 'Motor Left 5', spanGaps: false, stroke: '#0ea774' }); // green
    motorLeftTemp.addSeries({ label: 'Motor Left 6', spanGaps: false, stroke: '#f72585' }); // pink
    motorLeftTemp.addSeries({ label: 'Motor Left 7', spanGaps: false, stroke: '#43bccd' }); // teal
    motorLeftTemp.addSeries({ label: 'Motor Left 8', spanGaps: false, stroke: '#ffd166' }); // yellow
    // Min/Max
    const motorLeftTempMin = 0, motorLeftTempMax = 120;
    motorLeftTemp.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => motorLeftTempMin });
    motorLeftTemp.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => motorLeftTempMax });
    $chartStore.set('Motor Temperatures Left', motorLeftTemp);
    propCharts.push('Motor Temperatures Left');
    const xLenMotorLeftTemp = motorLeftTemp.getSeriesData(0).length;
    motorLeftTemp.updateSeries(8, new Float32Array(xLenMotorLeftTemp).fill(motorLeftTempMin));
    motorLeftTemp.updateSeries(9, new Float32Array(xLenMotorLeftTemp).fill(motorLeftTempMax));

    // Motor Temperatures Right
    let motorRightTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Right 1');
    motorRightTemp.addSeries({ label: 'Motor Right 2', spanGaps: false, stroke: '#0a85ff' }); // blue
    motorRightTemp.addSeries({ label: 'Motor Right 3', spanGaps: false, stroke: '#a259f7' }); // purple
    motorRightTemp.addSeries({ label: 'Motor Right 4', spanGaps: false, stroke: '#f78c25' }); // orange
    motorRightTemp.addSeries({ label: 'Motor Right 5', spanGaps: false, stroke: '#0ea774' }); // green
    motorRightTemp.addSeries({ label: 'Motor Right 6', spanGaps: false, stroke: '#f72585' }); // pink
    motorRightTemp.addSeries({ label: 'Motor Right 7', spanGaps: false, stroke: '#43bccd' }); // teal
    motorRightTemp.addSeries({ label: 'Motor Right 8', spanGaps: false, stroke: '#ffd166' }); // yellow
    // Min/Max
    const motorRightTempMin = 0, motorRightTempMax = 120;
    motorRightTemp.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => motorRightTempMin });
    motorRightTemp.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => motorRightTempMax });
    $chartStore.set('Motor Temperatures Right', motorRightTemp);
    propCharts.push('Motor Temperatures Right');
    const xLenMotorRightTemp = motorRightTemp.getSeriesData(0).length;
    motorRightTemp.updateSeries(8, new Float32Array(xLenMotorRightTemp).fill(motorRightTempMin));
    motorRightTemp.updateSeries(9, new Float32Array(xLenMotorRightTemp).fill(motorRightTempMax));

    // Offset
    let offsetChart = new PlotBuffer(500, 60000, [0, 30], true, 'Offset 1');
    offsetChart.addSeries({ label: 'Offset 2', spanGaps: false, stroke: '#0a85ff' }); // blue
    offsetChart.addSeries({ label: 'Offset 3', spanGaps: false, stroke: '#a259f7' }); // purple
    offsetChart.addSeries({ label: 'Offset 4', spanGaps: false, stroke: '#f78c25' }); // orange
    // Min/Max
    const offsetMin = 0, offsetMax = 30;
    offsetChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => offsetMin });
    offsetChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => offsetMax });
    $chartStore.set('Offset', offsetChart);
    propCharts.push('Offset');
    const xLenOffset = offsetChart.getSeriesData(0).length;
    offsetChart.updateSeries(5, new Float32Array(xLenOffset).fill(offsetMin));
    offsetChart.updateSeries(6, new Float32Array(xLenOffset).fill(offsetMax));

    let airGapChart = new PlotBuffer(
        500,
        60000,
        [0, 30],
        true,
        'Vertical Air Gap'
    );
    airGapChart.addSeries({ label: 'Lateral Air Gap', spanGaps: false, stroke: '#a259f7' }); // purple
    // Min/Max
    const airGapMin = 0, airGapMax = 30;
    airGapChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => airGapMin });
    airGapChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => airGapMax });
    $chartStore.set('Air Gaps', airGapChart);
    leviCharts.push('Air Gaps');
    const xLenAirGaps = airGapChart.getSeriesData(0).length;
    airGapChart.updateSeries(3, new Float32Array(xLenAirGaps).fill(airGapMin));
    airGapChart.updateSeries(4, new Float32Array(xLenAirGaps).fill(airGapMax));

    let anglesChart = new PlotBuffer(500, 60000, [0, 120], true, 'Roll');
    anglesChart.addSeries({ label: 'Pitch', spanGaps: false, stroke: '#a259f7' }); // purple
    anglesChart.addSeries({ label: 'Yaw', spanGaps: false, stroke: '#f78c25' }); // orange
    // Min/Max
    const anglesMin = 0, anglesMax = 120;
    anglesChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => anglesMin });
    anglesChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => anglesMax });
    $chartStore.set('Angles', anglesChart);
    leviCharts.push('Angles');
    const xLenAngles = anglesChart.getSeriesData(0).length;
    anglesChart.updateSeries(4, new Float32Array(xLenAngles).fill(anglesMin));
    anglesChart.updateSeries(5, new Float32Array(xLenAngles).fill(anglesMax));

    let hemsCurrentChart = new PlotBuffer(
        500,
        3 * 60000,
        [-11.3, 11.3],
        true,
        'VFL1'
    );
    hemsCurrentChart.addSeries({ label: 'VFL2', spanGaps: false, stroke: '#0a85ff' }); // blue
    hemsCurrentChart.addSeries({ label: 'VFR1', spanGaps: false, stroke: '#a259f7' }); // purple
    hemsCurrentChart.addSeries({ label: 'VFR2', spanGaps: false, stroke: '#f78c25' }); // orange
    hemsCurrentChart.addSeries({ label: 'VBL1', spanGaps: false, stroke: '#0ea774' }); // green
    hemsCurrentChart.addSeries({ label: 'VBL2', spanGaps: false, stroke: '#f72585' }); // pink
    hemsCurrentChart.addSeries({ label: 'VBR1', spanGaps: false, stroke: '#43bccd' }); // teal
    hemsCurrentChart.addSeries({ label: 'VBR2', spanGaps: false, stroke: '#ffd166' }); // yellow
    // Min/Max
    const hemsCurrentMin = -11.3, hemsCurrentMax = 11.3;
    hemsCurrentChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => hemsCurrentMin });
    hemsCurrentChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => hemsCurrentMax });
    $chartStore.set('HEMS Current', hemsCurrentChart);
    leviCharts.push('HEMS Current');
    const xLenHems = hemsCurrentChart.getSeriesData(0).length;
    hemsCurrentChart.updateSeries(9, new Float32Array(xLenHems).fill(hemsCurrentMin));
    hemsCurrentChart.updateSeries(10, new Float32Array(xLenHems).fill(hemsCurrentMax));

    let emsCurrentChart = new PlotBuffer(500, 3 * 60000, [-11.3, 11.3], true, 'LF');
    emsCurrentChart.addSeries({ label: 'LB', spanGaps: false, stroke: '#a259f7' }); // purple
    // Min/Max
    const emsCurrentMin = -11.3, emsCurrentMax = 11.3;
    emsCurrentChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => emsCurrentMin });
    emsCurrentChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => emsCurrentMax });
    $chartStore.set('EMS Current', emsCurrentChart);
    leviCharts.push('EMS Current');
    const xLenEms = emsCurrentChart.getSeriesData(0).length;
    emsCurrentChart.updateSeries(3, new Float32Array(xLenEms).fill(emsCurrentMin));
    emsCurrentChart.updateSeries(4, new Float32Array(xLenEms).fill(emsCurrentMax));

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
    leviRequestForceVerticalChart.addSeries({ label: 'Roll', spanGaps: false, stroke: '#0a85ff' }); // blue
    leviRequestForceVerticalChart.addSeries({ label: 'Pitch', spanGaps: false, stroke: '#a259f7' }); // purple
    // Min/Max
    const reqForceVertMin = 0, reqForceVertMax = 100;
    leviRequestForceVerticalChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => reqForceVertMin });
    leviRequestForceVerticalChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => reqForceVertMax });
    $chartStore.set('Requested Force Vertical', leviRequestForceVerticalChart);
    leviCharts.push('Requested Force Vertical');
    const xLenReqVert = leviRequestForceVerticalChart.getSeriesData(0).length;
    leviRequestForceVerticalChart.updateSeries(4, new Float32Array(xLenReqVert).fill(reqForceVertMin));
    leviRequestForceVerticalChart.updateSeries(5, new Float32Array(xLenReqVert).fill(reqForceVertMax));

    let leviRequestForceHorizontalChart = new PlotBuffer(500, 60000, [0, 100], true, 'Y');
    leviRequestForceHorizontalChart.addSeries({ label: 'Yaw', spanGaps: false, stroke: '#f78c25' }); // orange
    // Min/Max
    const reqForceHorizMin = 0, reqForceHorizMax = 100;
    leviRequestForceHorizontalChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => reqForceHorizMin });
    leviRequestForceHorizontalChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => reqForceHorizMax });
    $chartStore.set('Requested Force Horizontal', leviRequestForceHorizontalChart);
    leviCharts.push('Requested Force Horizontal');
    const xLenReqHoriz = leviRequestForceHorizontalChart.getSeriesData(0).length;
    leviRequestForceHorizontalChart.updateSeries(3, new Float32Array(xLenReqHoriz).fill(reqForceHorizMin));
    leviRequestForceHorizontalChart.updateSeries(4, new Float32Array(xLenReqHoriz).fill(reqForceHorizMax));

    let hemsTempChart = new PlotBuffer(500, 60000, [0, 100], true, 'L1');
    hemsTempChart.addSeries({ label: 'L2', spanGaps: false, stroke: '#0a85ff' }); // blue
    hemsTempChart.addSeries({ label: 'L3', spanGaps: false, stroke: '#a259f7' }); // purple
    hemsTempChart.addSeries({ label: 'L4', spanGaps: false, stroke: '#f78c25' }); // orange
    hemsTempChart.addSeries({ label: 'R1', spanGaps: false, stroke: '#0ea774' }); // green
    hemsTempChart.addSeries({ label: 'R2', spanGaps: false, stroke: '#f72585' }); // pink
    hemsTempChart.addSeries({ label: 'R3', spanGaps: false, stroke: '#43bccd' }); // teal
    hemsTempChart.addSeries({ label: 'R4', spanGaps: false, stroke: '#ffd166' }); // yellow
    // Min/Max
    const hemsTempMin = 0, hemsTempMax = 100;
    hemsTempChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => hemsTempMin });
    hemsTempChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => hemsTempMax });
    $chartStore.set('Temperatures HEMS', hemsTempChart);
    leviCharts.push('Temperatures HEMS');
    const xLenHemsTemp = hemsTempChart.getSeriesData(0).length;
    hemsTempChart.updateSeries(9, new Float32Array(xLenHemsTemp).fill(hemsTempMin));
    hemsTempChart.updateSeries(10, new Float32Array(xLenHemsTemp).fill(hemsTempMax));

    let emsTempChart = new PlotBuffer(500, 60000, [0, 100], true, 'L1');
    emsTempChart.addSeries({ label: 'L2', spanGaps: false, stroke: '#0a85ff' }); // blue
    emsTempChart.addSeries({ label: 'L3', spanGaps: false, stroke: '#a259f7' }); // purple
    emsTempChart.addSeries({ label: 'L4', spanGaps: false, stroke: '#f78c25' }); // orange
    emsTempChart.addSeries({ label: 'R1', spanGaps: false, stroke: '#0ea774' }); // green
    emsTempChart.addSeries({ label: 'R2', spanGaps: false, stroke: '#f72585' }); // pink
    emsTempChart.addSeries({ label: 'R3', spanGaps: false, stroke: '#43bccd' }); // teal
    emsTempChart.addSeries({ label: 'R4', spanGaps: false, stroke: '#ffd166' }); // yellow
    // Min/Max
    const emsTempMin = 0, emsTempMax = 100;
    emsTempChart.addSeries({ label: 'Min Safe', stroke: '#ffde0a', dash: [6, 6], spanGaps: true, show: true, value: () => emsTempMin });
    emsTempChart.addSeries({ label: 'Max Safe', stroke: '#ff0a43', dash: [6, 6], spanGaps: true, show: true, value: () => emsTempMax });
    $chartStore.set('Temperatures EMS', emsTempChart);
    leviCharts.push('Temperatures EMS');
    const xLenEmsTemp = emsTempChart.getSeriesData(0).length;
    emsTempChart.updateSeries(9, new Float32Array(xLenEmsTemp).fill(emsTempMin));
    emsTempChart.updateSeries(10, new Float32Array(xLenEmsTemp).fill(emsTempMax));

    let BMSVoltageHighChart = new PlotBuffer(500, 60000, [-500, 500], true, "BMS Voltage High");
    // Add static min/max lines (placeholder values)
    const bmsHighMin = 280; // TODO: update with real min
    const bmsHighMax = 420; // TODO: update with real max
    BMSVoltageHighChart.addSeries({
        label: "Min Safe",
        stroke: "#ffde0a",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => bmsHighMin
    });
    BMSVoltageHighChart.addSeries({
        label: "Max Safe",
        stroke: "#ff0a43",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => bmsHighMax
    });
    $chartStore.set("BMS Voltage High", BMSVoltageHighChart);
    powertrainCharts.push("BMS Voltage High");

    // After adding static min/max series for BMS Voltage High
    const xLenHigh = BMSVoltageHighChart.getSeriesData(0).length;
    BMSVoltageHighChart.updateSeries(2, new Float32Array(xLenHigh).fill(bmsHighMin)); // Min Safe
    BMSVoltageHighChart.updateSeries(3, new Float32Array(xLenHigh).fill(bmsHighMax)); // Max Safe

    let BMSVoltageLowChart = new PlotBuffer(500, 60000, [-500, 500], true, "BSM Voltage Low");
    // Add static min/max lines (placeholder values)
    const bmsLowMin = 280; // TODO: update with real min
    const bmsLowMax = 360; // TODO: update with real max
    BMSVoltageLowChart.addSeries({
        label: "Min Safe",
        stroke: "#ffde0a",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => bmsLowMin
    });
    BMSVoltageLowChart.addSeries({
        label: "Max Safe",
        stroke: "#ff0a43",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => bmsLowMax
    });
    $chartStore.set("BMS Voltage Low", BMSVoltageLowChart);
    powertrainCharts.push("BMS Voltage Low");

    // After adding static min/max series for BMS Voltage Low
    const xLenLow = BMSVoltageLowChart.getSeriesData(0).length;
    BMSVoltageLowChart.updateSeries(2, new Float32Array(xLenLow).fill(bmsLowMin)); // Min Safe
    BMSVoltageLowChart.updateSeries(3, new Float32Array(xLenLow).fill(bmsLowMax)); // Max Safe

    let BMSVoltageTempsChart = new PlotBuffer(500, 60000, [-500, 500], true, "Temp High");
    BMSVoltageTempsChart.addSeries(StrokePresets.blue("Temp Low"));
    // Add static min/max lines (placeholder values)
    const tempMin = 0; // TODO: update with real min
    const tempMax = 80; // TODO: update with real max
    BMSVoltageTempsChart.addSeries({
        label: "Min Safe",
        stroke: "#ffde0a",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => tempMin
    });
    BMSVoltageTempsChart.addSeries({
        label: "Max Safe",
        stroke: "#ff0a43",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => tempMax
    });
    $chartStore.set("BMS Temps", BMSVoltageTempsChart);
    powertrainCharts.push("BMS Temps");

    // After adding static min/max series for BMS Temps
    const xLenTemps = BMSVoltageTempsChart.getSeriesData(0).length;
    BMSVoltageTempsChart.updateSeries(3, new Float32Array(xLenTemps).fill(tempMin)); // Min Safe 
    BMSVoltageTempsChart.updateSeries(4, new Float32Array(xLenTemps).fill(tempMax)); // Max Safe 

    let BMSLogsChart = new PlotBuffer(500, 60000, [-500, 500], true, "V Pack");
    BMSLogsChart.addSeries(StrokePresets.blue("I Pack"));
    BMSLogsChart.addSeries({
        label: "V DC Link",
        spanGaps: false,
        stroke: "#a259f7"
    });
    // Add static min/max lines (placeholder values)
    const logMin = 0; // TODO: update with real min
    const logMax = 100; // TODO: update with real max
    BMSLogsChart.addSeries({
        label: "Min Safe",
        stroke: "#ffde0a",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => logMin
    });
    BMSLogsChart.addSeries({
        label: "Max Safe",
        stroke: "#ff0a43",
        dash: [6, 6],
        spanGaps: true,
        show: true,
        value: () => logMax
    });
    $chartStore.set("BMS Logs", BMSLogsChart);
    powertrainCharts.push("BMS Logs");

    // After adding static min/max series for BMS Logs
    const xLenLogs = BMSLogsChart.getSeriesData(0).length;
    BMSLogsChart.updateSeries(4, new Float32Array(xLenLogs).fill(logMin)); 
    BMSLogsChart.updateSeries(5, new Float32Array(xLenLogs).fill(logMax)); 

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
    gdd.stores.registerStore<number>("PTCNonCriticalFault", 0);

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

    gdd.stores.registerStore<number>("VPack", 0, data => {
            const curr = Number(data);
            $chartStore.get("BMS Logs")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("IPack", 0, data => {
            const curr = Number(data);
            $chartStore.get("BMS Logs")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("VDCLink", 0, data => {
            const curr = Number(data);
            $chartStore.get("BMS Logs")!.addEntry(3, curr);
            return curr;
        }
    );

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
            const curr = Number(data);
            $chartStore.get("Angles")!.addEntry(1, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Pitch", 0, data => {
            const curr = Number(data);
            $chartStore.get("Angles")!.addEntry(2, curr);
            return curr;
        }
    );

    gdd.stores.registerStore<number>("Yaw", 0, data => {
            const curr = Number(data);
            $chartStore.get("Angles")!.addEntry(3, curr);
            return curr;
        }
    );

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

    // End of generated

    gdd.stores.registerStore<number>("FrontendHeartbeating", 0);

    gdd.start(50);

    initializeStores();

    setInterval(() => {
        latestTimestamp.set(Date.now());
    }, 1000);

    let firstPass: boolean = true;

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

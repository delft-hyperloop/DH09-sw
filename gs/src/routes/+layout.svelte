<script lang="ts">
    import '../app.postcss';
    import { BottomBar, EventChannel, GrandDataDistributor, PlotBuffer, StrokePresets, TitleBar, util } from '$lib';
    import { initializeStores, Modal, type ModalComponent, storePopup, Toast } from '@skeletonlabs/skeleton';
    import {
        chartStore,
        debugModeActive,
        inDropdown,
        latestTimestamp,
        leviChartStore,
        logsVisible,
        powertrainChartStore,
        propChartStore,
        RedHVALTurnedOn,
        showcaseStateCounter,
        showcasingStates,
    } from '$lib/stores/state';
    import { initProcedures } from '$lib/stores/data';
    import { onDestroy, onMount } from 'svelte';
    import { listen } from '@tauri-apps/api/event';
    import { parseShortCut } from '$lib/util/parsers';
    import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
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
        parseShortCut(event.payload, $debugModeActive, $logsVisible),
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
        'Id measured',
    );
    propLog1LeftMotorChart.addSeries(StrokePresets.yellow('Id reference'));
    propLog1LeftMotorChart.addSeries(StrokePresets.blue('Iq measured'));
    propLog1LeftMotorChart.addSeries(StrokePresets.theoretical('Iq reference'));
    $chartStore.set('Propulsion Log 1 - Left Motor', propLog1LeftMotorChart);
    propCharts.push('Propulsion Log 1 - Left Motor');

    // Prop log 1 Right Motor chart for test runs
    let propLog1RightMotorChart = new PlotBuffer(
        500,
        3 * 60000,
        [0, 20],
        true,
        'Id measured',
    );
    propLog1RightMotorChart.addSeries(StrokePresets.yellow('Id reference'));
    propLog1RightMotorChart.addSeries(StrokePresets.blue('Iq measured'));
    propLog1RightMotorChart.addSeries(StrokePresets.theoretical('Iq reference'));
    $chartStore.set('Propulsion Log 1 - Right Motor', propLog1RightMotorChart);
    propCharts.push('Propulsion Log 1 - Right Motor');

    // Prop log 2 Left Motor chart for test runs
    let propLog2LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2LeftMotorChart.addSeries(StrokePresets.yellow('VD'));
    propLog2LeftMotorChart.addSeries(StrokePresets.blue('Vbus'));
    propLog2LeftMotorChart.addSeries(StrokePresets.theoretical('Ibus'));
    $chartStore.set('Propulsion Log 2 - Left Motor', propLog2LeftMotorChart);
    propCharts.push('Propulsion Log 2 - Left Motor');

    // Prop log 2 Right Motor chart for test runs
    let propLog2RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'VQ');
    propLog2RightMotorChart.addSeries(StrokePresets.yellow('VD'));
    propLog2RightMotorChart.addSeries(StrokePresets.blue('Vbus'));
    propLog2RightMotorChart.addSeries(StrokePresets.theoretical('Ibus'));
    $chartStore.set('Propulsion Log 2 - Right Motor', propLog2RightMotorChart);
    propCharts.push('Propulsion Log 2 - Right Motor');

    // Prop log 3 Left Motor chart for test runs
    let propLog3LeftMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3LeftMotorChart.addSeries(StrokePresets.yellow('Tb'));
    propLog3LeftMotorChart.addSeries(StrokePresets.blue('Tc'));
    propLog3LeftMotorChart.addSeries(StrokePresets.theoretical('TCASE'));
    $chartStore.set('Propulsion Log 3 - Left Motor', propLog3LeftMotorChart);
    propCharts.push('Propulsion Log 3 - Left Motor');

    // Prop log 3 Right Motor chart for test runs
    let propLog3RightMotorChart = new PlotBuffer(500, 3 * 60000, [0, 20], true, 'Ta');
    propLog3RightMotorChart.addSeries(StrokePresets.yellow('Tb'));
    propLog3RightMotorChart.addSeries(StrokePresets.blue('Tc'));
    propLog3RightMotorChart.addSeries(StrokePresets.theoretical('TCASE'));
    $chartStore.set('Propulsion Log 3 - Right Motor', propLog3RightMotorChart);
    propCharts.push('Propulsion Log 3 - Right Motor');

    let motorLeftTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Left 1');
    motorLeftTemp.addSeries(StrokePresets.yellow('Motor Left 2'));
    motorLeftTemp.addSeries(StrokePresets.blue('Motor Left 3'));
    motorLeftTemp.addSeries(StrokePresets.blueDashed('Motor Left 4'));
    motorLeftTemp.addSeries(StrokePresets.hyperLoopGreen('Motor Left 5'));
    motorLeftTemp.addSeries(StrokePresets.hyperloopGreenDashed('Motor Left 6'));
    motorLeftTemp.addSeries(StrokePresets.yellowDashed('Motor Left 7'));
    motorLeftTemp.addSeries(StrokePresets.theoretical('Motor Left 8'));
    $chartStore.set('Motor Temperatures Left', motorLeftTemp);
    propCharts.push('Motor Temperatures Left');

    let motorRightTemp = new PlotBuffer(500, 60000, [0, 120], true, 'Motor Right 1');
    motorRightTemp.addSeries(StrokePresets.yellow('Motor Right 2'));
    motorRightTemp.addSeries(StrokePresets.blue('Motor Right 3'));
    motorRightTemp.addSeries(StrokePresets.blueDashed('Motor Right 4'));
    motorRightTemp.addSeries(StrokePresets.hyperLoopGreen('Motor Right 5'));
    motorRightTemp.addSeries(StrokePresets.hyperloopGreenDashed('Motor Right 6'));
    motorRightTemp.addSeries(StrokePresets.yellowDashed('Motor Right 7'));
    motorRightTemp.addSeries(StrokePresets.theoretical('Motor Right 8'));
    $chartStore.set('Motor Temperatures Right', motorRightTemp);
    propCharts.push('Motor Temperatures Right');

    let offsetChart = new PlotBuffer(500, 60000, [0, 30], true, 'Offset 1');
    offsetChart.addSeries(StrokePresets.hyperLoopGreen('Offset 2'));
    offsetChart.addSeries(StrokePresets.yellow('Offset 3'));
    offsetChart.addSeries(StrokePresets.blue('Offset 4'));
    $chartStore.set('Offset', offsetChart);
    propCharts.push('Offset');

    let airGapChart = new PlotBuffer(
        500,
        60000,
        [0, 30],
        true,
        'Vertical Air Gap',
    );
    airGapChart.addSeries(StrokePresets.theoretical('Lateral Air Gap'));
    $chartStore.set('Air Gaps', airGapChart);
    leviCharts.push('Air Gaps');

    let anglesChart = new PlotBuffer(500, 60000, [0, 120], true, 'Roll');
    anglesChart.addSeries(StrokePresets.theoretical('Pitch'));
    anglesChart.addSeries(StrokePresets.blue('Yaw'));
    $chartStore.set('Angles', anglesChart);
    leviCharts.push('Angles');

    let hemsCurrentChart = new PlotBuffer(
        500,
        3 * 60000,
        [-11.3, 11.3],
        true,
        'VFL1',
    );
    hemsCurrentChart.addSeries(StrokePresets.blue('VFL2'));
    hemsCurrentChart.addSeries(StrokePresets.theoretical('VFR1'));
    hemsCurrentChart.addSeries(StrokePresets.yellow('VFR2'));
    hemsCurrentChart.addSeries(StrokePresets.blueDashed('VBL1'));
    hemsCurrentChart.addSeries(StrokePresets.theoreticalDashed('VBL2'));
    hemsCurrentChart.addSeries(StrokePresets.yellowDashed('VBR1'));
    hemsCurrentChart.addSeries(StrokePresets.hyperLoopGreen('VBR2'));
    $chartStore.set('HEMS Current', hemsCurrentChart);
    leviCharts.push('HEMS Current');

    let emsCurrentChart = new PlotBuffer(500, 3 * 60000, [-11.3, 11.3], true, 'LF');
    emsCurrentChart.addSeries(StrokePresets.theoretical('LB'));
    $chartStore.set('EMS Current', emsCurrentChart);
    leviCharts.push('EMS Current');

    let accelChart = new PlotBuffer(500, 60000, [0, 25], false);
    $chartStore.set('Acceleration', accelChart);
    propCharts.push('Acceleration');

    let velChart = new PlotBuffer(500, 60000, [0, 100], false);
    $chartStore.set('Velocity', velChart);
    propCharts.push('Velocity');

    let localizationChart = new PlotBuffer(500, 60000, [0, 13000], false);
    localizationChart.addSeries(StrokePresets.yellow('Localization'));
    $chartStore.set('Localization', localizationChart);

    let leviRequestForceVerticalChart = new PlotBuffer(500, 60000, [0, 100], true, 'Z');
    leviRequestForceVerticalChart.addSeries(StrokePresets.yellow('Roll'));
    leviRequestForceVerticalChart.addSeries(StrokePresets.theoretical('Pitch'));
    $chartStore.set('Requested Force Vertical', leviRequestForceVerticalChart);
    leviCharts.push('Requested Force Vertical');

    let leviRequestForceHorizontalChart = new PlotBuffer(500, 60000, [0, 100], true, 'Y');
    leviRequestForceHorizontalChart.addSeries(StrokePresets.yellow('Yaw'));
    $chartStore.set('Requested Force Horizontal', leviRequestForceHorizontalChart);
    leviCharts.push('Requested Force Horizontal');

    let hemsTempChart = new PlotBuffer(500, 60000, [0, 100], true, 'L1');
    hemsTempChart.addSeries(StrokePresets.yellow('L2'));
    hemsTempChart.addSeries(StrokePresets.theoretical('L3'));
    hemsTempChart.addSeries(StrokePresets.yellowDashed('L4'));
    hemsTempChart.addSeries(StrokePresets.hyperLoopGreen('R1'));
    hemsTempChart.addSeries(StrokePresets.blue('R2'));
    hemsTempChart.addSeries(StrokePresets.blueDashed('R3'));
    hemsTempChart.addSeries(StrokePresets.hyperloopGreenDashed('R4'));
    $chartStore.set('Temperatures HEMS', hemsTempChart);
    leviCharts.push('Temperatures HEMS');

    let emsTempChart = new PlotBuffer(500, 60000, [0, 100], true, 'L1');
    emsTempChart.addSeries(StrokePresets.yellow('L2'));
    emsTempChart.addSeries(StrokePresets.theoretical('L3'));
    emsTempChart.addSeries(StrokePresets.yellowDashed('L4'));
    emsTempChart.addSeries(StrokePresets.hyperLoopGreen('R1'));
    emsTempChart.addSeries(StrokePresets.blue('R2'));
    emsTempChart.addSeries(StrokePresets.blueDashed('R3'));
    emsTempChart.addSeries(StrokePresets.hyperloopGreenDashed('R4'));
    $chartStore.set('Temperatures EMS', emsTempChart);
    leviCharts.push('Temperatures EMS');

    let isolationChart = new PlotBuffer(500, 60000, [-500, 500], true, 'Isolation Resistance');
    $chartStore.set('Isolation Resistance', isolationChart);
    powertrainCharts.push('Isolation Resistance');

    let busCurrentChart = new PlotBuffer(500, 60000, [-500, 500], true, 'Bus Current');
    busCurrentChart.addSeries(StrokePresets.blue('HV Pack Current'));
    $chartStore.set('Bus Current', busCurrentChart);
    powertrainCharts.push('Bus Current');

    let bmsTempsChart = new PlotBuffer(500, 60000, [-500, 500], true, 'HV Highest Temperature');
    bmsTempsChart.addSeries(StrokePresets.blue('HV Lowest Temperatures'));
    bmsTempsChart.addSeries(StrokePresets.theoretical('LV Highest Temperature'));
    bmsTempsChart.addSeries(StrokePresets.hyperLoopGreen('LV Lowest Temperature'));
    $chartStore.set('BMS Temperatures', bmsTempsChart);
    powertrainCharts.push('BMS Temperatures');

    let bmsVoltagesChart = new PlotBuffer(500, 60000, [-500, 500], true, 'HV Highest Voltage');
    bmsVoltagesChart.addSeries(StrokePresets.blue('HV Lowest Voltage'));
    bmsVoltagesChart.addSeries(StrokePresets.theoretical('LV Highest Voltage'));
    bmsVoltagesChart.addSeries(StrokePresets.hyperLoopGreen('LV Lowest Voltage'));
    $chartStore.set('BMS Voltages', bmsVoltagesChart);
    powertrainCharts.push('BMS Voltages');

    let hvPackVoltageChart = new PlotBuffer(500, 60000, [-500, 500], true, 'HV Pack Voltage');
    $chartStore.set('HV Pack Voltage', hvPackVoltageChart);
    powertrainCharts.push('HV Pack Voltage');

    let lvPackVoltageChart = new PlotBuffer(500, 60000, [-500, 500], true, 'LV Pack Voltage');
    $chartStore.set('LV Pack Voltage', lvPackVoltageChart);
    powertrainCharts.push('LV Pack Voltage');

    let lvPackCurrentChart = new PlotBuffer(500, 60000, [-500, 500], true, 'LV Pack Current');
    $chartStore.set('LV Pack Current', lvPackCurrentChart);
    powertrainCharts.push('LV Pack Current');

    let dcLinkVoltageChart = new PlotBuffer(500, 60000, [-500, 500], true, 'DC Link Voltage');
    $chartStore.set('DC Link Voltage', dcLinkVoltageChart);
    powertrainCharts.push('DC Link Voltage');

    let pressureChart = new PlotBuffer(500, 60000, [-500, 500], true, 'Pressure Low');
    pressureChart.addSeries(StrokePresets.hyperLoopGreen('Pressure High'));
    $chartStore.set('Brake Pressure', pressureChart);

    leviChartStore.set(leviCharts);
    propChartStore.set(propCharts);
    powertrainChartStore.set(powertrainCharts);

    ////////////////////////////////////////////////////////////////

    let gdd = GrandDataDistributor.getInstance();

    ////////////////////////////////////////////////////////////////

    // gdd stores registration

	// BEGIN AUTO GENERATED STORES
        

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

		gdd.stores.registerStore<number>("HVALState", 10);

		gdd.stores.registerStore<number>("IMDWarnings", 0);

		gdd.stores.registerStore<number>("PTCErrors", 0);

		gdd.stores.registerStore<number>("HvVHigh", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Voltages")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("HvVLow", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Voltages")!.addEntry(2, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("BMSTemperatureHigh", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Temperatures")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("BMSTemperatureLow", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Temperatures")!.addEntry(2, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("VPack", 0, data => {
    const curr = Number(data);
    $chartStore.get("HV Pack Voltage")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("IPack", 0, data => {
    const curr = Number(data);
    $chartStore.get("Bus Current")!.addEntry(2, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("VDCLink", 0, data => {
    const curr = Number(data);
    $chartStore.get("DC Link Voltage")!.addEntry(1, curr);
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

		gdd.stores.registerStore<number>("LeviSystemCheckResponse", 0);

		gdd.stores.registerStore<number>("LvVHigh", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Voltages")!.addEntry(3, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("LvVLow", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Voltages")!.addEntry(4, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("THigh", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Temperatures")!.addEntry(3, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("TLow", 0, data => {
    const curr = Number(data);
    $chartStore.get("BMS Temperatures")!.addEntry(4, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("IsolationResistance", 0, data => {
    const curr = Number(data);
    $chartStore.get("Isolation Resistance")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("BusCurrent", 0, data => {
    const curr = Number(data);
    $chartStore.get("Bus Current")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("VPackLowVoltage", 0, data => {
    const curr = Number(data);
    $chartStore.get("LV Pack Voltage")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("IPackLowVoltage", 0, data => {
    const curr = Number(data);
    $chartStore.get("LV Pack Current")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("PtSystemCheckResponse", 0);

		gdd.stores.registerStore<number>("LeviFault", 0);

		gdd.stores.registerStore<number>("LeviFaultDriveNumber", 0);

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

		gdd.stores.registerStore<number>("PressureLow", 0, data => {
    const curr = Number(data);
    $chartStore.get("Brake Pressure")!.addEntry(1, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("PressureHigh", 0, data => {
    const curr = Number(data);
    $chartStore.get("Brake Pressure")!.addEntry(2, curr);
    return curr;
}
);

		gdd.stores.registerStore<number>("SensorHubEmergency", 0);

		gdd.stores.registerStore<number>("PtcErrorEmergency", 0);

		gdd.stores.registerStore<number>("BmsErrorLowVoltage", 0);

		gdd.stores.registerStore<number>("BmsErrorHighVoltage", 0);

		gdd.stores.registerStore<number>("FSMState", 0);

		gdd.stores.registerStore<number>("FSMTransitionFail", 100);

		gdd.stores.registerStore<number>("Emergency", 0);

		gdd.stores.registerStore<number>("LeviSystemCheckSuccess", 0);

		gdd.stores.registerStore<number>("LeviSystemCheckFailure", 0);

		gdd.stores.registerStore<number>("Prop1SystemCheckSuccess", 0);

		gdd.stores.registerStore<number>("Prop1SystemCheckFailure", 0);

		gdd.stores.registerStore<number>("Prop2SystemCheckSuccess", 0);

		gdd.stores.registerStore<number>("Prop2SystemCheckFailure", 0);
    // END AUTO GENERATED STORES

    gdd.stores.registerStore<number>('FrontendHeartbeating', 0);

    gdd.stores.registerStore<number>('EmergencyStaleCriticalData', 0);

    gdd.start(50);

    initializeStores();

    setInterval(() => {
        latestTimestamp.set(Date.now());
    }, 1000);

    registerSubscribers();

    onMount(() => setInterval(() => {
        RedHVALTurnedOn.set(!$RedHVALTurnedOn);
    }, 200));

    onMount(() => {
        setInterval(async () => {
            if ($showcasingStates) {
                showcaseStateCounter.set(($showcaseStateCounter + 1) % 14);
            }
            if ($inDropdown) {
                let val = 0;
                console.log(`Sending command: LeviDropdown`);
                await invoke('send_command', { cmdName: 'LeviDropdown', val }).then(() => {
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
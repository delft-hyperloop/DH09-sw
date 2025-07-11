import { EventChannel, GrandDataDistributor, util } from '$lib';
import { getModalStore, getToastStore } from '@skeletonlabs/skeleton';
import { invoke } from '@tauri-apps/api/tauri';
import { lastHeartbeatTimestamp, modalBody, modalTitle } from '$lib/stores/data';
import { MODAL_SETTINGS } from '$lib/types';
import {
    emergencyModalActive,
    emsTempsAcknowledged,
    hemsTempsAcknowledged,
    leftMotorTempsAcknowledged,
    propEmergency1Acknowledged,
    propEmergency2Acknowledged,
    propInitFault1Acknowledged,
    propInitFault2Acknowledged,
    rightMotorTempsAcknowledged,
} from '$lib/stores/state';
import { get } from 'svelte/store';

export function registerSubscribers() {
    const storeManager = GrandDataDistributor.getInstance().stores;
    const modalStore = getModalStore();
    const toastStore = getToastStore();

    const fsmTransitionFail = storeManager.getWritable("FSMTransitionFail");
    const propInitFault1 = storeManager.getWritable("PPInitFault1");
    const propEmergency1 = storeManager.getWritable("PPEmergency1");
    const propInitFault2 = storeManager.getWritable("PPInitFault2");
    const propEmergency2 = storeManager.getWritable("PPEmergency2");
    const heartbeat = storeManager.getWritable("FrontendHeartbeating");
    const emergency = storeManager.getWritable("Emergency");

    const systemCheckLabels = [
        "Levi",
        "Prop 1",
        "Prop 2",
    ]
    const systemCheckSuccesses = [
        storeManager.getWritable("LeviSystemCheckSuccess"),
        storeManager.getWritable("Prop1SystemCheckSuccess"),
        storeManager.getWritable("Prop2SystemCheckSuccess"),
    ]
    const systemCheckFailures = [
        storeManager.getWritable("LeviSystemCheckFailure"),
        storeManager.getWritable("Prop1SystemCheckFailure"),
        storeManager.getWritable("Prop2SystemCheckFailure"),
    ]

    fsmTransitionFail.subscribe(async (store) => {
        let state: string = await invoke('get_fsm_state_by_index', { index: store.value });
        if (state !== "UnknownState") {
            toastStore.trigger({
                message: `Transition to state ${state} failed!`,
                background: 'bg-error-400',
                autohide: false,
            });
            console.error(`Transition to state ${state} failed!`);
            util.log(`Transition to state ${state} failed!`, EventChannel.ERROR);
        }
    });

    systemCheckSuccesses.forEach((store, index) => {
        store.subscribe((value) => {
            if (value.value !== 0) {
                toastStore.trigger({
                    message: `${systemCheckLabels[index]} System Check Passed`,
                    background: `bg-surface-600`,
                    autohide: true,
                });
            }
        })
    });
    systemCheckFailures.forEach((store, index) => {
        store.subscribe((value) => {
            if (value.value !== 0) {
                toastStore.trigger({
                    message: `${systemCheckLabels[index]} System Check Failed!`,
                    background: 'bg-error-400',
                    autohide: false,
                });
            }
        })
    })

    heartbeat.subscribe(() => {
        lastHeartbeatTimestamp.set(Date.now());
    })

    let emsTemps = [
        storeManager.getWritable("TempEMS1"),
        storeManager.getWritable("TempEMS2"),
        storeManager.getWritable("TempEMS3"),
        storeManager.getWritable("TempEMS4"),
        storeManager.getWritable("TempEMS5"),
        storeManager.getWritable("TempEMS6"),
        storeManager.getWritable("TempEMS7"),
        storeManager.getWritable("TempEMS8"),
    ]
    let hemsTemps = [
        storeManager.getWritable("TempHEMS1"),
        storeManager.getWritable("TempHEMS2"),
        storeManager.getWritable("TempHEMS3"),
        storeManager.getWritable("TempHEMS4"),
        storeManager.getWritable("TempHEMS5"),
        storeManager.getWritable("TempHEMS6"),
        storeManager.getWritable("TempHEMS7"),
        storeManager.getWritable("TempHEMS8"),
    ]
    let leftMotorTemps = [
        storeManager.getWritable("TempMotorLeft0"),
        storeManager.getWritable("TempMotorLeft1"),
        storeManager.getWritable("TempMotorLeft2"),
        storeManager.getWritable("TempMotorLeft3"),
        storeManager.getWritable("TempMotorLeft4"),
        storeManager.getWritable("TempMotorLeft5"),
        storeManager.getWritable("TempMotorLeft6"),
        storeManager.getWritable("TempMotorLeft7"),
    ]
    let rightMotorTemps = [
        storeManager.getWritable("TempMotorRight0"),
        storeManager.getWritable("TempMotorRight1"),
        storeManager.getWritable("TempMotorRight2"),
        storeManager.getWritable("TempMotorRight3"),
        storeManager.getWritable("TempMotorRight4"),
        storeManager.getWritable("TempMotorRight5"),
        storeManager.getWritable("TempMotorRight6"),
        storeManager.getWritable("TempMotorRight7"),
    ]

    leftMotorTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(leftMotorTempsAcknowledged)) {
                leftMotorTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: "Temperature on the left motor is too high!",
                    background: "bg-error-400",
                    autohide: false,
                    callback: response => {
                        if (response.status == 'closed') {
                            leftMotorTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        })
    });
    rightMotorTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(rightMotorTempsAcknowledged)) {
                rightMotorTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: "Temperature on the right motor is too high!",
                    background: "bg-error-400",
                    autohide: false,
                    callback: response => {
                        if (response.status == 'closed') {
                            rightMotorTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        })
    });
    emsTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(emsTempsAcknowledged)) {
                emsTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: "Temperature on EMS is too high!",
                    background: "bg-error-400",
                    autohide: false,
                    callback: response => {
                        if (response.status == 'closed') {
                            emsTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        })
    });
    hemsTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(hemsTempsAcknowledged)) {
                hemsTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: "Temperature on HEMS is too high!",
                    background: "bg-error-400",
                    autohide: false,
                    callback: response => {
                        if (response.status == 'closed') {
                            hemsTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        })
    });
    propInitFault1.subscribe((store) => {
        if (store.value !== 255 && get(propInitFault1Acknowledged)) {
            propInitFault1Acknowledged.set(false);
            toastStore.trigger({
                message: `PropInitFault 1: ${store.value}`,
                background: "bg-error-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        propInitFault1Acknowledged.set(true);
                    }
                },
            });
            console.error(`PropInitFault 1: ${store.value}`);
            util.log(`PropInitFault 1: ${store.value}`, EventChannel.ERROR);
        }
    });
    propInitFault2.subscribe((store) => {
        if (store.value !== 255 && get(propInitFault2Acknowledged)) {
            propInitFault2Acknowledged.set(false);
            toastStore.trigger({
                message: `PropInitFault 2: ${store.value}`,
                background: "bg-error-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        propInitFault2Acknowledged.set(true);
                    }
                },
            });
            console.error(`PropInitFault 2: ${store.value}`);
            util.log(`PropInitFault 2: ${store.value}`, EventChannel.ERROR);
        }
    });
    propEmergency1.subscribe((store) => {
        if (store.value !== 0 && get(propEmergency1Acknowledged)) {
            propEmergency1Acknowledged.set(false);
            toastStore.trigger({
                message: `Prop Emergency 1: ${store.value}`,
                background: "bg-error-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        propEmergency1Acknowledged.set(true);
                    }
                },
            });
            console.error(`Prop Emergency 1: ${store.value}`);
            util.log(`Prop Emergency 1: ${store.value}`, EventChannel.ERROR);
        }
    });
    propEmergency2.subscribe((store) => {
        if (store.value !== 0 && get(propEmergency2Acknowledged)) {
            propEmergency2Acknowledged.set(false);
            toastStore.trigger({
                message: `Prop Emergency 2: ${store.value}`,
                background: "bg-error-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        propEmergency2Acknowledged.set(true);
                    }
                },
            });
            console.error(`Prop Emergency 2: ${store.value}`);
            util.log(`Prop Emergency 2: ${store.value}`, EventChannel.ERROR);
        }
    });

    emergency.subscribe((store) => {
        if (store.value !== 0 && !get(emergencyModalActive)) {
            console.log("Great!");
            emergencyModalActive.set(true);
            const sources: String[] = [
                "General",
                "Propulsion",
                "Levitation",
                "Powertrain Controller",
                "SenseCon",
            ]
            modalTitle.set(`${sources[store.value - 1]} Emergency!`);
            modalBody.set(
                `Emergency triggered: ${sources[store.value - 1]} Emergency!
                The Main PCB attempted to turn off high voltage with a message on the CAN bus.
                Always double check if it succeeded.`
            );
            modalStore.trigger(MODAL_SETTINGS);
            console.error(`Emergency triggered with source ${store.value - 1}!`);
            util.log(`Emergency triggered: ${sources[store.value - 1]} Emergency!`, EventChannel.ERROR);
        }
    })

    // --- Ported logic from PneumaticsTab.svelte ---
    const BRAKES_DEPLOYED_STATES = [0, 1, 2, 3, 4, 9, 10, 11, 12]; // boot, connected to gs, system check, idle, precharge, discharge, braking, charging, fault
    let brakePressureAcknowledged = true;
    let brakeWarningAcknowledged = true;
    let brakeDeployedAcknowledged = true;
    let brakeFaultAcknowledged = true;
    let brakeEmergencyActive = false;
    let lastFsmState = 0;
    let brakeShouldBeDeployedErrorAcknowledged = true;
    let brakePressureWarningActive = false;
    let lastPressure = 1000;

    const pressureBrakes = storeManager.getWritable("PressureBrakes1");
    const fsmState = storeManager.getWritable("FSMState");

    // Initialize FSM state
    const currentFsmState = get(fsmState);
    if (
        currentFsmState === undefined ||
        currentFsmState === null ||
        (typeof currentFsmState === 'object' && (currentFsmState as any).value === undefined || (currentFsmState as any).value === null)
    ) {
        lastFsmState = 0;
    } else {
        lastFsmState = (typeof currentFsmState === 'object' && 'value' in currentFsmState)
            ? (currentFsmState as any).value as number
            : currentFsmState as number;
    }

    fsmState.subscribe((val: any) => {
        let newState = val.value ?? val;
        if (newState === undefined || newState === null) {
            newState = 0;
        }
        lastFsmState = newState as number;
    });

    pressureBrakes.subscribe(async (val: any) => {
        const pressure = val.value ?? val;
        let fsmStateForLogic = lastFsmState;
        if (fsmStateForLogic === undefined || fsmStateForLogic === null) {
            fsmStateForLogic = 0;
        }
        const brakesShouldBeDeployed = BRAKES_DEPLOYED_STATES.includes(fsmStateForLogic);
        const brakesDeployed = pressure < 1;
        const brakesRetracted = pressure > 50;
        // --- Emergency: below 10, only if brakes should NOT be deployed ---
        if (pressure < 10 && brakePressureAcknowledged && !brakesShouldBeDeployed) {
            brakePressureAcknowledged = false;
            brakeEmergencyActive = true;
            try {
                await invoke('send_command', {cmdName: "EmergencyBrake", val: 0});
                util.log("EmergencyBrake triggered due to low brake pressure!", EventChannel.ERROR);
            } catch (e) {
                console.error("Failed to trigger EmergencyBrake:", e);
            }
            toastStore.trigger({
                message: "BRAKE PRESSURE CRITICAL! Emergency brake triggered. Pressure: " + pressure.toFixed(1) + " bar. Check brakes immediately!",
                background: "bg-error-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        brakeEmergencyActive = false;
                        if (pressure > 12) brakePressureAcknowledged = true;
                    }
                },
            });
        } else if (pressure < 30 && brakeWarningAcknowledged && pressure >= 10 && !brakesShouldBeDeployed) {
            brakeWarningAcknowledged = false;
            toastStore.trigger({
                message: "Brake pressure low! Pressure: " + pressure.toFixed(1) + " bar.",
                background: "bg-warning-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        if (pressure > 32) brakeWarningAcknowledged = true;
                    }
                },
            });
        } else if (pressure > 32) {
            brakePressureAcknowledged = true;
            brakeWarningAcknowledged = true;
        }
        // --- Orange warning if pressure drops below 30 (transition) ---
        if (lastPressure >= 30 && pressure < 30 && !brakePressureWarningActive) {
            brakePressureWarningActive = true;
            toastStore.trigger({
                message: 'WARNING: Brake pressure has dropped below 30 bar. Possible leak detected.',
                background: 'bg-warning-400',
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        if (pressure > 32) brakePressureWarningActive = false;
                    }
                },
            });
        } else if (pressure > 32) {
            brakePressureWarningActive = false;
        }
        lastPressure = pressure;
        // --- Brakes deployed popup (orange) ---
        if (brakesDeployed && brakeDeployedAcknowledged) {
            brakeDeployedAcknowledged = false;
            toastStore.trigger({
                message: "Brakes deployed.",
                background: "bg-warning-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        brakeDeployedAcknowledged = true;
                    }
                },
            });
        } else if (!brakesDeployed) {
            brakeDeployedAcknowledged = true;
        }
        // --- Brakes deployed in wrong FSM state (red) ---
        if (brakesDeployed && !brakesShouldBeDeployed && brakeFaultAcknowledged && !brakeEmergencyActive) {
            brakeFaultAcknowledged = false;
            toastStore.trigger({
                message: `FAULT: Brakes are deployed in FSM state ${fsmStateForLogic} where they should NOT be! Check for leaks or system issues.`,
                background: "bg-error-400",
                autohide: false,
                callback: response => {
                    if (response.status == 'closed') {
                        brakeFaultAcknowledged = true;
                    }
                },
            });
        } else if (!brakesDeployed || brakesShouldBeDeployed) {
            brakeFaultAcknowledged = true;
        }
        // --- Brakes should be deployed but are NOT (red error) ---
        if (brakesShouldBeDeployed && !brakesDeployed && brakeShouldBeDeployedErrorAcknowledged) {
            brakeShouldBeDeployedErrorAcknowledged = false;
            toastStore.trigger({
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
    });
}
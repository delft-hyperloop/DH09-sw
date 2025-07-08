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
    const localization = storeManager.getWritable("Localization");
    const emergencyStaleData = storeManager.getWritable("EmergencyStaleCriticalData");

    emergencyStaleData.subscribe(async (store) => {
        if (store.value !== 0) {
            let datatype: string = await invoke("get_datatype_by_id", { id: store.value });
            console.error("Stale critical data emergency with id " + store.value);
            modalBody.set(`${datatype} has been stale for more than one second! The main PCB went into the Fault state!`);
            modalTitle.set("Stale critical datatype!")
            modalStore.trigger(MODAL_SETTINGS);
        }
    })

    localization.subscribe((store) => {
        // console.log(`Style: ${store.style}`);
        console.log(`upper: ${store.upper}`);
    })

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
            emergencyModalActive.set(true);
            const sources: String[] = [
                "General",
                "Propulsion",
                "Levitation",
                "Powertrain Controller",
                "SenseCon",
                "Disconnection",
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
    });
}
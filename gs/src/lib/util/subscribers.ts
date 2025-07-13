import { EventChannel, GrandDataDistributor, util } from '$lib';
import { getModalStore, getToastStore } from '@skeletonlabs/skeleton';
import { invoke } from '@tauri-apps/api/tauri';
import {
    emergencySources,
    lastHeartbeatTimestamp,
    modalBody,
    modalTitle,
    staleCriticalDatatypes,
} from '$lib/stores/data';
import { EBSStates, leviErrorMessages, MODAL_SETTINGS } from '$lib/types';
import {
    ebsState,
    emergencyModalActive,
    emsTempsAcknowledged,
    hemsTempsAcknowledged,
    leftMotorTempsAcknowledged,
    propEmergency1Acknowledged,
    propEmergency2Acknowledged,
    propInitFault1Acknowledged,
    propInitFault2Acknowledged,
    rightMotorTempsAcknowledged,
    stalePopupActive,
} from '$lib/stores/state';
import { get } from 'svelte/store';

export function addEmergencySource(source: string) {
    let tempEmergencySources = get(emergencySources);
    if (!tempEmergencySources.includes(source)) {
        tempEmergencySources.push(source);
    }
    emergencySources.set(tempEmergencySources);
}

export function registerSubscribers() {
    const storeManager = GrandDataDistributor.getInstance().stores;
    const modalStore = getModalStore();
    const toastStore = getToastStore();

    const fsmTransitionFail = storeManager.getWritable('FSMTransitionFail');
    const propInitFault1 = storeManager.getWritable('PPInitFault1');
    const propEmergency1 = storeManager.getWritable('PPEmergency1');
    const propInitFault2 = storeManager.getWritable('PPInitFault2');
    const propEmergency2 = storeManager.getWritable('PPEmergency2');
    const heartbeat = storeManager.getWritable('FrontendHeartbeating');
    const emergency = storeManager.getWritable('Emergency');
    const emergencyStaleData = storeManager.getWritable(
        'EmergencyStaleCriticalData'
    );
    const lowPressure = storeManager.getWritable('PressureLow');
    const leviFault = storeManager.getWritable('LeviFault');
    const leviFaultDriveNumber = storeManager.getWritable(
        'LeviFaultDriveNumber'
    );

    const lowPressureThreshold: number = 30;

    lowPressure.subscribe((store) => {
        if (store.value > lowPressureThreshold) {
            ebsState.set(EBSStates.Armed);
        } else {
            ebsState.set(EBSStates.Triggered);
        }
    });

    leviFault.subscribe((store) => {
        if (store.value !== 0) {
            let drive = get(leviFaultDriveNumber);
            let leviErrorMessage = leviErrorMessages.filter(
                (x, index) => ((store.value >> index) & 1) === 1
            );

            const leviFaultMessage = `Levitation drive ${drive} signaled a fault with message: ${leviErrorMessage.join(', ')}`;

            addEmergencySource(
                `Levi drive ${drive}: ${leviErrorMessage.join(', ')}`
            );

            modalTitle.set('Levi Fault!');
            modalBody.set(leviFaultMessage);
            console.error(leviFaultMessage);
            modalStore.trigger(MODAL_SETTINGS);
        }
    });

    emergencyStaleData.subscribe(async (store) => {
        if (store.value !== 0) {
            stalePopupActive.set(true);
            let datatype: string = await invoke('get_datatype_by_id', {
                id: store.value,
            });
            let temp = get(staleCriticalDatatypes);
            if (!temp.includes(datatype)) {
                temp.push(datatype);

                addEmergencySource(`${datatype} Stale`);

                staleCriticalDatatypes.set(temp);
                if (temp.length > 1) {
                    modalBody.set(
                        `${get(staleCriticalDatatypes).join(', ')} have been stale for more than one second! The main PCB went into the Fault state and triggered an emergency brake!`
                    );
                } else {
                    modalBody.set(
                        `${temp.join(', ')} has been stale for more than one second! The main PCB went into the Fault state and triggered an emergency brake!`
                    );
                }
                console.error(
                    'Stale critical data emergency with id ' + store.value
                );
                modalTitle.set('Stale critical datatype!');
                if (!get(stalePopupActive)) {
                    modalStore.trigger(MODAL_SETTINGS);
                }
            }
        }
    });

    fsmTransitionFail.subscribe(async (store) => {
        let state: string = await invoke('get_fsm_state_by_index', {
            index: store.value,
        });
        if (state !== 'UnknownState') {
            toastStore.trigger({
                message: `Transition to state ${state} failed!`,
                background: 'bg-error-400',
                autohide: false,
            });
            console.error(`Transition to state ${state} failed!`);
            util.log(
                `Transition to state ${state} failed!`,
                EventChannel.ERROR
            );
        }
    });

    heartbeat.subscribe(() => {
        lastHeartbeatTimestamp.set(Date.now());
    });

    let emsTemps = [
        storeManager.getWritable('TempEMS1'),
        storeManager.getWritable('TempEMS2'),
        storeManager.getWritable('TempEMS3'),
        storeManager.getWritable('TempEMS4'),
        storeManager.getWritable('TempEMS5'),
        storeManager.getWritable('TempEMS6'),
        storeManager.getWritable('TempEMS7'),
        storeManager.getWritable('TempEMS8'),
    ];
    let hemsTemps = [
        storeManager.getWritable('TempHEMS1'),
        storeManager.getWritable('TempHEMS2'),
        storeManager.getWritable('TempHEMS3'),
        storeManager.getWritable('TempHEMS4'),
        storeManager.getWritable('TempHEMS5'),
        storeManager.getWritable('TempHEMS6'),
        storeManager.getWritable('TempHEMS7'),
        storeManager.getWritable('TempHEMS8'),
    ];
    let leftMotorTemps = [
        storeManager.getWritable('TempMotorLeft0'),
        storeManager.getWritable('TempMotorLeft1'),
        storeManager.getWritable('TempMotorLeft2'),
        storeManager.getWritable('TempMotorLeft3'),
        storeManager.getWritable('TempMotorLeft4'),
        storeManager.getWritable('TempMotorLeft5'),
        storeManager.getWritable('TempMotorLeft6'),
        storeManager.getWritable('TempMotorLeft7'),
    ];
    let rightMotorTemps = [
        storeManager.getWritable('TempMotorRight0'),
        storeManager.getWritable('TempMotorRight1'),
        storeManager.getWritable('TempMotorRight2'),
        storeManager.getWritable('TempMotorRight3'),
        storeManager.getWritable('TempMotorRight4'),
        storeManager.getWritable('TempMotorRight5'),
        storeManager.getWritable('TempMotorRight6'),
        storeManager.getWritable('TempMotorRight7'),
    ];

    leftMotorTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(leftMotorTempsAcknowledged)) {
                leftMotorTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: 'Temperature on the left motor is too high!',
                    background: 'bg-error-400',
                    autohide: false,
                    callback: (response) => {
                        if (response.status == 'closed') {
                            leftMotorTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        });
    });
    rightMotorTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(rightMotorTempsAcknowledged)) {
                rightMotorTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: 'Temperature on the right motor is too high!',
                    background: 'bg-error-400',
                    autohide: false,
                    callback: (response) => {
                        if (response.status == 'closed') {
                            rightMotorTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        });
    });
    emsTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(emsTempsAcknowledged)) {
                emsTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: 'Temperature on EMS is too high!',
                    background: 'bg-error-400',
                    autohide: false,
                    callback: (response) => {
                        if (response.status == 'closed') {
                            emsTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        });
    });
    hemsTemps.forEach((store) => {
        store.subscribe((value) => {
            if (value.value >= 60 && get(hemsTempsAcknowledged)) {
                hemsTempsAcknowledged.set(false);
                toastStore.trigger({
                    message: 'Temperature on HEMS is too high!',
                    background: 'bg-error-400',
                    autohide: false,
                    callback: (response) => {
                        if (response.status == 'closed') {
                            hemsTempsAcknowledged.set(true);
                        }
                    },
                });
            }
        });
    });
    propInitFault1.subscribe((store) => {
        if (store.value !== 255 && get(propInitFault1Acknowledged)) {
            propInitFault1Acknowledged.set(false);
            toastStore.trigger({
                message: `PropInitFault 1: ${store.value}`,
                background: 'bg-error-400',
                autohide: false,
                callback: (response) => {
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
                background: 'bg-error-400',
                autohide: false,
                callback: (response) => {
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

            addEmergencySource('Propulsion Motor 1');

            toastStore.trigger({
                message: `Prop Emergency 1: ${store.value}`,
                background: 'bg-error-400',
                autohide: false,
                callback: (response) => {
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

            addEmergencySource('Propulsion Motor 2');

            toastStore.trigger({
                message: `Prop Emergency 2: ${store.value}`,
                background: 'bg-error-400',
                autohide: false,
                callback: (response) => {
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

            // Stale critical data emergency should be handled in a different modal to
            // also show the datapoint that cause the emergency. Therefore, it is missing
            // from here. (Hint: check above)
            const sources: string[] = [
                'General',
                'Propulsion',
                'Levitation',
                'Powertrain Controller',
                'BMS',
                'SenseCon',
                'SensorHub',
                'Disconnection',
                'Wrong EBS State',
            ];

            addEmergencySource(sources[store.value - 1]);

            modalTitle.set(`${sources[store.value - 1]} Emergency!`);
            modalBody.set(
                `Emergency triggered: ${sources[store.value - 1]} Emergency!
                The Main PCB attempted to turn off high voltage with a message on the CAN bus.
                Always double check if it succeeded.`
            );
            modalStore.trigger(MODAL_SETTINGS);
            console.error(
                `Emergency triggered with source ${store.value - 1}!`
            );
            util.log(
                `Emergency triggered: ${sources[store.value - 1]} Emergency!`,
                EventChannel.ERROR
            );
        }
    });
}

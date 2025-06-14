<script lang="ts">
    import { getModalStore, getToastStore, Tab, TabGroup } from '@skeletonlabs/skeleton';
    import {
        HomeTab,
        ProceduresTab,
        PropulsionTab,
        detailTabSet,
        LeviTab,
        PneumaticsTab,
        PowertrainTab,
        DebugTab,
        GrandDataDistributor,
        util,
        EventChannel,
    } from '$lib';
    import {
        connectedToMainPCB,
        connectionAcknowledged,
        debugModeActive,
        emsTempsAcknowledged,
        hemsTempsAcknowledged,
        leftMotorTempsAcknowledged,
        logsPanelSize,
        propEmergency1Acknowledged,
        propEmergency2Acknowledged,
        propInitFault1Acknowledged,
        propInitFault2Acknowledged,
        rightMotorTempsAcknowledged,
    } from '$lib/stores/state';
    import { MODAL_SETTINGS } from '$lib/types';
    import { lastHeartbeatTimestamp, modalBody, modalTitle } from '$lib/stores/data';
    import { invoke } from '@tauri-apps/api/tauri';

    let i = 0;
    let tabs = [
        {name: "Home", value: i++},
        {name: "Propulsion", value: i++},
        {name: "Powertrain", value: i++},
        {name: "Levitation", value: i++},
        // {name: "Location", value: i++},
        {name: "Pneumatics", value: i++},
        {name: "Procedures", value: i++},
        {name: "Debug", value: i++}
    ];

    let style: string;
    $: style = `height: ${100 - ($logsPanelSize + 8.2 - 0.04 * $logsPanelSize)}vh`;

    const storeManager = GrandDataDistributor.getInstance().stores;
    const modalStore = getModalStore();
    const toastStore = getToastStore();
    const propInitFault1 = storeManager.getWritable("PPInitFault1");
    const propEmergency1 = storeManager.getWritable("PPEmergency1");
    const propInitFault2 = storeManager.getWritable("PPInitFault2");
    const propEmergency2 = storeManager.getWritable("PPEmergency2");
    const heartbeat = storeManager.getWritable("FrontendHeartbeating");
    const fsmTransitionFail = storeManager.getWritable("FSMTransitionFail");
    const emergency = storeManager.getWritable("Emergency");

    heartbeat.subscribe(() => {
        lastHeartbeatTimestamp.set(Date.now());
    })

    // let firstPass = true;
    // connectedToMainPCB.subscribe(() => {
    //     if (!$connectedToMainPCB) {
    //         if (!firstPass && $connectionAcknowledged) {
    //             // modalBody.set('The main PCB disconnected from the groundstation! Make sure to reconnect before continuing!');
    //             // modalTitle.set('Disconnected from the pod!');
    //             // modalStore.trigger(MODAL_SETTINGS);
    //
    //             connectionAcknowledged.set(false);
    //             toastStore.trigger({
    //                 message: "The main PCB disconnected from the Groundstation!",
    //                 background: "bg-error-400",
    //                 autohide: false,
    //                 callback: response => {
    //                     if (response.status == 'closed') {
    //                         connectionAcknowledged.set(true);
    //                     }
    //                 },
    //             });
    //         }
    //         firstPass = false;
    //     }
    // })

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
            if (value.value >= 60 && $leftMotorTempsAcknowledged) {
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
            if (value.value >= 60 && $rightMotorTempsAcknowledged) {
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
            if (value.value >= 60 && $emsTempsAcknowledged) {
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
            if (value.value >= 60 && $hemsTempsAcknowledged) {
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
        if (store.value !== 255 && $propInitFault1Acknowledged) {
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
        if (store.value !== 255 && $propInitFault2Acknowledged) {
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
        if (store.value !== 0 && $propEmergency1Acknowledged) {
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
        if (store.value !== 0 && $propEmergency2Acknowledged) {
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

    emergency.subscribe((store) => {
        if (store.value !== 0) {
            let sources: String[] = [
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

</script>

<TabGroup regionPanel="m-0 !mt-0" padding="px-3 py-3" regionList="bg-surface-700" border="border-b border-surface-900" >
    {#each tabs as tab}
        {#if $debugModeActive || (!$debugModeActive && tab.name !== "Debug")}
            <Tab bind:group={$detailTabSet} value={tab.value} name={tab.name}>
                <span>{tab.name}</span>
            </Tab>
        {/if}
    {/each}
    <svelte:fragment slot="panel">
        <div style={style} class="overflow-y-scroll scroll-smooth">
            <div class="h-full">
                {#if $detailTabSet === 0}
                    <HomeTab />
                {:else if $detailTabSet === 1}
                    <PropulsionTab />
                {:else if $detailTabSet === 2}
                    <PowertrainTab />
                {:else if $detailTabSet === 3}
                    <LeviTab />
                {:else if $detailTabSet === 4}
                    <PneumaticsTab />
                {:else if $detailTabSet === 5}
                    <ProceduresTab />
                {:else if $detailTabSet === 6}
                    <DebugTab />
                {/if}
            </div>
        </div>
    </svelte:fragment>
</TabGroup>

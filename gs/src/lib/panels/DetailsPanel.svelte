<script lang="ts">
    import { run } from 'svelte/legacy';

    import { Tab, Tabs } from '@skeletonlabs/skeleton-svelte';
    import {
        HomeTab,
        ProceduresTab,
        RunInitTab,
        detailTabSet,
        LocationTab,
        LeviTab,
        PneumaticsTab,
        BatteriesTab,
        DebugTab, GrandDataDistributor, util, EventChannel,
    } from '$lib';
    import { debugModeActive, logsPanelSize } from '$lib/stores/state';
    import { MODAL_SETTINGS } from '$lib/types';

    let i = 0;
    let tabs = [
        {name: "Home", value: i++},
        {name: "Run", value: i++},
        {name: "Location", value: i++},
        {name: "Batteries", value: i++},
        {name: "Levitation", value: i++},
        {name: "Pneumatics", value: i++},
        {name: "Procedures", value: i++},
        {name: "Debug", value: i++}
    ];

    let style: string = $derived(`height: ${100 - ($logsPanelSize + 8.2 - 0.04 * $logsPanelSize)}vh`);
    

    const storeManager = GrandDataDistributor.getInstance().stores;
    const modalStore = getModalStore();
    const propInitFault1 = storeManager.getWritable("PPInitFault1");
    const propEmergency1 = storeManager.getWritable("PPEmergency1");
    const propInitFault2 = storeManager.getWritable("PPInitFault2");
    const propEmergency2 = storeManager.getWritable("PPEmergency2");

    run(() => {
        // TODO: Replace all with subscribers
        let faultValue1 = $propInitFault1.value
        if (faultValue1 !== 256) {
            console.error(`PropInitFault1: ${faultValue1}`);
            util.log(`PropInitFault1: ${faultValue1}`, EventChannel.ERROR);
        }
        let faultValue2 = $propInitFault2.value
        if (faultValue2 !== 256) {
            console.error(`PropInitFault2: ${faultValue2}`);
            util.log(`PropInitFault2: ${faultValue2}`, EventChannel.ERROR);
        }

        let propEmergencyValue1 = $propEmergency1.value;
        if (propEmergencyValue1 !== 0) {
            console.error(`PropEmergency1: ${propEmergencyValue1}`);
            util.log(`PropEmergency1: ${propEmergencyValue1}`, EventChannel.ERROR);

            MODAL_SETTINGS.body = `Propulsion motor drive 1 sent an emergency message: ${propEmergencyValue1}`;
            MODAL_SETTINGS.title = "Propulsion Fault!";
            modalStore.trigger(MODAL_SETTINGS);
        }
        let propEmergencyValue2 = $propEmergency2.value;
        if (propEmergencyValue2 !== 0) {
            console.error(`PropEmergency2: ${propEmergencyValue2}`);
            util.log(`PropEmergency2: ${propEmergencyValue2}`, EventChannel.ERROR);

            MODAL_SETTINGS.body = `Propulsion motor drive 2 sent an emergency message: ${propEmergencyValue2}`;
            MODAL_SETTINGS.title = "Propulsion Fault!";
            modalStore.trigger(MODAL_SETTINGS);
        }
    });
</script>

<Tabs regionPanel="m-0 mt-0!" padding="px-3 py-3" regionList="bg-surface-700" border="border-b border-surface-900" >
    {#each tabs as tab}
        {#if $debugModeActive || (!$debugModeActive && tab.name !== "Debug")}
            <Tab bind:group={$detailTabSet} value={tab.value} name={tab.name}>
                <span>{tab.name}</span>
            </Tab>
        {/if}
    {/each}
    {#snippet panel()}
    
            <div style={style} class="overflow-y-scroll scroll-smooth">
                <div class="h-full">
                    {#if $detailTabSet === 0}
                        <HomeTab />
                    {:else if $detailTabSet === 1}
                        <RunInitTab />
                    {:else if $detailTabSet === 2}
                        <LocationTab />
                    {:else if $detailTabSet === 3}
                        <BatteriesTab />
                    {:else if $detailTabSet === 4}
                        <LeviTab />
                    {:else if $detailTabSet === 5}
                        <PneumaticsTab />
                    {:else if $detailTabSet === 6}
                        <ProceduresTab />
                    {:else if $detailTabSet === 7}
                        <DebugTab />
                    {/if}
                </div>
            </div>
        
    {/snippet}
</Tabs>

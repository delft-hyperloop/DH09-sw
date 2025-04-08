<script lang="ts">
    import { getModalStore, Tab, TabGroup } from '@skeletonlabs/skeleton';
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

    let style: string;
    $: style = `height: ${100 - ($logsPanelSize + 8.2 - 0.04 * $logsPanelSize)}vh`;

    const storeManager = GrandDataDistributor.getInstance().stores;
    // const modalStore = getModalStore();
    const propInitFault = storeManager.getWritable("PPInitFault");
    const propEmergency = storeManager.getWritable("PPEmergency");

    $: {
        let faultValue = $propInitFault.value
        if (faultValue !== 2**7 - 2) {
            console.error(`PropInitFault not whatever: ${faultValue}`);
            util.log(`PropInitFault not whatever: ${faultValue}`, EventChannel.ERROR);
        }

        let propEmergencyValue = $propEmergency.value;
        if (propEmergencyValue !== 0) {
            console.error(`PropEmergency: ${propEmergencyValue}`);
            util.log(`PropEmergency: ${propEmergencyValue}`, EventChannel.ERROR);
        }
        // TODO: Modal popup
    }
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
    </svelte:fragment>
</TabGroup>

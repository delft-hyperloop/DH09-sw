<script lang="ts">
    import { getModalStore, getToastStore, Tab, TabGroup } from '@skeletonlabs/skeleton';
    import {
        HomeTab,
        ProceduresTab,
        PropulsionTab,
        detailTabSet,
        LocationTab,
        LeviTab,
        PneumaticsTab,
        PowertrainTab,
        DebugTab,
        GrandDataDistributor,
        util,
        EventChannel,
    } from '$lib';
    import {
        debugModeActive, emergencyModalActive,
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

    let i = 0;
    let tabs = [
        {name: "Home", value: i++},
        {name: "Powertrain", value: i++},
        {name: "Levitation", value: i++},
        {name: "Propulsion", value: i++},
        {name: "Location", value: i++},
        {name: "Pneumatics", value: i++},
        {name: "Procedures", value: i++},
        {name: "Debug", value: i++}
    ];

    let style: string;
    $: style = `height: ${100 - ($logsPanelSize + 8.2 - 0.04 * $logsPanelSize)}vh`;

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
                    <PowertrainTab />
                {:else if $detailTabSet === 2}
                    <LeviTab />
                {:else if $detailTabSet === 3}
                    <PropulsionTab />
                {:else if $detailTabSet === 4}
                    <LocationTab />
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

<script lang="ts">
    import {
        Battery,
        Chart,
        Command,
        GrandDataDistributor,
        Tile,
        TileGrid,
    } from '$lib';
    import { imdWarnings, ptcErrorCodes, ptcStates } from '$lib/types';
    import ValueStore from '$lib/components/generic/ValueStore.svelte';
    import { ChargingStation, Flash, FlashOff, StopOutline } from 'carbon-icons-svelte';
    import { inStateIdle, ptResetMessage } from '$lib/stores/state';
    import BinaryInput from '$lib/components/BinaryInput.svelte';
    import { inStateActive, inStateCharging } from '$lib/stores/state.js';
    const storeManager = GrandDataDistributor.getInstance().stores;

    const ptcState = storeManager.getWritable("PTCState");
    const ptcFaultStore = storeManager.getWritable("PTCErrors");
    const imdWarningStore = storeManager.getWritable("IMDWarnings");

    $: ptcFaultMessage = ptcErrorCodes.filter((x, index) =>
        ((($ptcFaultStore.value >> index - 1) & 1) == 1)
    );

    $: imdWarningMessage = imdWarnings.filter((x, index) =>
        ((($imdWarningStore.value >> index - 1) & 1) == 1)
    );

    const resetPowertrainLabels = [
        "Precharge",
        "Overvoltage",
        "Undervoltage",
        "Overtemperature",
        "Undertemperature",
    ]

    export const pop_up: boolean = true;
</script>

<div class="p-4">
    <h2 class="text-2xl font-semibold mb-4">Batteries</h2>
    <div class="flex gap-x-2 items-start">
    </div>
    <TileGrid columns="1fr 1fr 1fr 1fr 1fr" rows="auto">
        <Tile containerClass="col-span-full">
            <div class="flex flex-row gap-4 w-full h-full">
                <Command
                    cmd="StartHV"
                    text="Start HV"
                    icon={Flash}
                    dependency={inStateIdle}
                    dependencyTitle="Wrong State!"
                    dependencyMessage="The pod should be in the Idle state to turn on high voltage!"
                />
                <Command cmd="StopHV" text="Stop HV" className="text-error-400 border-error-400 border-2" icon={FlashOff}/>
                <Command cmd="Charge" icon={ChargingStation} dependency={inStateActive}/>
                <Command cmd="StopCharge" icon={StopOutline} dependency={inStateCharging}/>
            </div>
        </Tile>
        <Tile containerClass="col-span-full px-10">
            <div class="grid grid-rows-2 grid-cols-7 gap-2 h-full items-center">
                <span class="text-wrap text-center">Powertrain Reset Flags</span>
                {#each Array.from({ length: resetPowertrainLabels.length }, (_, i) => resetPowertrainLabels.length - 1 - i) as i}
                    <BinaryInput store={ptResetMessage} index={i}/>
                {/each}
                <Command cmd="ResetPowertrain" val={$ptResetMessage} text="Reset"/>
                <span/>
                {#each resetPowertrainLabels as l}
                    <span class="text-center">{l}</span>
                {/each}
                <span/>
            </div>
        </Tile>
        <Tile containerClass="col-span-full">
            <div class="flex flex-row gap-4 items-center">
                <div class="flex flex-col mx-8 gap-2">
                    <div class="flex flex-col items-center">
                        <Battery fill="#3b669c" orientation="horizontal" height={40} perc={0} />
                        <!--                <Battery fill="#3b669c" orientation="horizontal" height={40} perc={Number($lvBattery.value)} />-->
                        <p>Low voltage</p>
                    </div>
                    <div class="flex flex-col items-center">
                        <Battery fill="#723f9c" orientation="horizontal" height={40} perc={0} />
                        <!--                <Battery fill="#723f9c" orientation="horizontal" height={40} perc={Number($hvBattery.value)} />-->
                        <p>High voltage</p>
                    </div>
                </div>
                <div class="flex flex-col gap-4 ">
                <span>
                    PT Controller State:
                    <ValueStore
                        value={ptcStates[$ptcState.value]}
                        timestamp={$ptcState.timestamp}
                    />
                </span>
                    <span>
                    PT Controller Fault:
                    <ValueStore
                        value={ptcFaultMessage.length === 0 ? "None" : ptcFaultMessage.join(", ")}
                        timestamp={$ptcFaultStore.timestamp}
                    />
                </span>
                    <span>
                    IMD Warning:
                    <ValueStore
                        value={imdWarnings.length === 0 ? "None" : imdWarningMessage.join(", ")}
                        timestamp={$imdWarningStore.timestamp}
                    />
                </span>
                </div>
            </div>
        </Tile>
<!--        <Tile containerClass="col-span-2" insideClass="flex flex-col h-full gap-2 items-center">-->
<!--            <div class="w-full flex justify-between items-center">-->
<!--                <Status label="HV Battery relay status" onColor="text-error-400" offColor="text-surface-50"-->
<!--                        on="HV Relays ON" off="HV Relays Off" bind:status={connectorStatus} />-->
<!--                <ToggleCommand onCmd="StartHV" offCmd="StopHV" bind:status={connectorStatus} />-->
<!--            </div>-->
<!--            <div class="w-full flex justify-between items-center">-->
<!--                <Status label="DC Converter status" on="charging" off="off" offColor="text-surface-50" bind:status={dcStatus} />-->
<!--&lt;!&ndash;                <ToggleCommand onCmd="DcOn" offCmd="DcOff" bind:status={dcStatus} disabled={$lvTotalStore.value > 21} />&ndash;&gt;-->
<!--            </div>-->
<!--        </Tile>-->
<!--        <Tile insideClass="flex h-full items-center ">-->
<!--            <div class="flex flex-col ml-4">-->
<!--                <span>Area under maintenance...</span>-->
<!--&lt;!&ndash;                <p>LV Current: <Store datatype="BatteryCurrentLow" /></p>&ndash;&gt;-->
<!--&lt;!&ndash;                <p>HV Current: <Store datatype="BatteryCurrentHigh" /></p>&ndash;&gt;-->
<!--&lt;!&ndash;                <p>IMD Voltage: <Store datatype="IMDVoltageDetails" /></p>&ndash;&gt;-->
<!--            </div>-->
<!--        </Tile>-->
<!--        <Tile containerClass="col-span-4" heading="Battery stats">-->
<!--            <Table background="bg-surface-900" {tableArr} {titles} />-->
<!--        </Tile>-->
    </TileGrid>
    <TileGrid columns="1fr 1fr 1fr 1fr" rows="auto 1fr auto" className="mt-2">
        <Tile containerClass="col-span-full">
            <Chart title="BMS Voltages" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="HV Pack Voltage" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="LV Pack Voltage" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="Bus Current" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="LV Pack Current" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="DC Link Voltage" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="Isolation Resistance" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-full">
            <Chart title="BMS Temperatures" background="bg-surface-900"/>
        </Tile>
<!--        <Tile containerClass="col-span-4" heading="Lv Cell Voltages">-->
<!--            <Table background="bg-surface-900" tableArr={lvCellVoltArr} titles =  {lvTitles} />-->
<!--        </Tile>-->
<!--        <Tile containerClass="col-span-4" heading="HV Cell Voltages">-->
<!--            <Table background="bg-surface-900 text-xs" tableArr={hvCellVoltArr} titles ={hvTitles} />-->
<!--        </Tile>-->
    </TileGrid>
</div>

<script lang="ts">
    import { Chart, Command, Tile, TileGrid } from '$lib';
    import { ChargingStation, Flash, FlashOff, StopOutline } from 'carbon-icons-svelte';
    import { inStateIdle, ptResetMessage } from '$lib/stores/state';
    import { inStateActive, inStateCharging } from '$lib/stores/state.js';

    // const ptcState = storeManager.getWritable("PTCState");
    // const ptcFaultStore = storeManager.getWritable("PTCErrors");
    // const imdWarningStore = storeManager.getWritable("IMDWarnings");

    // $: ptcFaultMessage = ptcErrorCodes.filter((x, index) =>
    //     ((($ptcFaultStore.value >> index - 1) & 1) == 1)
    // );
    //
    // $: imdWarningMessage = imdWarnings.filter((x, index) =>
    //     ((($imdWarningStore.value >> index - 1) & 1) == 1)
    // );

    const resetPowertrainLabels = [
        "Precharge",
        "Overvoltage",
        "Undervoltage",
        "Overtemperature",
        "Undertemperature",
    ]

    let ptReset: number = 0;

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
            <div class="flex flex-row gap-4 h-full w-full items-center">
                <span class="text-wrap text-center">Powertrain Reset Flags</span>
                <!--{#each Array.from({ length: resetPowertrainLabels.length }, (_, i) => resetPowertrainLabels.length - 1 - i) as i}-->
                <!--    <BinaryInput store={ptResetMessage} index={i}/>-->
                <!--{/each}-->
                <input bind:value={ptReset} type="number" min="0" class="input p-4 rounded-md " on:change={() => {ptResetMessage.set(ptReset)}}>
                <Command cmd="ResetPowertrain" val={$ptResetMessage} text="Reset"/>
<!--                <span/>-->
<!--                {#each resetPowertrainLabels as l}-->
<!--                    <span class="text-center">{l}</span>-->
<!--                {/each}-->
<!--                <span/>-->
            </div>
        </Tile>
    </TileGrid>
<!--    <TileGrid columns="1fr 1fr 1fr 1fr 1fr 1fr" rows="auto" className="mt-2">-->
<!--        <Tile containerClass="col-span-1" insideClass="gap-4 items-center">-->
<!--            <div class="flex flex-col gap-2">-->
<!--                <div class="flex flex-col items-center">-->
<!--                    <Battery fill="#3b669c" orientation="horizontal" height={40} perc={0} />-->
<!--                    &lt;!&ndash;                <Battery fill="#3b669c" orientation="horizontal" height={40} perc={Number($lvBattery.value)} />&ndash;&gt;-->
<!--                    <p>Low voltage</p>-->
<!--                </div>-->
<!--                <div class="flex flex-col items-center">-->
<!--                    <Battery fill="#723f9c" orientation="horizontal" height={40} perc={0} />-->
<!--                    &lt;!&ndash;                <Battery fill="#723f9c" orientation="horizontal" height={40} perc={Number($hvBattery.value)} />&ndash;&gt;-->
<!--                    <p>High voltage</p>-->
<!--                </div>-->
<!--            </div>-->
<!--        </Tile>-->
<!--        <Tile containerClass="col-span-5">-->
<!--            <div class="flex flex-col gap-4 ">-->
<!--                <ValueStore-->
<!--                    name="PT Controller State"-->
<!--                    value={ptcStates[$ptcState.value]}-->
<!--                    timestamp={$ptcState.timestamp}-->
<!--                />-->
<!--                <ValueStore-->
<!--                    name="PT Controller Fault"-->
<!--                    value={ptcFaultMessage.length === 0 ? "None" : ptcFaultMessage.join(", ")}-->
<!--                    timestamp={$ptcFaultStore.timestamp}-->
<!--                />-->
<!--                <ValueStore-->
<!--                    name="IMD Warning"-->
<!--                    value={imdWarningMessage.length === 0 ? "None" : imdWarningMessage.join(", ")}-->
<!--                    timestamp={$imdWarningStore.timestamp}-->
<!--                />-->
<!--            </div>-->
<!--        </Tile>-->
<!--    </TileGrid>-->
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
    </TileGrid>
</div>

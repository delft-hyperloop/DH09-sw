<script lang="ts">
    import {
        Battery,
        Chart,
        GrandDataDistributor,
        Status,
        Tile,
        TileGrid,
        ToggleCommand,
    } from "$lib"
    const storeManager = GrandDataDistributor.getInstance().stores;
    // const lvBattery = storeManager.getWritable("");
    // const hvBattery = storeManager.getWritable("BMSVoltageHigh");

    let dcStatus:boolean = false;
    let connectorStatus:boolean = false;

    export const pop_up: boolean = true;
</script>

<div class="p-4">
    <h2 class="text-2xl font-semibold mb-4">Batteries</h2>
    <div class="flex gap-x-2 items-start">
    </div>
    <TileGrid columns="1fr 1fr 1fr 1fr" rows="auto 1fr auto">
        <Tile insideClass="flex h-full items-center gap-4 justify-center">
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
        </Tile>
        <Tile containerClass="col-span-2" insideClass="flex flex-col h-full gap-2 items-center">
            <div class="w-full flex justify-between items-center">
                <Status label="HV Battery relay status" onColor="text-error-400" offColor="text-surface-50"
                        on="HV Relays ON" off="HV Relays Off" bind:status={connectorStatus} />
                <ToggleCommand onCmd="StartHV" offCmd="StopHV" bind:status={connectorStatus} />
            </div>
            <div class="w-full flex justify-between items-center">
                <Status label="DC Converter status" on="charging" off="off" offColor="text-surface-50" bind:status={dcStatus} />
<!--                <ToggleCommand onCmd="DcOn" offCmd="DcOff" bind:status={dcStatus} disabled={$lvTotalStore.value > 21} />-->
            </div>
        </Tile>
        <Tile insideClass="flex h-full items-center ">
            <div class="flex flex-col ml-4">
                <span>Area under maintenance...</span>
<!--                <p>LV Current: <Store datatype="BatteryCurrentLow" /></p>-->
<!--                <p>HV Current: <Store datatype="BatteryCurrentHigh" /></p>-->
<!--                <p>IMD Voltage: <Store datatype="IMDVoltageDetails" /></p>-->
            </div>
        </Tile>
<!--        <Tile containerClass="col-span-4" heading="Battery stats">-->
<!--            <Table background="bg-surface-900" {tableArr} {titles} />-->
<!--        </Tile>-->
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

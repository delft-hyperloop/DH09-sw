<script lang="ts">
    import {
        Battery,
        Chart,
        GrandDataDistributor,
        Status,
        Store,
        Table,
        Tile,
        TileGrid, ToggleCommand,
    } from "$lib"
    import type { NamedDatatype } from "$lib/types";
    import { NamedDatatypeValues } from "$lib/types";
  const storeManager = GrandDataDistributor.getInstance().stores;
  const lvBattery = storeManager.getWritable("BMSVoltageLow");
  const hvBattery = storeManager.getWritable("BMSVoltageHigh");

  const avg1Temp = "Module1AvgTemperature", max1Temp = "Module1MaxTemperature", min1Temp = "Module1MinTemperature",
    avg1Vol = "Module1AvgVoltage", max1Vol = "Module1MaxVoltage", min1Vol = "Module1MinVoltage",
    avg2Temp = "Module2AvgTemperature", max2Temp = "Module2MaxTemperature", min2Temp = "Module2MinTemperature",
    avg2Vol = "Module2AvgVoltage", max2Vol = "Module2MaxVoltage", min2Vol = "Module2MinVoltage",
    avg3Temp = "Module3AvgTemperature", max3Temp = "Module3MaxTemperature", min3Temp = "Module3MinTemperature",
    avg3Vol = "Module3AvgVoltage", max3Vol = "Module3MaxVoltage", min3Vol = "Module3MinVoltage",
    avg4Temp = "Module4AvgTemperature", max4Temp = "Module4MaxTemperature", min4Temp = "Module4MinTemperature",
    avg4Vol = "Module4AvgVoltage", max4Vol = "Module4MaxVoltage", min4Vol = "Module4MinVoltage",
    avg5Temp = "Module5AvgTemperature", max5Temp = "Module5MaxTemperature", min5Temp = "Module5MinTemperature",
    avg5Vol = "Module5AvgVoltage", max5Vol = "Module5MaxVoltage", min5Vol = "Module5MinVoltage",
    avg6Temp = "Module6AvgTemperature", max6Temp = "Module6MaxTemperature", min6Temp = "Module6MinTemperature",
    avg6Vol = "Module6AvgVoltage", max6Vol = "Module6MaxVoltage", min6Vol = "Module6MinVoltage",
    avg7Temp = "Module7AvgTemperature", max7Temp = "Module7MaxTemperature", min7Temp = "Module7MinTemperature",
    avg7Vol = "Module7AvgVoltage", max7Vol = "Module7MaxVoltage", min7Vol = "Module7MinVoltage",
    avg8Temp = "Module8AvgTemperature", max8Temp = "Module8MaxTemperature", min8Temp = "Module8MinTemperature",
    avg8Vol = "Module8AvgVoltage", max8Vol = "Module8MaxVoltage", min8Vol = "Module8MinVoltage",
    avgLvTemp = "BatteryTemperatureLow", minLvTemp = "BatteryMinTemperatureLow", maxLvTemp = "BatteryMaxTemperatureLow",
    avgLvVol = "BatteryVoltageLow", minLvVol = "BatteryMinVoltageLow", maxLvVol = "BatteryMaxVoltageHigh";

    let dcStatus:boolean = false;
    let connectorStatus:boolean = false;

    export const pop_up: boolean = true;

    // Battery stats table
    let titles = ["Battery", "Avg cell V", "Max cell V", "Min cell V", "Avg cell °C", "Max cell °C", "Min cell °C"];
    $: tableArr = [
        ["LV", avgLvVol, maxLvVol, minLvVol, avgLvTemp, maxLvTemp, minLvTemp],
        ["HV mod 1", avg1Vol, max1Vol, min1Vol, avg1Temp, max1Temp, min1Temp],
        ["HV mod 2", avg2Vol, max2Vol, min2Vol, avg2Temp, max2Temp, min2Temp],
        ["HV mod 3", avg3Vol, max3Vol, min3Vol, avg3Temp, max3Temp, min3Temp],
        ["HV mod 4", avg4Vol, max4Vol, min4Vol, avg4Temp, max4Temp, min4Temp],
        ["HV mod 5", avg5Vol, max5Vol, min5Vol, avg5Temp, max5Temp, min5Temp],
        ["HV mod 6", avg6Vol, max6Vol, min6Vol, avg6Temp, max6Temp, min6Temp],
        ["HV mod 7", avg7Vol, max7Vol, min7Vol, avg7Temp, max7Temp, min7Temp],
        ["HV mod 8", avg8Vol, max8Vol, min8Vol, avg8Temp, max8Temp, min8Temp],
    ];

    // Helper functions for cell variable names
    function hvVoltageName(module: number, cell: number): NamedDatatype {
      return `HVCellVolt${(module - 1) * 16 + cell}` as NamedDatatype;
    }
    function hvTempName(module: number, cell: number): NamedDatatype {
      return `HVCellTemp${(module - 1) * 16 + cell}` as NamedDatatype;
    }
    function lvVoltageName(module: number, cell: number): NamedDatatype {
      return `LVCellVolt${(module - 1) * 16 + cell}` as NamedDatatype;
    }
    function lvTempName(module: number, cell: number): NamedDatatype {
      return `LVCellTemp${(module - 1) * 16 + cell}` as NamedDatatype;
    }

    // Fixed cell table generation - ensures consistent column counts
    const CELLS_PER_MODULE = 16;
    const COLUMN_COUNT = CELLS_PER_MODULE + 1; // +1 for module column

    // Generate titles with consistent count
    let hvCellTitles = ["Module", ...Array.from({length: CELLS_PER_MODULE}, (_, i) => `cell ${i+1}`)];
    
    // Generate HV cell arrays with guaranteed consistent column counts
    let hvCellVoltArr = Array.from({length: 10}, (_, m) => {
      const row = [`Module ${m+1}`];
      for (let c = 1; c <= CELLS_PER_MODULE; c++) {
        row.push(hvVoltageName(m+1, c));
      }
      return row;
    });
    
    let hvCellTempArr = Array.from({length: 10}, (_, m) => {
      const row = [`Module ${m+1}`];
      for (let c = 1; c <= CELLS_PER_MODULE; c++) {
        row.push(hvTempName(m+1, c));
      }
      return row;
    });

    // Generate LV cell arrays with consistent column counts
    let lvCellTitles = ["Module", ...Array.from({length: CELLS_PER_MODULE}, (_, i) => `cell ${i+1}`)];
    
    let lvCellVoltArr = Array.from({length: 2}, (_, m) => {
      const row = [`Module ${m+1}`];
      for (let c = 1; c <= CELLS_PER_MODULE; c++) {
        row.push(lvVoltageName(m+1, c));
      }
      return row;
    });
    
    let lvCellTempArr = Array.from({length: 2}, (_, m) => {
      const row = [`Module ${m+1}`];
      for (let c = 1; c <= CELLS_PER_MODULE; c++) {
        row.push(lvTempName(m+1, c));
      }
      return row;
    });

    // Responsive view state
    let showCompactView = false;
    let selectedModule = 1;

    // Always keep selectedModule in range
    $: if (selectedModule > 10) selectedModule = 10;
    $: if (selectedModule < 1) selectedModule = 1;

    // Compact view data - shows only selected module, fallback to empty row if out of bounds
    $: compactHvVoltArr = [hvCellVoltArr[selectedModule - 1] || Array(COLUMN_COUNT).fill("")];
    $: compactHvTempArr = [hvCellTempArr[selectedModule - 1] || Array(COLUMN_COUNT).fill("")];
    $: compactLvVoltArr = [lvCellVoltArr[selectedModule - 1] || Array(COLUMN_COUNT).fill("")];
    $: compactLvTempArr = [lvCellTempArr[selectedModule - 1] || Array(COLUMN_COUNT).fill("")];

    // Improved cell lookup state and logic
    let cellLookupType: 'HV' | 'LV' = 'HV';
    let cellLookupNum = '';
    let cellLookupInfo = null;

    function getCellInfo(type: 'HV' | 'LV', numStr: string) {
        const num = parseInt(numStr, 10);
        if (isNaN(num)) return null;
        const max = type === 'HV' ? 160 : 32;
        if (num < 1 || num > max) return null;
        const module = Math.floor((num - 1) / 16) + 1;
        const cell = ((num - 1) % 16) + 1;
        // Use correct variable names for lookup
        const voltName = type === 'HV'
            ? `SingleCellVoltageHV${num}`
            : `SingleCellVoltageLV${num}`;
        const tempName = type === 'HV'
            ? `SingleCellTemperatureHV${num}`
            : `SingleCellTemperatureLV${num}`;
        return { type, num, module, cell, voltName, tempName };
    }

    $: cellLookupInfo = getCellInfo(cellLookupType, cellLookupNum);
    $: cellLookupVoltNameTyped = cellLookupInfo ? (cellLookupInfo.voltName as NamedDatatype) : undefined;
    $: cellLookupTempNameTyped = cellLookupInfo ? (cellLookupInfo.tempName as NamedDatatype) : undefined;
</script>

<div class="p-4">
    <h2 class="text-2xl font-semibold mb-4">Batteries</h2>
    <div class="flex gap-x-2 items-start">
    </div>
    <TileGrid columns="1fr 1fr 1fr 1fr" rows="auto 1fr auto">
        <Tile insideClass="flex h-full items-center gap-4">
            <div class="flex flex-col items-center">
                <Battery fill="#3b669c" orientation="horizontal" height={40} perc={Number($lvBattery.value)} />
                <p>Low voltage</p>
            </div>
            <div class="flex flex-col items-center">
                <Battery fill="#723f9c" orientation="horizontal" height={40} perc={Number($hvBattery.value)} />
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
                <!-- <span>Area under maintenance...</span> -->
               <p>LV Current: <Store datatype="BMSVoltageLow" /></p>
         <p>HV Current: <Store datatype="BMSVoltageHigh" /></p>
                <p>IMD Voltage: <Store datatype="VDCLink" /></p>
            </div>
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="BMS Voltage High" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="BMS Voltage Low" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-4">
            <Chart title="BMS Temps" background="bg-surface-900" />
        </Tile>
        <!-- <Tile containerClass="col-span-full">
            <Chart title="BMS Logs" background="bg-surface-900"/>
        </Tile> -->
        <Tile containerClass="col-span-4" heading="Battery stats">
            <Table background="bg-surface-900" {tableArr} {titles} />
        </Tile>
        
        <!-- Improved Cell lookup UI with green dropdown and debug output -->
        <div class="mb-4 flex flex-col gap-2 max-w-md">
          <label class="font-semibold">Cell Lookup:</label>
          <div class="flex gap-2 items-center">
            <select bind:value={cellLookupType} class="px-2 py-1 rounded text-white" style="background-color: #1ec48e; color: #222; font-weight: bold;">
              <option value="HV">HV</option>
              <option value="LV">LV</option>
            </select>
            <input
              class="px-3 py-1 rounded bg-surface-700 text-white focus:outline-none focus:ring w-24"
              type="number"
              min={cellLookupType === 'HV' ? 1 : 1}
              max={cellLookupType === 'HV' ? 160 : 32}
              placeholder={cellLookupType === 'HV' ? '1-160' : '1-32'}
              bind:value={cellLookupNum}
            />
          </div>
          {#if cellLookupInfo}
            <div class="p-2 rounded bg-surface-800 border border-primary-700 flex flex-col gap-1">
              <span><b>Type:</b> {cellLookupInfo.type}</span>
              <span><b>Module:</b> {cellLookupInfo.module}</span>
              <span><b>Cell:</b> {cellLookupInfo.cell}</span>
              <span><b>Voltage:</b>
                {#if cellLookupVoltNameTyped}
                  {#if !NamedDatatypeValues.includes(cellLookupVoltNameTyped)}
                    <span class="text-error-400">No data for this cell (not in NamedDatatypeValues)</span>
                  {:else}
                    <Store datatype={cellLookupVoltNameTyped} />
                  {/if}
                {/if}
              </span>
              <span><b>Temperature:</b>
                {#if cellLookupTempNameTyped}
                  {#if !NamedDatatypeValues.includes(cellLookupTempNameTyped)}
                    <span class="text-error-400">No data for this cell (not in NamedDatatypeValues)</span>
                  {:else}
                    <Store datatype={cellLookupTempNameTyped} />
                  {/if}
                {/if}
              </span>
              <span class="text-xs text-gray-400">Debug: {cellLookupVoltNameTyped}, {cellLookupTempNameTyped}</span>
            </div>
          {:else if cellLookupNum}
            <span class="text-error-400">Invalid cell number</span>
          {/if}
        </div>

        <!-- Cell tables with responsive layout -->
        <div class="col-span-full mt-8">
            <!-- View toggle controls for HV only -->
            <div class="mb-4 flex gap-4 items-center">
                <button 
                    class="px-4 py-2 rounded {showCompactView ? 'bg-surface-700' : 'bg-primary-600'} text-white"
                    on:click={() => { showCompactView = false; selectedModule = 1; }}>
                    Full View
                </button>
                <button 
                    class="px-4 py-2 rounded {showCompactView ? 'bg-primary-600' : 'bg-surface-700'} text-white"
                    on:click={() => showCompactView = true}>
                    Compact View
                </button>
                
                {#if showCompactView}
                    <select bind:value={selectedModule} class="px-3 py-1 rounded text-white" style="background-color: #1ec48e; color: #222; font-weight: bold;">
                        {#each Array.from({length: 10}, (_, i) => i + 1) as moduleNum}
                            <option value={moduleNum}>Module {moduleNum}</option>
                        {/each}
                    </select>
                {/if}
            </div>

            {#if showCompactView}
                <!-- Compact view - single module -->
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                    <Tile heading="HV Cell Voltages - Module {selectedModule}" containerClass="min-w-0">
                        <div class="overflow-x-auto">
                            <Table background="bg-surface-900 text-sm" tableArr={compactHvVoltArr} titles={hvCellTitles} />
                        </div>
                    </Tile>
                    <Tile heading="HV Cell Temperatures - Module {selectedModule}" containerClass="min-w-0">
                        <div class="overflow-x-auto">
                            <Table background="bg-surface-900 text-sm" tableArr={compactHvTempArr} titles={hvCellTitles} />
                        </div>
                    </Tile>
                </div>
            {:else}
                <!-- Full view with improved responsive layout -->
                <div class="flex flex-col gap-4">
                    <Tile heading="HV Cell Voltages" containerClass="min-w-0">
                        <div class="overflow-x-auto">
                            <div class="min-w-max">
                                <Table background="bg-surface-900 text-xs" tableArr={hvCellVoltArr} titles={hvCellTitles} />
                            </div>
                        </div>
                    </Tile>
                    <Tile heading="HV Cell Temperatures" containerClass="min-w-0">
                        <div class="overflow-x-auto">
                            <div class="min-w-max">
                                <Table background="bg-surface-900 text-xs" tableArr={hvCellTempArr} titles={hvCellTitles} />
                            </div>
                        </div>
                    </Tile>
                </div>
            {/if}
            <!-- Always show LV tables -->
            <div class="flex flex-col gap-4 mt-4">
                <Tile heading="LV Cell Voltages" containerClass="min-w-0">
                    <div class="overflow-x-auto">
                        <Table background="bg-surface-900 text-xs" tableArr={lvCellVoltArr} titles={lvCellTitles} />
                    </div>
                </Tile>
                <Tile heading="LV Cell Temperatures" containerClass="min-w-0">
                    <div class="overflow-x-auto">
                        <Table background="bg-surface-900 text-xs" tableArr={lvCellTempArr} titles={lvCellTitles} />
                    </div>
                </Tile>
            </div>
        </div>
    </TileGrid>
</div>
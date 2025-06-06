<script lang="ts">
    import { Chart, Command, serverStatus, TauriCommand, Tile, TileGrid } from '$lib';
    import {getToastStore} from "@skeletonlabs/skeleton";
    import { pinnedCharts, procedures } from '$lib/stores/data';
    import {parseProcedure} from "$lib/util/parsers";
    import { debugModeActive } from '$lib/stores/state';
    import Icon from '@iconify/svelte';
    import { ViewWindow } from '$lib/util/WindowControl';
    import { ChartLineSmooth, Flash, FlashOff, Wifi, WifiOff } from 'carbon-icons-svelte';

    const toastStore = getToastStore();
    const handleSuccess = () => {
        toastStore.trigger({
            message: "Server started successfully",
            background: "bg-primary-400",
            timeout: 1500
        });
        serverStatus.set(true);
    };

    const handleFailure = (error:string) => {
        toastStore.trigger({
            message: `Server did not start successfully: ${error}`,
            background: "bg-error-400"
        });
    };

    const parseProcedures = (rawProcedures: string[][]) => {
        procedures.set(rawProcedures.map(parseProcedure));
    };

    let graphVisualizerCount = 0;

</script>

<div class="h-full w-full p-5 flex flex-col gap-8">
    <div class="flex flex-row items-center gap-5">
        <img src="/images/logo-green-new.png" alt="Delft Hyperloop logo" class="w-40" />
        <h1 class="text-4xl text-primary-500">Delft Hyperloop Ground Station</h1>
    </div>
        <div class="flex gap-3 flex-wrap">
            {#if $debugModeActive}
                <TauriCommand cmd="connect_to_pod" successCallback={handleSuccess} errorCallback={handleFailure} icon={Wifi}/>
                <TauriCommand cmd="disconnect" successCallback={() => serverStatus.set(false)} icon={WifiOff}/>
                <Command cmd="StartHV" text="Start HV" icon={Flash}/>
                <Command cmd="StopHV" text="Stop HV" className="text-error-400 border-error-400 border-2 py-2" icon={FlashOff}/>
            {/if}

            <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
                   py-2 bg-primary-500 text-surface-900" on:click={() => new ViewWindow(`Charts_${graphVisualizerCount++}`, `/view/charts`)}>
                <ChartLineSmooth size={20} class="mr-1"/>
                Graph Visualizer
            </button>
            <TauriCommand cmd="save_logs"/>
            {#if $debugModeActive}
                <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
                   bg-primary-500 text-surface-900" on:click={() => {debugModeActive.set(false)}}>
                    <Icon icon="mdi:bug-outline" class="mr-1 w-6 h-6"/>
                    Disable Debug Mode
                </button>
            {:else}
                <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
                   bg-primary-500 text-surface-900" on:click={() => {debugModeActive.set(true)}}>
                    <Icon icon="mdi:bug-outline" class="mr-1 w-6 h-6"/>
                    Enable Debug Mode
                </button>
            {/if}
        </div>
    <p>
        Press <kbd class="kbd">Esc</kbd> or <kbd class="kbd">Space</kbd> to trigger Emergency Braking or
        <kbd class="kbd">D</kbd> to toggle Debug Mode. Use <kbd class="kbd">Shift+Number</kbd> to navigate through tabs.
    </p>
    {#if $pinnedCharts.length === 0}
        <p>Pinned graphs will appear here.</p>
    {/if}
    <TileGrid columns="" rows="">
        {#each $pinnedCharts as title}
            <Tile>
                <Chart title={title} background="bg-surface-900" height={250}/>
            </Tile>
        {/each}
    </TileGrid>
</div>

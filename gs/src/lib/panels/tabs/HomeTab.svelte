<script lang="ts">
    import { Chart, Command, serverStatus, TauriCommand, Tile, TileGrid } from '$lib';
    import {getToastStore} from "@skeletonlabs/skeleton";
    import { pinnedCharts, procedures } from '$lib/stores/data';
    import {parseProcedure} from "$lib/util/parsers";
    import { debugModeActive, inStateSystemCheck } from '$lib/stores/state';
    import Icon from '@iconify/svelte';
    import { ViewWindow } from '$lib/util/WindowControl';

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

</script>

<div class="h-full w-full p-5 flex flex-col gap-8">
    <div class="flex flex-row items-center gap-5">
        <img src="/images/logo-green-new.png" alt="Delft Hyperloop logo" class="w-40" />
        <h1 class="text-4xl text-primary-500">Delft Hyperloop Ground Station</h1>
    </div>
        <div class="flex gap-3 flex-wrap">
            {#if $debugModeActive}
                <TauriCommand cmd="connect_to_pod" successCallback={handleSuccess} errorCallback={handleFailure} />
                <TauriCommand cmd="disconnect" successCallback={() => serverStatus.set(false)} />
            {/if}
            <Command
                cmd="SystemCheck"
                dependency={inStateSystemCheck}
                dependencyTitle="Not in System Check"
                dependencyMessage="The pod must be in the System Check state to perform a system check!"
            />
            {#if $debugModeActive}
                <TauriCommand cmd="procedures" textOverride="Refresh Procedures" successCallback={parseProcedures} />
            {/if}
            <TauriCommand cmd="save_logs"/>

            <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
                   py-2 bg-primary-500 text-surface-900" on:click={() => new ViewWindow("Charts", `/view/charts`)}>
                <!--                <Icon icon="" class="mr-2 w-6 h-6"/>-->
                Graph list
            </button>
            {#if $debugModeActive}
                <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
                   py-2 bg-primary-500 text-surface-900" on:click={() => {debugModeActive.set(false)}}>
                    <Icon icon="mdi:bug-outline" class="mr-2 w-6 h-6"/>
                    Disable Debug Mode
                </button>
            {:else}
                <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
                   py-2 bg-primary-500 text-surface-900" on:click={() => {debugModeActive.set(true)}}>
                    <Icon icon="mdi:bug-outline" class="mr-2 w-6 h-6"/>
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

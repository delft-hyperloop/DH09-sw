<script lang="ts">
    import { Chart, serverStatus, TauriCommand, TileGrid } from '$lib';
    import {getToastStore} from "@skeletonlabs/skeleton";
    import { pinnedCharts, procedures } from '$lib/stores/data';
    import {parseProcedure} from "$lib/util/parsers";
    import { debugModeActive, GreenHVALTurnedOn, RedHVALTurnedOn } from '$lib/stores/state';
    import Icon from '@iconify/svelte';

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
        <TauriCommand cmd="connect_to_pod" successCallback={handleSuccess} errorCallback={handleFailure} />
        <TauriCommand cmd="disconnect" successCallback={() => serverStatus.set(false)} />
<!--        <TauriCommand cmd="start_levi" />-->
<!--        <TauriCommand cmd="quit_levi" />-->
        <TauriCommand cmd="procedures" textOverride="Refresh Procedures" successCallback={parseProcedures} />
        <TauriCommand cmd="save_logs"/>
        {#if $debugModeActive}
            <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
               py-2 bg-primary-500 text-surface-900" on:click={() => {debugModeActive.set(false)}}>
                <Icon icon="mdi:bug-outline" class="mr-2 w-6 h-6"/>
                Disable Debug Mode
            </button>
            <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
               py-2 bg-primary-500 text-surface-900" on:click={() => {GreenHVALTurnedOn.set(!$GreenHVALTurnedOn)}}>
                Toggle Green HVAL
            </button>
            <button class="btn [&>*]:pointer-events-none rounded-md font-number font-medium
               py-2 bg-primary-500 text-surface-900" on:click={() => {RedHVALTurnedOn.set(!$RedHVALTurnedOn)}}>
                Toggle Red HVAL
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
        <kbd class="kbd">D</kbd> to toggle Debug Mode. Use numbers to navigate through tabs.
    </p>
    {#if $pinnedCharts.length === 0}
        <p>Pinned tabs will appear here.</p>
    {/if}
    <TileGrid columns="" rows="">
        {#each $pinnedCharts as title}
            <Chart title={title} background="bg-surface-900" height={250}/>
        {/each}
    </TileGrid>
</div>

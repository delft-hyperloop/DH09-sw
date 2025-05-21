<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import VitalsPanel from "$lib/panels/VitalsPanel.svelte";
    import DetailsPanel from "$lib/panels/DetailsPanel.svelte";
    import LogsPanel from "$lib/panels/LogsPanel.svelte";
    import { logsPanelSize, logsScrollAreaSize, logsVisible, rendering, threeDModeActive } from '$lib/stores/state';
    import { VIEWPORT_HEIGHT_NORMALIZING_VALUE } from '$lib';
    import LogsPanel3D from '$lib/panels3d/LogsPanel3D.svelte';
    import MainPanel3D from '$lib/panels3d/MainPanel3D.svelte';
    import StartScreen3D from '$lib/panels3d/StartScreen3D.svelte';
</script>

<main class="w-full grow border-t border-black overflow-hidden max-h-screen">
    {#if $threeDModeActive}
        {#if !$rendering}
            <StartScreen3D/>
        {:else}
            <Splitpanes theme="modern-theme" dblClickSplitter={false} horizontal={false} class="bg-black">
                <Pane>
                    <MainPanel3D/>
                </Pane>
                <Pane maxSize={30} minSize={2.5} size={2.5} class="bg-surface-900">
                    <LogsPanel3D/>
                </Pane>
            </Splitpanes>
        {/if}
    {:else}
        <Splitpanes theme="modern-theme" dblClickSplitter={false}>
            <Pane maxSize={50} snapSize={10} minSize={2} size={40} class="bg-surface-700" >
                <VitalsPanel />
            </Pane>
            <Pane minSize={50} class="bg-surface-900">
                <Splitpanes horizontal={true} theme="modern-theme-logs" dblClickSplitter={false} on:resize={(event) => {
                    logsPanelSize.set(event.detail[1].size);
                    logsScrollAreaSize.set($logsPanelSize - ($logsPanelSize * 0.05 + 4.5) + window.innerHeight / VIEWPORT_HEIGHT_NORMALIZING_VALUE * 10 - 10);
                    if (event.detail[1].size === 5) {
                        logsVisible.set(false);
                    } else {
                        logsVisible.set(true);
                    }
                }}>
                    <Pane minSize={50} class="bg-surface-800">
                        <DetailsPanel />
                    </Pane>
                    <Pane size={($logsVisible) ? $logsPanelSize : 5} minSize={5} class="bg-surface-800">
                        <LogsPanel />
                    </Pane>
                </Splitpanes>
            </Pane>
        </Splitpanes>
    {/if}
</main>
<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import VitalsPanel from "$lib/panels/VitalsPanel.svelte";
    import DetailsPanel from "$lib/panels/DetailsPanel.svelte";
    import LogsPanel from "$lib/panels/LogsPanel.svelte";
    import { logsPanelSize, logsScrollAreaSize, logsVisible } from '$lib/stores/state';
    import { VIEWPORT_HEIGHT_NORMALIZING_VALUE } from '$lib';
</script>

<main class="w-full flex-grow border-t border-black overflow-auto">
    <Splitpanes theme="modern-theme" dblClickSplitter={false}>
        <Pane maxSize={50} snapSize={10} minSize={2} size={43} class="bg-surface-700" >
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
</main>
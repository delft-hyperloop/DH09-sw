<script lang="ts">
    import { Pane, Splitpanes } from "svelte-splitpanes";
    import VitalsPanel from "$lib/panels/VitalsPanel.svelte";
    import DetailsPanel from "$lib/panels/DetailsPanel.svelte";
    import LogsPanel from "$lib/panels/LogsPanel.svelte";

    import {details_pane, vitals_pane} from "$lib";
    import { logsVisible } from '$lib/stores/state';
</script>

<main class="w-full flex-grow border-t border-black overflow-auto">
    <Splitpanes theme="modern-theme" dblClickSplitter={false} on:resize={(event) => vitals_pane.set(event.detail[0].size)}>
        <Pane maxSize={50} snapSize={10} minSize={2} size={40} class="bg-surface-700" >
            <VitalsPanel />
        </Pane>
        <Pane minSize={50} class="bg-surface-900">
            <Splitpanes horizontal={true} theme="modern-theme-logs" dblClickSplitter={false} on:resize={(event) => {
                details_pane.set(event.detail[0].size);
                if (event.detail[0].size === 95) {
                    logsVisible.set(false);
                } else {
                    logsVisible.set(true);
                }
            }}>
                <Pane minSize={50} class="bg-surface-800">
                    <DetailsPanel />
                </Pane>
                <Pane size={($logsVisible) ? 30 : 5} minSize={5} class="bg-surface-800 ">
                    <LogsPanel />
                </Pane>
            </Splitpanes>
        </Pane>
    </Splitpanes>
</main>
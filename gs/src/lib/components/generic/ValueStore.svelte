<script lang="ts">
    import {latestTimestamp} from "$lib/stores/state";

    const STALE_DATA_TICKS = 500;
    export let value: number | string;
    export let timestamp: number;
</script>

<span class="{$latestTimestamp - timestamp > STALE_DATA_TICKS ? 'text-surface-400' : ''}">
    {value}
</span>



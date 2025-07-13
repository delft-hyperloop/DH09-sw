<script lang="ts">
    import { latestTimestamp } from '$lib/stores/state';

    const STALE_DATA_TICKS = 500;

    export let displayName: boolean = true;
    export let value: number | string;
    export let timestamp: number;
    export let name: string;
    export let upper: string | undefined = undefined;
    export let lower: string | undefined = undefined;
    export let className: string = "";
</script>

<div class="flex flex-col">
    <div class="flex flex-row gap-1 {className}">
        {#if displayName}
            <span>{name}: </span>
        {/if}
        <span class="{$latestTimestamp - timestamp > STALE_DATA_TICKS ? 'text-surface-400' : ''}">
            {value}
        </span>
    </div>
    {#if lower !== undefined && upper !== undefined}
        <span class="text-surface-400">
            Safety Range: [{lower}, {upper}]
        </span>
    {/if}
</div>

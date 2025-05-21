<script lang="ts">
    interface Props {
        status: boolean;
        on?: string;
        off?: string;
        label?: string;
        onColor?: string;
        offColor?: string;
    }

    let {
        status,
        on = "Connected",
        off = "Disconnected",
        label = '',
        onColor = 'text-primary-400',
        offColor = 'text-error-400'
    }: Props = $props();
</script>

<div class="flex gap-2 text-sm">
    {#if label !== ''}
        <p>{label}:</p>
    {/if}
    {#if status}
        <p class={onColor}>{on}</p>
    {:else}
        <p class={offColor}>{off}</p>
    {/if}
</div>

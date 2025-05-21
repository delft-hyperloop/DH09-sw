<script lang="ts">
    interface Props {
        bgToken?: number;
        insideClass?: string;
        heading?: string;
        containerClass?: string;
        children?: import('svelte').Snippet;
    }

    let {
        bgToken = 900,
        insideClass = '',
        heading = '',
        containerClass = '',
        children
    }: Props = $props();
</script>
<div class="bg-surface-{bgToken} p-4 {containerClass} rounded-lg">
    {#if heading !== ''}
        <h3 class="text-xl mb-2 font-normal col-span-full text-center">{heading}</h3>
        <hr class="col-span-full mb-2">
    {/if}
    <div class="{insideClass}">
        {@render children?.()}
    </div>
</div>
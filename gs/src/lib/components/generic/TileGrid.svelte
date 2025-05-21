<script lang="ts">
    interface Props {
        columns: string;
        rows: string;
        className?: string;
        children?: import('svelte').Snippet;
    }

    let {
        columns,
        rows,
        className = '',
        children
    }: Props = $props();
</script>

<div class="grid gap-2 {className}" style="grid-template-columns: {columns}; grid-template-rows: {rows};">
    {@render children?.()}
</div>
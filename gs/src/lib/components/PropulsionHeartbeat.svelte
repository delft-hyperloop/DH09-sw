<script lang="ts">
    import { GrandDataDistributor } from '$lib';
    import type { NamedDatatype } from '$lib/types';

    interface Props {
        storeName: NamedDatatype;
        labels?: string[];
    }

    let { storeName, labels = [""] }: Props = $props();

    const storeManager = GrandDataDistributor.getInstance().stores;
    const ppControlWordStore = storeManager.getWritable(storeName);
</script>

<div class="grid grid-cols-11 gap-2 items-center">
    <span class="text-center">Heartbeat:</span>
    <span class="text-center">{($ppControlWordStore.value >> 2 & 1) + ($ppControlWordStore.value >> 3 & 1) * 2}</span>
    <span class="text-center">{$ppControlWordStore.value >> 1 & 1}</span>
    <span class="text-center">{$ppControlWordStore.value & 1}</span>

    {#each Array.from({ length: 7 }, (_, i) => labels.length - i) as i}
        <span class="text-center">{$ppControlWordStore.value >> (i + 4) & 1}</span>
    {/each}

    <span></span>
    {#each labels as l}
        <span class="text-center">{l}</span>
    {/each}
    <span></span>
</div>
<script lang="ts">
    import { GrandDataDistributor } from '$lib';
    import type { NamedDatatype } from '$lib/types';
    import ValueStore from '$lib/components/generic/ValueStore.svelte';

    export let storeName: NamedDatatype;
    export let labels: string[] = [""];

    const storeManager = GrandDataDistributor.getInstance().stores;
    const ppControlWordStore = storeManager.getWritable(storeName);
</script>

<div class="grid grid-cols-11 gap-2 items-center">
    <span class="text-center">Heartbeat:</span>
    <ValueStore className="justify-center" value={($ppControlWordStore.value >> 2 & 1) + ($ppControlWordStore.value >> 3 & 1) * 2} timestamp={$ppControlWordStore.timestamp} name="" displayName={false}/>
    <ValueStore className="justify-center" value={$ppControlWordStore.value >> 1 & 1} timestamp={$ppControlWordStore.timestamp} name="" displayName={false}/>
    <ValueStore className="justify-center" value={$ppControlWordStore.value & 1} timestamp={$ppControlWordStore.timestamp} name="" displayName={false}/>

    {#each Array.from({ length: 7 }, (_, i) => labels.length - i) as i}
        <ValueStore className="justify-center" value={$ppControlWordStore.value >> (i + 4) & 1} timestamp={$ppControlWordStore.timestamp} displayName={false} name=""/>
    {/each}

    <span/>
    {#each labels as l}
        <span class="text-center">{l}</span>
    {/each}
    <span/>
</div>
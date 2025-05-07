<script lang="ts">
    import type { NamedDatatype } from '$lib/types';
    import { GrandDataDistributor } from '$lib';

    export let storeName: NamedDatatype;

    const storeManager = GrandDataDistributor.getInstance().stores;
    const faultStore = storeManager.getWritable(storeName);
    let labels: string[] = [
        "Initialized",
        "Bus Voltage",
        "Temperature",
        "Localization",
        "Current Angle",
        "Control Data",
        "Gate Drivers",
        "Fault Cleared",
    ];

</script>

<div class="grid grid-cols-9 gap-2 items-center">
    <span class="text-center">Init Fault:</span>

    {#each Array.from({ length: labels.length }, (_, i) => i) as i}
        <span class="text-center">{($faultStore.value >> i) & 1}</span>
    {/each}

    <span/>
    {#each labels as l}
        <span class="text-center">{l}</span>
    {/each}
    <span/>
</div>
<script lang="ts">
    import { propulsionConfigSent } from '$lib/stores/state';
    import { setPropulsionPoints } from '$lib/stores/data';

    export let index: number;

    let spanClass: string = "text-lg flex flex-col gap-4";
</script>

<div class={spanClass}>
    {#if $propulsionConfigSent}
        {#if index === 0}
            <span class="flex flex-row gap-4">
                X<sub>b</sub>: {$setPropulsionPoints[0].location}
            </span>
            <span>
                I<sub>max_b</sub>: {$setPropulsionPoints[0].imax}
            </span>
        {:else}
            <span>
                X<sub>{index}</sub>: {$setPropulsionPoints[index].location}
            </span>
            <span>
                I<sub>max_{index}</sub>: {$setPropulsionPoints[index].imax}
            </span>
        {/if}
    {:else}
        {#if index === 0}
            <span>
                X<sub>b</sub>: Not Set
            </span>
            <span>
                I<sub>max_b</sub>: Not Set
            </span>
        {:else}
            <span>
                X<sub>{index}</sub>: Not Set
            </span>
            <span>
                I<sub>max_{index}</sub>: Not Set
            </span>
        {/if}
    {/if}
</div>
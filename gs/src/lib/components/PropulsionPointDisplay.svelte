<script lang="ts">
    import { propulsionConfigSent } from '$lib/stores/state';
    import { propulsionPoints } from '$lib/stores/data';

    export let index: number;

    let spanClass: string = "text-lg flex flex-col gap-4";
</script>

<div class={spanClass}>
    {#if $propulsionConfigSent}
        {#if index === 0}
            <span class="flex flex-row gap-4">
                X<sub>b</sub>: {$propulsionPoints[0].location}
            </span>
            <span>
                I<sub>max_b</sub>: {$propulsionPoints[0].imax}
            </span>
        {:else}
            <span>
                X<sub>{index}</sub>: {$propulsionPoints[index].location}
            </span>
            <span>
                I<sub>max_{index}</sub>: {$propulsionPoints[index].imax}
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
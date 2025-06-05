<script lang="ts">
    import { propulsionPoints } from '$lib/stores/data';

    export let index: number = 0;

    let inputClass: string = "input rounded-lg px-1 min-h-10 mb-2";
    let spanClass: string = "text-lg";

    let location: number = 0;
    let iq: number = 0;
    let id: number = 0;

    let remakePropPoints = () => {
        let current = $propulsionPoints;

        current[index].location = location;
        current[index].iq = iq;
        current[index].id = id;

        propulsionPoints.set(current);

        console.log(`location: ${$propulsionPoints[index].location}, iq: ${$propulsionPoints[index].iq}, id: ${$propulsionPoints[index].id}`);
    }
</script>

<div class="flex flex-col gap-2 mb-2 ">
    <div class="flex flex-row gap-1">
        <span class={spanClass}>
            {#if index === 0}
                X<sub>b</sub>
            {:else}
                X<sub>{index}</sub>
            {/if}
        </span>
        <input class={inputClass} type="number" bind:value={location} on:change={remakePropPoints}/>
    </div>
    <div class="flex flex-row gap-1">
        <span class={spanClass}>
            {#if index === 0}
                Iq<sub>b</sub>
            {:else}
                Iq<sub>{index}</sub>
            {/if}
        </span>
        <input class={inputClass} type="number" bind:value={iq} on:change={remakePropPoints}/>
    </div>
    <div class="flex flex-row gap-1">
        <span class={spanClass}>
            {#if index === 0}
                Id<sub>b</sub>
            {:else}
                Id<sub>{index}</sub>
            {/if}
        </span>
        <input class={inputClass} type="number" bind:value={id} on:change={remakePropPoints}/>
    </div>
</div>
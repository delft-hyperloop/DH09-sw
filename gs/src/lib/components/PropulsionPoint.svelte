<script lang="ts">
    import { propulsionPoints } from '$lib/stores/data';

    export let index: number = 0;

    let inputClass: string = "input rounded-lg px-1 min-h-10";
    let spanClass: string = "text-lg w-20 text-center justify-center";

    let location: number = 0;
    // let iq: number = 0;
    // let id: number = 0;
    let imax: number = 0;

    let remakePropPoints = () => {
        let current = $propulsionPoints;

        current[index].location = location;
        current[index].imax = imax;
        // current[index].iq = iq;
        // current[index].id = id;

        propulsionPoints.set(current);

        console.log(`prop point: ${$propulsionPoints[index].location}, imax: ${$propulsionPoints[index].imax}`);
    }
</script>

<div class="flex flex-col gap-2 justify-center items-center h-full w-full">
    <div class="flex flex-row gap-1 justify-center items-center">
        <span class={spanClass}>
            {#if index === 0}
                X<sub>b</sub>
            {:else}
                X<sub>{index}</sub>
            {/if}
        </span>
        <input class={inputClass} type="number" bind:value={location} on:change={remakePropPoints}/>
    </div>
    <div class="flex flex-row gap-1  justify-center items-center">
        <span class={spanClass}>
            {#if index === 0}
                I<sub>max_b</sub>
            {:else}
                I<sub>max_{index}</sub>
            {/if}
        </span>
        <input class={inputClass} type="number" bind:value={imax} on:change={remakePropPoints}/>
    </div>
<!--    <div class="flex flex-row gap-1">-->
<!--        <span class={spanClass}>-->
<!--            {#if index === 0}-->
<!--                Id<sub>b</sub>-->
<!--            {:else}-->
<!--                Id<sub>{index}</sub>-->
<!--            {/if}-->
<!--        </span>-->
<!--        <input class={inputClass} type="number" bind:value={id} on:change={remakePropPoints}/>-->
<!--    </div>-->
</div>
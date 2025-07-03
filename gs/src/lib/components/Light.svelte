<script lang="ts">
    import { onDestroy } from 'svelte';
    import { GreenHVALTurnedOn, RedHVALTurnedOn } from '$lib/stores/state';
    import { GrandDataDistributor } from '$lib';

    // True means the light will be green while false is red.
    export let innerClass: string = "";
    export let isGreen: boolean = true;

    const greenShadow = 'shadow-[0_0_10px_rgba(0,255,0,0.8),0_0_20px_rgba(0,255,0,0.6),0_0_30px_rgba(0,255,0,0.4),0_0_40px_rgba(0,255,0,0.2)]';
    const redShadow = 'shadow-[0_0_10px_rgba(255,0,0,0.8),0_0_20px_rgba(255,0,0,0.6),0_0_30px_rgba(255,0,0,0.4),0_0_40px_rgba(255,0,0,0.2)]';
    let shadow = "";

    // red is off or flashing
    // green is on or off

    const green: string = "bg-[#0f0]";
    const red: string = "bg-[#f00]";
    const colorOff = "bg-surface-800";
    let color: string = "bg-[#000]";

    let blinkOn: boolean = false;
    let timer: number = 200;
    let blinkInterval: NodeJS.Timeout | null = null;

    let stores = GrandDataDistributor.getInstance().stores;
    let hvalSTate = stores.getWritable("HVALState");

    function startBlinking() {
        if (blinkInterval) {
            clearTimeout(blinkInterval);
        }
        blinkInterval = setInterval(() => {
            blinkOn = !blinkOn;
            shadow = blinkOn ? redShadow : "";
            color = blinkOn ? red : "bg-[#000]";
        }, timer);
    }

    $: {
        if ($hvalSTate.value == 2 && !isGreen) { // red turned on
            startBlinking();
        } else if (!isGreen) {
            color = colorOff;
            shadow = colorOff;
            if (blinkInterval) {
                clearTimeout(blinkInterval);
            }
            blinkOn = true;
        } else if ($hvalSTate.value == 1) { // green turned
            color = green;
            shadow = greenShadow;
            blinkOn = true;
        } else {
            color = colorOff;
            shadow = colorOff;
        }
    }

    onDestroy(() => {
        if (blinkInterval) {
            clearTimeout(blinkInterval);
        }
    });

</script>

<div class="
    {innerClass}
    rounded-full {blinkOn ? color : 'bg-surface-800'} size-7
    {shadow}
    "
/>

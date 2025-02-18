<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { HVALTurnedOn, blinking } from '$lib/stores/state';

    // True means the light will be green while false is red.
    export let innerClass: string = "";

    const greenShadow = 'shadow-[0_0_10px_rgba(0,255,0,0.8),0_0_20px_rgba(0,255,0,0.6),0_0_30px_rgba(0,255,0,0.4),0_0_40px_rgba(0,255,0,0.2)]';
    const redShadow = 'shadow-[0_0_10px_rgba(255,0,0,0.8),0_0_20px_rgba(255,0,0,0.6),0_0_30px_rgba(255,0,0,0.4),0_0_40px_rgba(255,0,0,0.2)]';
    let shadow = $HVALTurnedOn ? greenShadow : redShadow;

    const green: string = "bg-[#0f0]";
    const red: string = "bg-[#f00]";
    let color: string = $HVALTurnedOn ? green : red;

    let blinkOn: boolean = true;
    let timer: number = 500;
    let blinkInterval: NodeJS.Timeout | null = null;

    function startBlinking() {
        if (blinkInterval) {
            clearTimeout(blinkInterval);
        }
        blinkInterval = setInterval(() => {
            blinkOn = !blinkOn;
            shadow = blinkOn ? ($HVALTurnedOn ? greenShadow : redShadow) : "";
            color = blinkOn ? ($HVALTurnedOn ? green : red) : "#000";
        }, timer);
    }

    $: {
        if ($blinking) {
            startBlinking();
        } else {
            if (blinkInterval) {
                clearTimeout(blinkInterval);
            }
            blinkOn = true;
            shadow = $HVALTurnedOn ? greenShadow : redShadow;
            color = $HVALTurnedOn ? green : red;
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
    rounded-full {blinkOn ? color : 'bg-surface-800'} w-10 h-10
    {shadow}
    "
/>

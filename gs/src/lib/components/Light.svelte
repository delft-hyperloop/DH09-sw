<script lang="ts">
    import { run } from 'svelte/legacy';

    import { onDestroy } from 'svelte';
    import { GreenHVALTurnedOn, RedHVALTurnedOn } from '$lib/stores/state';

    
    interface Props {
        // True means the light will be green while false is red.
        innerClass?: string;
        isGreen?: boolean;
    }

    let { innerClass = "", isGreen = true }: Props = $props();

    const greenShadow = 'shadow-[0_0_10px_rgba(0,255,0,0.8),0_0_20px_rgba(0,255,0,0.6),0_0_30px_rgba(0,255,0,0.4),0_0_40px_rgba(0,255,0,0.2)]';
    const redShadow = 'shadow-[0_0_10px_rgba(255,0,0,0.8),0_0_20px_rgba(255,0,0,0.6),0_0_30px_rgba(255,0,0,0.4),0_0_40px_rgba(255,0,0,0.2)]';
    let shadow = $state("");

    // red is off or flashing
    // green is on or off

    const green: string = "bg-[#0f0]";
    const red: string = "bg-[#f00]";
    const colorOff = "bg-surface-800";
    let color: string = $state("bg-[#000]");

    let blinkOn: boolean = $state(false);
    let timer: number = 200;
    let blinkInterval: NodeJS.Timeout | null = $state(null);

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

    run(() => {
        if ($RedHVALTurnedOn && !isGreen) {
            startBlinking();
        } else if (!isGreen) {
            color = colorOff;
            shadow = colorOff;
            if (blinkInterval) {
                clearTimeout(blinkInterval);
            }
            blinkOn = true;
        } else if ($GreenHVALTurnedOn) {
            color = green;
            shadow = greenShadow;
            blinkOn = true;
        } else {
            color = colorOff;
            shadow = colorOff;
        }
    });

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
></div>

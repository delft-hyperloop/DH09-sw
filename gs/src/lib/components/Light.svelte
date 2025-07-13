<script lang="ts">
    import { GrandDataDistributor } from '$lib';
    import { RedHVALTurnedOn } from '$lib/stores/state';

    export let innerClass: string = "";
    // True means the light will be green while false is red.
    export let isGreen: boolean = true;

    const greenShadow = 'shadow-[0_0_10px_rgba(0,255,0,0.8),0_0_20px_rgba(0,255,0,0.6),0_0_30px_rgba(0,255,0,0.4),0_0_40px_rgba(0,255,0,0.2)]';
    const redShadow = 'shadow-[0_0_10px_rgba(255,0,0,0.8),0_0_20px_rgba(255,0,0,0.6),0_0_30px_rgba(255,0,0,0.4),0_0_40px_rgba(255,0,0,0.2)]';
    $: shadow = "";

    // red is off or flashing
    // green is on or off

    const green: string = "bg-[#0f0]";
    const red: string = "bg-[#f00]";
    const colorOff = "bg-surface-800";
    $: color = "bg-[#000]";

    let blinkOn: boolean = false;

    let stores = GrandDataDistributor.getInstance().stores;
    let hvalSTate = stores.getWritable("HVALState");

    $: {
        console.log("Changed hval value to " + $hvalSTate.value);
        switch($hvalSTate.value) {
            case 1: {
                if (isGreen) {
                    color = green;
                    shadow = greenShadow;

                    console.log("Color should be green");
                } else {
                    color = colorOff;
                    shadow = colorOff;
                }
                break;
            }
            case 2: {
                if (isGreen) {
                    color = colorOff;
                    shadow = colorOff;
                } else {
                    if ($RedHVALTurnedOn) {
                        color = red;
                        shadow = redShadow;
                    } else {
                        color = colorOff;
                        shadow = colorOff;
                    }
                }
                break;
            }
            case 3: {
                if (isGreen) {
                    color = green;
                    shadow = greenShadow;
                } else {
                    if ($RedHVALTurnedOn) {
                        color = red;
                        shadow = redShadow;
                    } else {
                        color = colorOff;
                        shadow = colorOff;
                    }
                }
                break;
            }
            default: {
                color = colorOff;
                shadow = colorOff;
            }
        }
    }

</script>

<div class="
    {innerClass}
    rounded-full {color} size-7
    {shadow}
    "
/>

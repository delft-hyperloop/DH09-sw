<script lang="ts">
    import { T } from '@threlte/core'
    import { onMount } from 'svelte';
    import { cubicInOut } from 'svelte/easing';
    import { tweened } from 'svelte/motion';
    import {
        enteringScene,
        GROUND_Y_COORD,
        START_Y_COORD,
    } from '../../routes/view/pod/PodState';
    import { inScene } from '../../routes/view/pod/PodState.js';

    let levitating: boolean = $state(false);
    let levitatingPositionUp: boolean = true;
    let levitationLimit: number = 0.025;
    const y = tweened(START_Y_COORD, {
        duration: 3000,
        easing: cubicInOut
    });

    const yLevitating = tweened(GROUND_Y_COORD, {
        duration: 1100,
        easing: cubicInOut
    });

    onMount(() => {
        setInterval(() => {
            if (levitating) {
                if (levitatingPositionUp) {
                    yLevitating.set(levitationLimit);
                } else {
                    yLevitating.set(-levitationLimit);
                }
                levitatingPositionUp = !levitatingPositionUp;
            }
        }, 1000);
    })

    onMount(() => {
        enteringScene.subscribe(() => {
            if ($enteringScene) {
                y.set(GROUND_Y_COORD);
            }
        });

        y.subscribe(() => {
            if ($enteringScene && $y === GROUND_Y_COORD) {
                enteringScene.set(false);
                inScene.set(true);
            }
        })

        inScene.subscribe(() => {
            if ($inScene) {
                levitating = true;
            }
        });

        // FSM_BROADCAST_CHANNEL.onmessage = (event) => {
        //     let state = event.data;
        //     console.log(`FSM state: ${state}`);
        //     if (state === 7) {
        //         start();
        //     } else {
        //         stop();
        //         if ($y !== START_Y_COORD) {
        //             y.set(0);
        //         }
        //     }
        // }

        // setInterval(() => {
        //     if ($inScene) {
        //         let stored = localStorage.getItem('FSMState');
        //         let fsmState = stored ? JSON.parse(stored) : 0;
        //
        //         console.log(`FSM state: ${fsmState}`);
        //         if (fsmState === 7) {
        //             start();
        //         } else {
        //             stop();
        //             if ($y !== START_Y_COORD) {
        //                 y.set(0);
        //             }
        //         }
        //     }
        // }, 500);
    })
</script>

<T.Mesh
    position.y={levitating ? $yLevitating : $y}
>
    <T.BoxGeometry />
    <T.MeshBasicMaterial color="gray" />
</T.Mesh>
<T.PerspectiveCamera
    makeDefault
    position={[5, 2, 5]}
    on:create={({ ref }) => {
        ref.lookAt(0, 0, 0)
    }}
/>

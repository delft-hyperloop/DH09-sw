<script lang="ts">
    import { T } from '@threlte/core'
    import { onMount } from 'svelte';
    import { cubicInOut, sineInOut } from 'svelte/easing';
    import { tweened } from 'svelte/motion';
    import { GROUND_Y_COORD, START_Y_COORD } from '$lib';
    import { enteringScene, inScene } from '$lib/stores/state';

    let levitating: boolean = false;
    let levitatingPositionUp: boolean = true;
    let levitationLimit: number = 0.025;

    const y = tweened(START_Y_COORD, {
        duration: 3000,
        easing: cubicInOut
    });
    const yLevitating = tweened(GROUND_Y_COORD, {
        duration: 1000,
        easing: sineInOut
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
    })
</script>

<T.PerspectiveCamera
    makeDefault
    position={[5, 2, 5]}
    on:create={({ ref }) => {
        ref.lookAt(0, 0, 0)
    }}
/>
<T.DirectionalLight
    intensity={0.5}
    position={[5, 5, -5]}
    castShadow
/>
<T.DirectionalLight
    intensity={0.8}
    position={[-5, 5, 5]}
    castShadow
/>


<T.Mesh
    position.y={levitating ? $yLevitating : $y}
    receiveShadow
>
    <T.BoxGeometry />
    <T.MeshStandardMaterial color="gray" />
</T.Mesh>
<T.Mesh
    position.y={$y - 1}
    receiveShadow
    rotation={[-Math.PI/2, 0, 0]}
>
    <T.PlaneGeometry
        args={[2, 2]}
    />
    <T.MeshStandardMaterial color="gray"/>
</T.Mesh>

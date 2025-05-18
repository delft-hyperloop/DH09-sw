<script lang="ts">
    import { Canvas } from '@threlte/core';
    import Scene from '$lib/panels3d/Scene.svelte';
    import { rendering } from '$lib/stores/state';
</script>

<div class="justify-center h-full w-full {$rendering ? 'visible' : 'invisible'}">
    <Canvas>
        <Scene/>
    </Canvas>
</div>

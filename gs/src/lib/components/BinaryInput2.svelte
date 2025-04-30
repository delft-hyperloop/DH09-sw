<script lang="ts">
    import { propControlWord2 } from '$lib/stores/state';

    export let index: number = 0;
    let value = $propControlWord2 >> index & 1;

    let changeValue = () => {
        let old = $propControlWord2;
        $propControlWord2 = $propControlWord2 ^ (1 << index);
        value = $propControlWord2 >> index & 1;
        console.log(`Control Word: ${$propControlWord2}`);
    }

</script>

<div class="flex flex-col items-center">
    <button on:click={changeValue}>▲</button>
    <input type="text" maxlength="1" class="text-center input rounded-md " bind:value={value}>
    <button on:click={changeValue}>▼</button>
</div>
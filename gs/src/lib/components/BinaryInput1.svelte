<script lang="ts">
    import { propControlWord1 } from '$lib/stores/state';

    export let index: number = 0;
    let value = $propControlWord1 >> index & 1;

    let changeValue = () => {
        $propControlWord1 = $propControlWord1 ^ (1 << index);
        value = $propControlWord1 >> index & 1;
    }

</script>

<div class="flex flex-col items-center">
    <button on:click={changeValue}>▲</button>
    <input type="text" maxlength="1" class="text-center input rounded-md " bind:value={value}>
    <button on:click={changeValue}>▼</button>
</div>
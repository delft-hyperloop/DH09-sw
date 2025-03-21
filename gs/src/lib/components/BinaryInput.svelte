<script lang="ts">

    import { propControlWord } from '$lib/stores/state';

    export let index: number = 0;
    let value = $propControlWord >> index & 1;

    let changeValue = () => {
        let old = $propControlWord;
        $propControlWord = $propControlWord ^ (1 << index);
        value = $propControlWord >> index & 1;
        console.log($propControlWord);
    }

</script>

<div class="flex flex-col items-center">
    <button on:click={changeValue}>▲</button>
    <input type="text" maxlength="1" class="text-center input rounded-md " bind:value={value}>
    <button on:click={changeValue}>▼</button>
</div>
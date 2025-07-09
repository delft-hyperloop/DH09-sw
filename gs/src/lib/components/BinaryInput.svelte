<script lang="ts">
    import { ptResetMessage } from '$lib/stores/state';
    import type { Writable } from 'svelte/store';

    export let store: Writable<number>;
    export let index: number = 0;

    // let value = $ptResetMessage >> index & 1;
    let value = $store >> index & 1;

    let changeValue = () => {
        // $ptResetMessage = $ptResetMessage ^ (1 << index);
        // value = $ptResetMessage >> index & 1;

        $store = $store ^ (1 << index);
        value = $store >> index & 1;

        console.log(`Store value: ${$store}`);
        console.log(`PtReset value: ${$ptResetMessage}`);
    }

</script>

<div class="flex flex-col items-center">
    <button on:click={changeValue}>▲</button>
    <input type="text" maxlength="1" class="text-center input rounded-md " bind:value={value}>
    <button on:click={changeValue}>▼</button>
</div>
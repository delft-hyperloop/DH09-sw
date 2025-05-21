<script lang="ts">
    import { propControlWord1 } from '$lib/stores/state';

    interface Props {
        index?: number;
    }

    let { index = 0 }: Props = $props();
    let value = $state($propControlWord1 >> index & 1);

    let changeValue = () => {
        $propControlWord1 = $propControlWord1 ^ (1 << index);
        value = $propControlWord1 >> index & 1;
    }

</script>

<div class="flex flex-col items-center">
    <button onclick={changeValue}>▲</button>
    <input type="text" maxlength="1" class="text-center input rounded-md " bind:value={value}>
    <button onclick={changeValue}>▼</button>
</div>
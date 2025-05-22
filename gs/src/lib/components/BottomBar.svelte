<script lang="ts">
    import { onMount } from 'svelte';
    import { bigErrorStatus, threeDModeActive } from '$lib/stores/state';
    import { GrandDataDistributor } from '$lib';

    let time = $state(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: true }));

    let stores = GrandDataDistributor.getInstance().stores;
    let fsmState = stores.getWritable("FSMState");

    onMount(() => {
        const interval = setInterval(() => {
            time = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: true });
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    });
</script>

<!---->
<!-- <button class="btn" on:click={() => bigErrorStatus.set(0)}> -->
<!--     change to 0 -->
<!-- </button> -->
<!-- <button class="btn" on:click={() => bigErrorStatus.set(1)}> -->
<!--     change to 1 -->
<!-- </button> -->
<!-- <button class="btn" on:click={() => bigErrorStatus.set(2)}> -->
<!--     change to 2 -->
<!-- </button> -->

<footer class="text-sm {$threeDModeActive ? 'bg-surface-900 text-primary-500' : $bigErrorStatus === 0 ? 'bg-primary-600 text-zinc-100' :
               $bigErrorStatus === 1 ? 'bg-warning-500 text-zinc-100' : 'bg-error-500 text-zinc-100'} w-full flex
               flex-row flex-nowrap justify-between px-4 gap-4 border-t border-black">
    <p>Delft Hyperloop: Theia</p>
    <div class="flex gap-4">
        <p>Current state: {$fsmState.value}</p>
        <p class="">{time}</p>
    </div>
</footer>

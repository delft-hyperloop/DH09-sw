<script lang="ts">
    import { Tile } from '$lib';
    import { slide } from 'svelte/transition';
    import { ArrowUp } from 'carbon-icons-svelte';

    export let title: string;

    let visible: boolean = true;

    let toggleVisibility = () => {
        visible = !visible;
    }
</script>

<Tile containerClass="col-span-full">
    <div class="flex justify-between">
        <h3 class="text-xl font-normal">
            <button on:click={toggleVisibility}>
                {title}
            </button>
        </h3>
        <div class="transition-transform duration-300 flex items-center text-center " class:rotate-180={!visible}>
            <button on:click={toggleVisibility}>
                <ArrowUp size={16}/>
            </button>
        </div>
    </div>
    {#if visible}
        <div transition:slide={{ duration: 300 }}>
            <hr class="col-span-full my-4">
            <slot name="content"/>
        </div>
    {/if}
</Tile>
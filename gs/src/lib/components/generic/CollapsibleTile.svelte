<script lang="ts">
    import { Tile } from '$lib';
    import Icon from '@iconify/svelte';
    import { slide } from 'svelte/transition';

    export let title: string;

    let visible: boolean = false;

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
                <Icon icon="carbon:arrow-up"/>
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
<script lang="ts">
    import { Tile } from '$lib';
    import { slide } from 'svelte/transition';
    import { ArrowUp } from 'carbon-icons-svelte';

    interface Props {
        title: string;
        content?: import('svelte').Snippet;
    }

    let { title, content }: Props = $props();

    let visible: boolean = $state(true);

    let toggleVisibility = () => {
        visible = !visible;
    }
</script>

<Tile containerClass="col-span-full">
    <div class="flex justify-between">
        <h3 class="text-xl font-normal">
            <button onclick={toggleVisibility}>
                {title}
            </button>
        </h3>
        <div class="transition-transform duration-300 flex items-center text-center " class:rotate-180={!visible}>
            <button onclick={toggleVisibility}>
                <ArrowUp size={16}/>
            </button>
        </div>
    </div>
    {#if visible}
        <div transition:slide={{ duration: 300 }}>
            <hr class="col-span-full my-4">
            {@render content?.()}
        </div>
    {/if}
</Tile>
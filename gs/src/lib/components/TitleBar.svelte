<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'
    import { Close, SubtractLarge, Maximize } from 'carbon-icons-svelte';
</script>

<div data-tauri-drag-region class="bg-surface-900 flex justify-between items-center p-1 px-2">
    <div class="flex items-center gap-2">
        <img src="/images/logo-green-new.png" alt="Delft logo" class="w-7"/>
        <span class="text-primary-500 font-medium text-xs">DELFT HYPERLOOP</span>
    </div>
    <div class="text-surface-300">
        <button onclick={appWindow.minimize} class="rounded-full bg-surface-800 text-primary-500 hover:bg-surface-700 p-1">
            <SubtractLarge size={16} />
        </button>
        <button onclick={appWindow.maximize} class="rounded-full bg-surface-800 text-primary-500 hover:bg-surface-700 p-1" >
            <Maximize size={16} />
        </button>
        <button onclick={appWindow.close} class="rounded-full bg-surface-800 text-primary-500 hover:bg-surface-700 p-1">
            <Close size={16} />
        </button>
    </div>
</div>
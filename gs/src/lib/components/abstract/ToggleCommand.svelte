<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri';
    import {EventChannel, type NamedCommand, util} from "$lib";
    
    interface Props {
        offCmd: NamedCommand;
        onCmd: NamedCommand;
        val?: number;
        callback?: (val:number) => void;
        disabled?: boolean;
        // for binding
        status?: boolean;
    }

    let {
        offCmd,
        onCmd,
        val = 0,
        callback = () => {},
        disabled = false,
        status = $bindable(false)
    }: Props = $props();

    const toggleOff = () => {
        invoke('send_command', {cmdName:offCmd, val}).then((r) => {
            if (r) {
                status = false;
                util.log(`send command ${offCmd} worked`, EventChannel.WARNING);
                callback(val);
            } else {
                util.log(`send command ${offCmd} failed`, EventChannel.WARNING);
            }
        });
    };

    const toggleOn = () => {
        invoke('send_command', {cmdName:onCmd, val}).then((r) => {
            if (r) {
                status = true;
                util.log(`send command ${onCmd} worked`, EventChannel.WARNING);
                callback(val);
            } else {
                util.log(`send command ${onCmd} failed`, EventChannel.WARNING);
            }
        });
    };
</script>

<div class="flex">
    <button class="btn rounded-none rounded-l-lg font-number py-2 text-surface-50 bg-surface-700"
            class:active={!status}
            onclick={toggleOff}
            disabled={disabled}>
        Off
    </button>
    <button class="btn rounded-none rounded-r-lg font-number py-2 text-surface-50 bg-surface-700"
            class:active={status}
            onclick={toggleOn}
            disabled={disabled}>
        On
    </button>
</div>

<style lang="scss">
  .active {
    background-color: rgb(77, 156, 137);
  }
</style>

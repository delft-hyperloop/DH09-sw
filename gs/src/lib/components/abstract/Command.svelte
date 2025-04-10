<script lang="ts">
    import {invoke} from '@tauri-apps/api/tauri';
    import {EventChannel, type NamedCommand, util} from "$lib";

    export let className: string = '';
    export let cmd: NamedCommand;
    // export let cmds: [NamedCommand] = ["DefaultCommand"];
    export let val: number | bigint = 0;
    // export let values: [number] = [0];
    export let callback: (val:number | bigint) => void = () => {};
    // export let callbacks: [(val: number) => void] = [() => {}];
    export let text: string = '';

    let send = async () => {
        // if (cmds.length > 0 && cmds[0] !== "DefaultCommand") {
        //     if (cmds.length != values.length || cmds.length != callbacks.length) {
        //         console.error("Different length arrays provided for sending multiple commands");
        //         return;
        //     }
        //     const cmd_val_callback: [string, number, (val: number) => void][] = [];
        //
        //     for (let i = 0; i < cmd.length; i++) {
        //         cmd_val_callback.push([cmd[i], values[i], callbacks[i] ?? (() => {})]);
        //     }
        //
        //     for ([c, v, b] of cmd_val_callback) {
        //         console.log(`Sending command: ${c}, value: ${v}`);
        //         await invoke('send_command', {cmdName: c, v}).then(() => {
        //             console.log(`Command ${c} sent`);
        //         }).catch((e) => {
        //             console.error(`Error sending command ${c}: ${e}`);
        //         });
        //         util.log(`Command ${c} sent`, EventChannel.INFO);
        //         if (typeof b === "function" && typeof v === "number") {
        //             b(v);
        //         } else {
        //             console.error(`callback or value with different types. callback: ${typeof b}; value: ${typeof v}`);
        //         }
        //     }
        // } else {
            console.log(`Sending command: ${cmd}, value: ${val}`);
            await invoke('send_command', {cmdName: cmd, val}).then(() => {
                console.log(`Command ${cmd} sent`);
            }).catch((e) => {
                console.error(`Error sending command ${cmd}: ${e}`);
            });
            util.log(`Command ${cmd} sent`, EventChannel.INFO);
            callback(val);
        // }
    };
</script>

<button class="btn rounded-md font-number font-medium text-wrap overflow-auto {className ? className : 'py-2 bg-primary-500 text-surface-900'}"
        on:click={send}>
    {text ? text : util.snakeToCamel(cmd)}
</button>

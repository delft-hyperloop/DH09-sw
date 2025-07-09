<script lang="ts">
    import {
        Command,
        Tile,
        TileGrid,
        Chart,
        util,
        EventChannel,
        type NamedCommand,
    } from '$lib';
    import { goingForward, propulsionConfigSent, usingTestTrack } from '$lib/stores/state';
    import { invoke } from '@tauri-apps/api/tauri';
    import PropulsionPoint from '$lib/components/PropulsionPoint.svelte';
    import { propulsionPoints } from '$lib/stores/data';
    import { onMount } from 'svelte';
    import type { PropPoint } from '$lib/types';

    let currentDirectionForward: boolean = $goingForward;
    let pointCount: number = 3;

    onMount(() => {
        let array: PropPoint[] = [];
        for (let i = 0; i < pointCount; i++) {
            array[i] = {
                location: 0,
                iq: 0,
                id: 0,
            }
        }
        propulsionPoints.set(array);
    })

    // Look here if values are wrong;
    async function submitRun() {
        goingForward.set(currentDirectionForward);

        let direction: number = 1;
        if (!$goingForward) {
            direction = 0;
        }

        const value1 = 0;
        const value2 = (direction << 16) | 0;

        await invoke('send_command_64_bits', {cmdName: "PPControlParams", vals: [value1, value2]}).then(() => {
            console.log(`Sending command: PPControlParams, value: ${value1} ${value2}`);
        }).catch((e) => {
            console.error(`Error sending command PPControlParams: ${e}`);
        });
        util.log(`Command PPControlParams sent`, EventChannel.INFO);

        // This is sending the run point params
        for (let i = 0; i < pointCount; i++) {
            const command: NamedCommand = `PPRunParameters${i === 0 ? 'B' : i}` as NamedCommand;

            // Look here if order of values is wrong
            const v1 = ($propulsionPoints[i].id & 0xFFFF) << 16;
            const v2 = ((($propulsionPoints[i].location * 10) & 0xFFFF) << 16) | ($propulsionPoints[i].iq & 0xFFFF);

            await invoke('send_command_64_bits', { cmdName: command, vals: [v2, v1]}).then(() => {
                console.log(`Sending command: ${command} with values: ${v1}, ${v2}`);
            }).catch((e) => {
                console.error(`Error sending command ${command}: ${e}`);
            });
            util.log(`Command ${command} sent`, EventChannel.INFO);
        }

        propulsionConfigSent.set(true);
    }

</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4 ">Initialization</h2>

    <TileGrid columns="1fr 1fr 1.5fr" rows="auto 1fr">
        <Tile containerClass="row-span-2 col-span-1" insideClass="flex flex-col gap-2" heading="Run Initialization">
            <div class="grid grid-cols-2 gap-2 row-span-2 col-span-1 p-4">
                <Command
                    cmd="PropulsionOn"
                    text="Enable Propulsion"
                    className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto"
                    dependency={propulsionConfigSent}
                    dependencyMessage="Run configuration was not sent! Can't start propulsion without specifying
                a direction, top speed and modulation factor!"
                    dependencyTitle="Configuration not sent!"
                />
                <Command text="Disable Propulsion" cmd="PropulsionOff" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <Command cmd="SystemReset" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />
                <Command cmd="RearmSDC" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" text="Rearm SDC"/>
                <p class="col-span-2">
                    Choose Direction:
                </p>
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-1 flex-grow font-medium overflow-hidden"
                        on:click={() => {currentDirectionForward = true}}
                        disabled={currentDirectionForward}>
                    Forward
                </button>
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-1 flex-grow font-medium overflow-hidden"
                        on:click={() => {currentDirectionForward = false}}
                        disabled={!currentDirectionForward}>
                    Backward
                </button>
                <p class="col-span-2">
                    Choose Track:
                </p>
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-1 flex-grow font-medium overflow-hidden text-wrap"
                        on:click={() => {usingTestTrack.set(true)}}
                        disabled={$usingTestTrack}>
                    Test Track
                </button>
                <button class="btn rounded-md bg-primary-500 text-surface-900 col-span-1 flex-grow font-medium overflow-hidden text-wrap"
                        on:click={() => {usingTestTrack.set(false)}}
                        disabled={!$usingTestTrack}>
                    EHC Track
                </button>
                <div class="flex gap-4 col-span-2 mt-4">
                    <div class="flex flex-col overflow-y-auto gap-2 w-full">
                        {#each Array.from({ length: pointCount }, (_, i) => i) as i}
                            <PropulsionPoint index={i}/>
                        {/each}
                    </div>
                </div>
                <button class="btn text-wrap rounded-md bg-primary-500 text-surface-900 col-span-full flex-grow font-medium overflow-hidden"
                        on:click={submitRun} disabled={false}>
                    Submit All Run Parameters
                </button>
                <hr class="col-span-full">
                <p class="col-span-full font-normal text-xl justify-center text-center pb-3 ">Current Values:</p>
                <p>Run Direction:</p>
                {#if $propulsionConfigSent}
                    {#if $goingForward}
                        <p>Forward</p>
                    {:else}
                        <p>Backward</p>
                    {/if}
                {:else}
                    <p>Not set</p>
                {/if}
            </div>
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="Velocity" background="bg-surface-900"/>
        </Tile>
        <Tile containerClass="col-span-2">
            <Chart title="Offset" background="bg-surface-900"/>
        </Tile>
    </TileGrid>
    <TileGrid columns="1fr 1fr" rows="auto" className="mt-2">
        <Tile containerClass="col-span-1">
            <Chart title="Propulsion Log 1 - Right Motor" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Propulsion Log 1 - Left Motor" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Propulsion Log 2 - Right Motor" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Propulsion Log 2 - Left Motor" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Propulsion Log 3 - Right Motor" background="bg-surface-900" />
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Propulsion Log 3 - Left Motor" background="bg-surface-900" />
        </Tile>
    </TileGrid>
</div>

<style>
    :root {
        --range-slider:            hsl(165.6, 33.9%, 45.7%);
        --range-handle-inactive:   hsl(165.6, 33.9%, 45.7%);
        --range-handle:            hsl(165.6, 33.9%, 45.7%);
        --range-handle-focus:      hsl(165.6, 33.9%, 45.7%);
        --range-handle-border:     hsl(180, 5.2%, 33.9%);
        --range-range-inactive:    hsl(180, 5.2%, 33.9%);
        --range-range:             hsl(180, 5.2%, 33.9%);
        --range-range-limit:       hsl(180, 5.2%, 33.9%);
        --range-inactive:          hsl(180, 5.2%, 33.9%);

        --range-pip:               hsl(180, 5.2%, 33.9%);
        --range-pip-text:          hsl(180, 5.2%, 33.9%);
        --range-pip-active:        hsl(165.6, 33.9%, 45.7%);
        --range-pip-active-text:   hsl(165.6, 33.9%, 45.7%);
        --range-pip-hover:         hsl(165.6, 33.9%, 45.7%);
        --range-pip-hover-text:    hsl(165.6, 33.9%, 45.7%);
        --range-pip-in-range:      hsl(180, 5.2%, 33.9%);
        --range-pip-in-range-text: hsl(180, 5.2%, 33.9%);
        --range-pip-out-of-limit:      hsl(180, 5.2%, 33.9%);
        --range-pip-out-of-limit-text: hsl(180, 5.2%, 33.9%);
    }
</style>

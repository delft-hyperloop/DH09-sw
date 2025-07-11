<script lang="ts">
    import {
        Command,
        Tile,
        TileGrid,
        Chart,
        util,
        EventChannel,
        type NamedCommand, GrandDataDistributor,
    } from '$lib';
    import {
        goingForward,
        propControlWord1,
        propControlWord2,
        propulsionConfigSent,
        usingTestTrack,
    } from '$lib/stores/state';
    import { invoke } from '@tauri-apps/api/tauri';
    import PropulsionPoint from '$lib/components/PropulsionPoint.svelte';
    import { propulsionPoints, setPropulsionPoints } from '$lib/stores/data';
    import { onMount } from 'svelte';
    import type { PropPoint } from '$lib/types';
    import { Meter } from 'carbon-icons-svelte';
    import PropulsionPointDisplay from '$lib/components/PropulsionPointDisplay.svelte';
    import PropulsionHeartbeat from '$lib/components/PropulsionHeartbeat.svelte';
    import PropulsionInitFault from '$lib/components/PropulsionInitFault.svelte';
    import BinaryInput1 from '$lib/components/BinaryInput1.svelte';
    import BinaryInput2 from '$lib/components/BinaryInput2.svelte';

    let currentDirectionForward: boolean = $goingForward;
    let pointCount: number = 3;

    const storeManager = GrandDataDistributor.getInstance().stores;
    const ppEmergency1 = storeManager.getWritable("PPEmergency1");
    const ppEmergency2 = storeManager.getWritable("PPEmergency2");

    onMount(() => {
        let array: PropPoint[] = [];
        for (let i = 0; i < pointCount; i++) {
            array[i] = {
                location: 0,
                imax: 0,
            }
        }
        propulsionPoints.set(array);
    })

    const propLabels: string[] = [
        "Reset",
        "Phase Enable 1",
        "Phase Enable 2",
        "Phase Enable 3",
        "Logic Enable 1",
        "Logic Enable 2",
        "Logic Enable 3",
    ].reverse();

    const propReadLabels: string[] = [
        "Direction",
        "General Fault",
        "State",
    ].reverse().concat(propLabels);

    let currentTrack = $usingTestTrack;

    // Look here if values are wrong;
    async function submitRun() {
        goingForward.set(currentDirectionForward);
        setPropulsionPoints.set($propulsionPoints);
        currentTrack = $usingTestTrack;

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
            // const v1 = ($propulsionPoints[i].id & 0xFFFF) << 16;
            const v2 = ((($propulsionPoints[i].location * 10) & 0xFFFF) << 16) | ($propulsionPoints[i].imax & 0xFFFF);

            await invoke('send_command', { cmdName: command, vals: v2}).then(() => {
                console.log(`Sending command: ${command} with value: ${v2}`);
            }).catch((e) => {
                console.error(`Error sending command ${command}: ${e}`);
            });
            util.log(`Command ${command} sent`, EventChannel.INFO);
        }

        propulsionConfigSent.set(true);
    }

</script>

<div class="p-4 h-full">
    <h2 class="text-2xl font-semibold mb-4">Initialization</h2>

    <TileGrid columns="1fr 1fr 1fr" rows="auto 1fr">
        <Tile insideClass="flex flex-row gap-4" containerClass="col-span-full">
            <Command
                cmd="PropulsionOn"
                icon={Meter}
                dependency={propulsionConfigSent}
                dependencyMessage="Run configuration was not sent! Can't start propulsion without specifying
                                            a direction, top speed and modulation factor!"
                dependencyTitle="Configuration not sent!"
            />
            <Command cmd="MotorBrake" icon={Meter} iconClass="scale-x-[-1]"/>
        </Tile>
        <Tile containerClass="row-span-2 col-span-full" insideClass="flex flex-col gap-2" heading="Run Initialization" headingOnLeft={true}>
            <div class="grid grid-cols-5 grid-rows-1">
                <div class="grid grid-cols-2 gap-2 row-span-2 col-span-2 py-4">
                    <!--                <Command cmd="SystemReset" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" />-->
                    <!--                <Command cmd="RearmSDC" className="btn flex-grow rounded-md bg-primary-500 text-surface-900 text-wrap overflow-auto" text="Rearm SDC"/>-->
<!--                    <p class="col-span-2 text-center w-full items-center h-full justify-center">-->
<!--                        Choose Direction:-->
<!--                    </p>-->
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
<!--                    <p class="col-span-2 text-center w-full items-center h-full justify-center">-->
<!--                        Choose Track:-->
<!--                    </p>-->
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
                </div>
                <div class="col-span-3 h-full flex justify-center items-center pl-2">
                    <div class="grid grid-cols-3 overflow-y-auto gap-2 w-full h-full">
                        {#each Array.from({ length: pointCount }, (_, i) => i) as i}
                            <PropulsionPoint index={i}/>
                        {/each}
                    </div>
                </div>
            </div>
            <button class="btn text-wrap rounded-md bg-primary-500 text-surface-900 col-span-full flex-grow font-medium overflow-hidden"
                    on:click={submitRun} disabled={false}>
                Submit All Run Parameters
            </button>
<!--            <hr class="col-span-full">-->
            <p class="col-span-full font-normal text-xl py-3 ">Current Values:</p>
            <div class="grid grid-cols-4 gap-8">
                <div class="flex flex-col gap-4">
                    <span>
                        Run Direction:
                                {#if $propulsionConfigSent}
                            {#if $goingForward}
                                <span>Forward</span>
                            {:else}
                                <span>Backward</span>
                            {/if}
                        {:else}
                            <span>Not set</span>
                        {/if}
                    </span>
                            <span>
                        Track:
                                {#if $propulsionConfigSent}
                            {#if currentTrack}
                                <span>Test Track</span>
                            {:else}
                                <span>EHC Track</span>
                            {/if}
                        {:else}
                            <span>Not Set</span>
                        {/if}
                    </span>
                </div>
                {#each Array.from({ length: pointCount }, (_, i) => i) as i}
                    <PropulsionPointDisplay index={i}/>
                {/each}
            </div>
        </Tile>
    </TileGrid>
    <TileGrid columns="1fr 1fr" rows="auto" className="mt-2">
        <Tile containerClass="col-span-full" heading="Other Commands" headingOnLeft={true}>
            <div class="border-surface-600 border-[1px] rounded-lg m-4 p-2">
                <div class="m-4">
                    <PropulsionHeartbeat storeName="Word1" labels={propReadLabels}/>
                </div>
                <div class="grid grid-cols-9 gap-2 items-center m-4">
                    <span class="text-center">Propulsion Control Word 1:</span>
                    {#each Array.from({ length: propLabels.length }, (_, i) => propLabels.length - 1 - i) as i}
                        <BinaryInput1 index={i} />
                    {/each}
                    <Command cmd="SendPropulsionControlWord1" val={$propControlWord1} text={"Send"}/>

                    <span/>
                    {#each propLabels as l}
                        <span class="text-center">{l}</span>
                    {/each}
                    <span/>
                </div>
                <div class="m-4 mt-8">
                    <PropulsionInitFault storeName="PPInitFault1"/>
                </div>
                <div class="m-4 text-center">
                    <span>Emergency 1: {$ppEmergency1.value}</span>
                </div>
                <!--                    <div class="m-4">-->
                <!--                        <span>Init fault 1: {$propInitFault1.value}</span>-->
                <!--                    </div>-->
            </div>
            <div class="border-surface-600 border-[1px] rounded-lg m-4 p-2">
                <div class="m-4 mt-8">
                    <PropulsionHeartbeat storeName="Word2" labels={propReadLabels}/>
                </div>
                <div class="grid grid-cols-9 gap-2 items-center m-4">
                    <span class="text-center">Propulsion Control Word 2:</span>
                    {#each Array.from({ length: propLabels.length }, (_, i) => propLabels.length - 1 - i) as i}
                        <BinaryInput2 index={i} />
                    {/each}
                    <Command cmd="SendPropulsionControlWord2" val={$propControlWord2} text={"Send"}/>

                    <span/>
                    {#each propLabels as l}
                        <span class="text-center">{l}</span>
                    {/each}
                    <span/>
                </div>
                <div class="m-4 mt-8">
                    <PropulsionInitFault storeName="PPInitFault2"/>
                </div>
                <div class="m-4 text-center">
                    <span>Emergency 2: {$ppEmergency2.value}</span>
                </div>
                <!--                    <div class="m-4 mt-8">-->
                <!--                        <span>Init fault 2: {$propInitFault2.value}</span>-->
                <!--                    </div>-->
            </div>
        </Tile>
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
            <Chart title="Motor Temperatures Left" background="bg-surface-900"/>
        </Tile>
        <Tile containerClass="col-span-1">
            <Chart title="Motor Temperatures Right" background="bg-surface-900"/>
        </Tile>
        <Tile containerClass="col-span-full">
            <Chart title="Velocity" background="bg-surface-900"/>
        </Tile>
        <Tile containerClass="col-span-full">
            <Chart title="Offset" background="bg-surface-900"/>
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

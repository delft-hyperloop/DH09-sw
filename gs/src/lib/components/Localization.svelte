<script lang="ts">

    import { goingForward } from '$lib/stores/state';
    import { GrandDataDistributor } from '$lib';
    import { trackLength } from '$lib/stores/data';
    
    export let showLabels: boolean = true;

    const storeManager = GrandDataDistributor.getInstance().stores;
    // 100_000 received from the localization sensor is 1 meter
    const location = storeManager.getWritable("Localization");

    // SVG elements
    let progress_container: SVGGElement;
    let progress: SVGPathElement;
    let path: SVGGElement;

    // points
    let point_start: SVGCircleElement;
    let point_quarter: SVGCircleElement;
    let point_half: SVGCircleElement;
    let point_three_quarters: SVGCircleElement;
    let point_end: SVGCircleElement;

    // screen coords for points
    const x_coord_start = 15;
    const x_coord_end = 905;
    const y_coord = 26;
    const x_coord_labels = 100;

    const color_active = "#4D9C89";
    const color_off = "#525B5B";

    $: if (progress) {
        let path_length: number = progress.getTotalLength();

        path.style.strokeDasharray = path_length.toString();
        point_half.style.fill = ($location.value / 100) >  $trackLength / 2 ? color_active : color_off;
        progress.style.strokeDashoffset = (path_length - path_length * (($location.value / 100) /  $trackLength)).toString();

        if ($goingForward) {
            point_start.style.fill = ($location.value / 100) > 0 ? color_active : color_off;
            point_quarter.style.fill = ($location.value / 100) >  $trackLength / 4 ? color_active : color_off;
            point_three_quarters.style.fill = ($location.value / 100) >  $trackLength / 4 * 3 ? color_active : color_off;
            point_end.style.fill = ($location.value / 100) >=  $trackLength ? color_active : color_off;
        } else {
            point_end.style.fill = ($location.value / 100) > 0 ? color_active : color_off;
            point_three_quarters.style.fill = ($location.value / 100) >  $trackLength / 4 ? color_active : color_off;
            point_quarter.style.fill = ($location.value / 100) >  $trackLength / 4 * 3 ? color_active : color_off;
            point_start.style.fill = ($location.value / 100) >=  $trackLength ? color_active : color_off;
        }
    }

</script>
<div class="w-full">
    <svg viewBox="0 0 920 60" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g id="localiser" transform="translate(0, 2)">
            <g id="track">
                <g id="progress_container" bind:this={progress_container}>
                    <g id="path" bind:this={path}>
                        <path d="M10 26H909" stroke="#525B5B" stroke-width="6" />
                        {#if $goingForward}
                            <path bind:this={progress} d="M10 25.7224H909" stroke="#4D9C89" stroke-width="6" />
                        {:else}
                            <path bind:this={progress} d="M909 25.7224H10" stroke="#4D9C89" stroke-width="6" />
                        {/if}
                    </g>
                </g>
                <g id="point">
                    <circle bind:this={point_start} cx={x_coord_start} cy={y_coord} r="10" fill="#525B5B" />
                    <circle bind:this={point_quarter} cx={(x_coord_end - x_coord_start) / 4 + x_coord_start} cy={y_coord} r="7.5" fill="#525B5B" />
                    <circle bind:this={point_half} cx={(x_coord_end - x_coord_start) / 2 + x_coord_start} cy={y_coord} r="7.5" fill="#525B5B" />
                    <circle bind:this={point_three_quarters} cx={(x_coord_end - x_coord_start) / 4 * 3 + x_coord_start} cy={y_coord} r="7.5" fill="#525B5B" />
                    <circle bind:this={point_end} cx={x_coord_end} cy={y_coord} r="10" fill="#525B5B" />
                </g>
            </g>
            <g id="labels">
                {#if showLabels}
                    <text x="3" y="55" fill="#EFF0F0" font-size="14">Start</text>
                    <text x="895" y="55" fill="#EFF0F0" font-size="12">End</text>
                    <text x={x_coord_labels + 5} y="8" fill="#EFF0F0" font-size="12">Forward</text>
                    <text x={x_coord_labels + 1} y="52" fill="#EFF0F0" font-size="12">Backward</text>
                {/if}
            </g>
        </g>
        <g filter="url(#filter0_d_64_411)">
            <svg xmlns="http://www.w3.org/2000/svg" width="160" height="64" fill="none" viewBox="0 0 160 64">
                <g transform="translate(120, 14)">
                    <path
                        fill="#4D9C89"
                        d="M19.354 4.354a.5.5 0 0 0 0-.708L16.172.464a.5.5 0 1 0-.708.708L18.293 4l-2.828 2.828a.5.5 0 1 0 .707.708zM0 4.5h19v-1H0z"
                    ></path>
                </g>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" width="160" height="64" fill="none" viewBox="0 0 160 64">
                <g transform="translate(139, 34) scale(-1, 1)">
                    <path
                        fill="#4D9C89"
                        d="M19.354 4.354a.5.5 0 0 0 0-.708L16.172.464a.5.5 0 1 0-.708.708L18.293 4l-2.828 2.828a.5.5 0 1 0 .707.708zM0 4.5h19v-1H0z"
                    ></path>
                </g>
            </svg>
        </g>
        <defs>
            <filter
                id="filter0_d_64_411"
                x="-4"
                y="0"
                width="927"
                height="156"
                filterUnits="userSpaceOnUse"
                color-interpolation-filters="sRGB"
            >
                <feFlood flood-opacity="0" result="BackgroundImageFix" />
                <feColorMatrix
                    in="SourceAlpha"
                    type="matrix"
                    values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                    result="hardAlpha"
                />
                <feOffset dy="4" />
                <feGaussianBlur stdDeviation="2" />
                <feComposite in2="hardAlpha" operator="out" />
                <feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0" />
                <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_64_411" />
                <feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow_64_411" result="shape" />
            </filter>
        </defs>
    </svg>
</div>
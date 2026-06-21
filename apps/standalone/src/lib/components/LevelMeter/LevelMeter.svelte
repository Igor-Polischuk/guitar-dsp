<script lang="ts">
    import { onDestroy } from "svelte";

    export let label: string;
    export let value = -60;
    export let min = -60;
    export let max = 0;
    export let unit = "dB";
    export let clip = false;
    export let labelSmoothing = 0.12;
    export let ticks = [0, -6, -12, -18, -24, -30, -42, -48, -60];

    const barCount = 80;
    const bars = Array.from({ length: barCount });
    let displayValue = value;
    let previousValue = value;
    let smoothingFrame = 0;

    function clamp(next: number) {
        return Math.max(min, Math.min(max, next));
    }

    function normalize(next: number) {
        const range = max - min;

        if (range <= 0) {
            return 0;
        }

        return (clamp(next) - min) / range;
    }

    function tickOffset(tick: number) {
        return `${(1 - normalize(tick)) * 100}%`;
    }

    function formatValue(next: number) {
        return `${next.toFixed(0)} ${unit}`;
    }

    function startSmoothing() {
        if (smoothingFrame) {
            return;
        }

        smoothingFrame = requestAnimationFrame(smoothDisplayValue);
    }

    function smoothDisplayValue() {
        const distance = value - displayValue;
        const step = distance * labelSmoothing;

        if (Math.abs(distance) < 0.05) {
            displayValue = value;
            smoothingFrame = 0;
            return;
        }

        displayValue += step;
        smoothingFrame = requestAnimationFrame(smoothDisplayValue);
    }

    $: normalizedValue = normalize(value);
    $: activeBars = Math.round(normalizedValue * barCount);
    $: clipped = clip || value >= max;
    $: if (value !== previousValue) {
        previousValue = value;
        startSmoothing();
    }

    onDestroy(() => {
        if (smoothingFrame) {
            cancelAnimationFrame(smoothingFrame);
        }
    });
</script>

<aside class="meter-card" aria-label={`${label} level meter`}>
    <div class="meter-label">{label}</div>
    <div class="clip-row" class:active={clipped}>
        <span>CLIP</span>
        <i></i>
    </div>

    <div class="meter-body">
        <div class="bar-stack" aria-hidden="true">
            {#each bars as _, index}
                {@const percent = (index + 1) / barCount}
                <span
                    class:lit={index < activeBars}
                    class:mid={percent >= 0.5 && percent < 0.9}
                    class:hot={percent >= 0.9}
                ></span>
            {/each}
        </div>
        <div class="scale" aria-hidden="true">
            {#each ticks as tick}
                <span style:top={tickOffset(tick)}>{tick}</span>
            {/each}
        </div>
    </div>

    <div class="meter-value">{formatValue(displayValue)}</div>
</aside>

<style>
    .meter-card {
        display: grid;
        grid-template-rows: auto auto minmax(0, 1fr) auto;
        gap: 0.55rem;
        width: 7.25rem;
        min-height: 34rem;
        padding: 1rem 0.62rem 0.85rem;
        border: var(--border-panel);
        border-radius: var(--radius-panel);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.035), transparent),
            var(--color-panel-deep);
        box-shadow:
            inset 0 1px 0 rgba(255, 255, 255, 0.035),
            0 10px 30px rgba(0, 0, 0, 0.28);
    }

    .meter-label {
        color: var(--color-text-soft);
        font-size: 0.78rem;
        font-weight: 740;
        letter-spacing: 0.06em;
        text-align: center;
    }

    .clip-row {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.6rem;
        color: rgba(240, 91, 79, 0.1);
        font-size: 0.68rem;
        font-weight: 760;
    }

    .clip-row i {
        width: 0.22rem;
        height: 0.85rem;
        border-radius: 999px;
        background: rgba(240, 91, 79, 0.1);
    }

    .clip-row.active {
        color: var(--color-accent-red);
    }

    .clip-row.active i {
        background: var(--color-accent-red);
        box-shadow: 0 0 8px rgba(240, 91, 79, 0.42);
    }

    .meter-body {
        display: grid;
        grid-template-columns: minmax(1.25rem, 1fr) 1.65rem;
        gap: 0.58rem;
        align-items: stretch;
        min-height: 0;
    }

    .bar-stack {
        display: flex;
        flex-direction: column-reverse;
        justify-content: space-between;
        align-items: center;
        min-height: 100%;
        padding: 0;
    }

    .bar-stack span {
        display: block;
        width: 1.08rem;
        height: 0.1rem;
        border-radius: 999px;
        background: rgba(71, 88, 101, 0.22);
    }

    .bar-stack span.lit {
        background: linear-gradient(
            90deg,
            var(--color-accent-green),
            var(--color-meter-lime)
        );
        box-shadow: 0 0 8px rgba(22, 215, 161, 0.14);
    }

    .bar-stack span.lit.mid {
        background: linear-gradient(
            90deg,
            var(--color-meter-lime),
            var(--color-meter-yellow)
        );
        box-shadow: 0 0 8px rgba(246, 241, 70, 0.18);
    }

    .bar-stack span.lit.hot {
        background: linear-gradient(
            90deg,
            var(--color-meter-yellow),
            var(--color-accent-red)
        );
        box-shadow: 0 0 10px rgba(240, 91, 79, 0.24);
    }

    .scale {
        position: relative;
        min-height: 0;
        color: var(--color-text-muted);
        font-size: 0.68rem;
        font-weight: 620;
        line-height: 1;
        text-align: right;
    }

    .scale span {
        position: absolute;
        right: 0;
        transform: translateY(-50%);
    }

    .meter-value {
        color: var(--color-accent-green);
        font-size: 0.8rem;
        font-weight: 760;
        text-align: center;
    }

    @media (max-width: 980px) {
        .meter-card {
            width: min(100%, 12rem);
            min-height: 14rem;
        }

        .meter-body {
            min-height: 9rem;
        }
    }

    @media (max-width: 680px) {
        .meter-card {
            width: 100%;
            min-height: 16rem;
        }
    }
</style>

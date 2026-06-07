<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    type Accent = "green" | "cyan" | "blue" | "purple";

    export let label: string;
    export let value: number;
    export let min: number;
    export let max: number;
    export let step: number;
    export let accent: Accent = "green";
    export let formatter: (value: number) => string = (next) => String(next);

    const dragSensitivity = 170;

    let currentValue = value;
    let isDragging = false;
    let startX = 0;
    let startY = 0;
    let startValue = 0;

    function clamp(next: number) {
        return Math.max(min, Math.min(max, next));
    }

    function stepPrecision() {
        const [, decimals = ""] = String(step).split(".");
        return decimals.length;
    }

    function snap(next: number) {
        if (step <= 0) {
            return clamp(next);
        }

        const stepped = min + Math.round((clamp(next) - min) / step) * step;
        return Number(clamp(stepped).toFixed(stepPrecision()));
    }

    function setValue(next: number) {
        currentValue = snap(next);
        invoke("update_parameter", { label, value: currentValue });
    }

    function handleWheel(event: WheelEvent) {
        event.preventDefault();

        const direction = event.deltaY > 0 ? -1 : 1;
        const multiplier = event.shiftKey ? 5 : 1;

        setValue(currentValue + direction * step * multiplier);
    }

    function handlePointerDown(event: PointerEvent) {
        event.preventDefault();

        isDragging = true;
        startX = event.clientX;
        startY = event.clientY;
        startValue = currentValue;

        (event.currentTarget as HTMLDivElement).setPointerCapture(
            event.pointerId,
        );
    }

    function handlePointerMove(event: PointerEvent) {
        if (!isDragging) {
            return;
        }

        const deltaY = startY - event.clientY;
        const deltaX = event.clientX - startX;
        const movement = deltaY + deltaX * 0.35;
        const range = max - min;

        setValue(startValue + (movement / dragSensitivity) * range);
    }

    function handlePointerEnd(event: PointerEvent) {
        isDragging = false;

        const target = event.currentTarget as HTMLDivElement;
        if (target.hasPointerCapture(event.pointerId)) {
            target.releasePointerCapture(event.pointerId);
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Home") {
            event.preventDefault();
            setValue(min);
            return;
        }

        if (event.key === "End") {
            event.preventDefault();
            setValue(max);
            return;
        }

        const keySteps: Record<string, number> = {
            ArrowUp: 1,
            ArrowRight: 1,
            ArrowDown: -1,
            ArrowLeft: -1,
            PageUp: 10,
            PageDown: -10,
        };

        const direction = keySteps[event.key];
        if (direction === undefined) {
            return;
        }

        event.preventDefault();
        setValue(currentValue + direction * step);
    }

    $: safeValue = snap(currentValue);
    $: safeAmount = ((safeValue - min) / (max - min)) * 100;
    $: arc = (safeAmount / 100) * 280;
    $: angle = -140 + arc;
    $: displayValue = formatter(safeValue);
</script>

<div class="knob-control accent-{accent}">
    <div class="knob-label">{label}</div>
    <div
        class="dial"
        class:dragging={isDragging}
        style="--knob-arc: {arc}deg; --knob-angle: {angle}deg;"
        role="slider"
        tabindex="0"
        aria-label={label}
        aria-valuemin={min}
        aria-valuemax={max}
        aria-valuenow={safeValue}
        aria-valuetext={displayValue}
        onkeydown={handleKeydown}
        onpointercancel={handlePointerEnd}
        onpointerdown={handlePointerDown}
        onpointermove={handlePointerMove}
        onpointerup={handlePointerEnd}
        onwheel={handleWheel}
    >
        <div class="cap">
            <span class="needle"></span>
        </div>
    </div>
    <div class="knob-value">{displayValue}</div>
</div>

<style>
    .knob-control {
        --knob-accent: var(--color-accent-green);
        display: grid;
        justify-items: center;
        gap: 0.5rem;
        min-width: 5.8rem;
        color: var(--color-text-soft);
    }

    .accent-green {
        --knob-accent: var(--color-accent-green);
    }

    .accent-cyan {
        --knob-accent: var(--color-accent-cyan);
    }

    .accent-blue {
        --knob-accent: var(--color-accent-blue);
    }

    .accent-purple {
        --knob-accent: var(--color-accent-purple);
    }

    .knob-label {
        color: var(--color-text-soft);
        font-size: 0.78rem;
        font-weight: 680;
        letter-spacing: 0.08em;
        line-height: 1;
    }

    .dial {
        position: relative;
        display: grid;
        width: clamp(4.95rem, 7.4vw, 6.35rem);
        aspect-ratio: 1;
        place-items: center;
        border-radius: 50%;
        background: conic-gradient(
            from 220deg,
            var(--knob-accent) 0deg var(--knob-arc),
            rgba(95, 115, 132, 0.24) var(--knob-arc) 280deg,
            transparent 280deg 360deg
        );
        cursor: grab;
        filter: drop-shadow(0 12px 17px var(--color-shadow));
        outline: none;
        touch-action: none;
        user-select: none;
        transition: filter 140ms ease;
    }

    .dial:hover,
    .dial:focus-visible,
    .dial.dragging {
        filter: drop-shadow(0 12px 17px var(--color-shadow))
            drop-shadow(
                0 0 12px color-mix(in srgb, var(--knob-accent), transparent 64%)
            );
    }

    .dial:focus-visible {
        box-shadow: 0 0 0 2px
            color-mix(in srgb, var(--knob-accent), transparent 36%);
    }

    .dial.dragging {
        cursor: grabbing;
    }

    .dial::before {
        content: "";
        position: absolute;
        inset: 0.42rem;
        border-radius: inherit;
        background:
            radial-gradient(
                circle at 42% 30%,
                rgba(255, 255, 255, 0.12),
                transparent 31%
            ),
            linear-gradient(145deg, #303b49, #161f29 70%);
        box-shadow:
            inset 0 1px 3px rgba(255, 255, 255, 0.12),
            inset 0 -10px 18px rgba(0, 0, 0, 0.45),
            0 0 0 1px rgba(0, 0, 0, 0.45);
    }

    .dial::after {
        content: "";
        position: absolute;
        inset: 0.18rem;
        border-radius: inherit;
        border: 2px solid rgba(9, 14, 20, 0.62);
        box-shadow:
            inset 0 0 0 1px rgba(255, 255, 255, 0.04),
            0 0 0 1px rgba(0, 0, 0, 0.34);
    }

    .cap {
        position: relative;
        z-index: 1;
        width: 68%;
        aspect-ratio: 1;
        border-radius: 50%;
    }

    .needle {
        position: absolute;
        top: 8%;
        left: calc(50% - 0.12rem);
        width: 0.24rem;
        height: 0.5rem;
        border-radius: 999px;
        background: var(--knob-accent);
        box-shadow: 0 0 11px
            color-mix(in srgb, var(--knob-accent), transparent 35%);
        transform: rotate(var(--knob-angle)) translateY(-0.2rem);
        transform-origin: 50% 205%;
    }

    .knob-value {
        min-height: 1.25rem;
        color: var(--color-text-soft);
        font-size: 0.88rem;
        font-weight: 560;
        line-height: 1.15;
        white-space: nowrap;
    }

    .accent-green .knob-value,
    .accent-cyan .knob-value {
        color: var(--knob-accent);
    }

    @media (max-width: 620px) {
        .knob-control {
            min-width: 5.2rem;
        }

        .dial {
            width: 4.85rem;
        }
    }
</style>

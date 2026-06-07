<script lang="ts">
    import { Lock } from "@lucide/svelte";

    export let label: string;
    export let value: string;
    export let activeBars = 23;
    export let secondaryBars = 19;

    const bars = Array.from({ length: 34 });
    const scale = ["0", "-6", "-12", "-18", "-24", "-30", "-42", "-48", "-60"];

    $: firstLevel = Math.max(0, Math.min(activeBars, bars.length));
    $: secondLevel = Math.max(0, Math.min(secondaryBars, bars.length));
    $: isInput = label.toUpperCase() === "INPUT";
</script>

<aside class="meter-card" aria-label={`${label} level meter`}>
    <div class="meter-label">{label}</div>
    <div class="clip-row">
        <span>CLIP</span>
        <i></i>
    </div>
    <div class="meter-body">
        <div class="bar-pair" aria-hidden="true">
            <div class="bar-stack">
                {#each bars as _, index}
                    <span
                        class:lit={index < firstLevel}
                        class:warm={index >= 20}
                        class:hot={index >= 28}
                    ></span>
                {/each}
            </div>
            <div class="bar-stack">
                {#each bars as _, index}
                    <span
                        class:lit={index < secondLevel}
                        class:warm={index >= 20}
                        class:hot={index >= 28}
                    ></span>
                {/each}
            </div>
        </div>
        <div class="scale" aria-hidden="true">
            {#each scale as tick}
                <span>{tick}</span>
            {/each}
        </div>
    </div>
    <div class="meter-value">{value}</div>

    {#if isInput}
        <div class="meter-actions">
            <button class="mode active" type="button">Hi-Z</button>
            <button class="lock" type="button" aria-label="Input lock" title="Input lock">
                <Lock size={14} />
            </button>
        </div>
        <div class="meter-strip">
            <span>NOISE GATE</span>
            <i></i>
        </div>
    {:else}
        <button class="mute" type="button">MUTE</button>
        <div class="meter-strip stereo">
            <i></i>
            <span>STEREO</span>
        </div>
    {/if}
</aside>

<style>
    .meter-card {
        display: grid;
        grid-template-rows: auto auto minmax(18rem, 1fr) auto auto auto;
        gap: 0.55rem;
        width: 10.6rem;
        min-height: 34rem;
        padding: 1rem 0.75rem 0.65rem;
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
        gap: 0.75rem;
        color: var(--color-accent-red);
        font-size: 0.68rem;
        font-weight: 760;
    }

    .clip-row i {
        width: 0.22rem;
        height: 0.85rem;
        border-radius: 999px;
        background: var(--color-accent-red);
        box-shadow: 0 0 8px rgba(240, 91, 79, 0.42);
    }

    .meter-body {
        display: grid;
        grid-template-columns: 1fr auto;
        gap: 0.7rem;
        align-items: stretch;
        min-height: 0;
    }

    .bar-pair {
        display: flex;
        justify-content: center;
        gap: 0.55rem;
        min-width: 0;
    }

    .bar-stack {
        display: flex;
        flex-direction: column-reverse;
        justify-content: flex-start;
        gap: 0.18rem;
        padding: 0.1rem 0;
    }

    .bar-stack span {
        display: block;
        width: 1.12rem;
        height: 0.16rem;
        border-radius: 999px;
        background: rgba(71, 88, 101, 0.26);
    }

    .bar-stack span.lit {
        background: linear-gradient(90deg, var(--color-accent-green), var(--color-meter-lime));
        box-shadow: 0 0 9px rgba(22, 215, 161, 0.15);
    }

    .bar-stack span.lit.warm {
        background: linear-gradient(90deg, var(--color-meter-lime), var(--color-meter-yellow));
        box-shadow: 0 0 9px rgba(246, 241, 70, 0.18);
    }

    .bar-stack span.lit.hot {
        background: var(--color-meter-yellow);
        box-shadow: 0 0 11px rgba(246, 241, 70, 0.24);
    }

    .scale {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        color: var(--color-text-muted);
        font-size: 0.7rem;
        font-weight: 620;
        line-height: 1;
        text-align: right;
    }

    .meter-value {
        color: var(--color-accent-green);
        font-size: 0.8rem;
        font-weight: 760;
        text-align: center;
    }

    .meter-actions {
        display: grid;
        grid-template-columns: 1fr 2.25rem;
        gap: 0.45rem;
    }

    .mode,
    .lock,
    .mute {
        min-height: 2rem;
        border: 1px solid var(--color-control-border);
        border-radius: var(--radius-control);
        background: rgba(20, 29, 39, 0.72);
        color: var(--color-text-soft);
        cursor: pointer;
        font-size: 0.76rem;
        font-weight: 720;
        letter-spacing: 0.04em;
    }

    .mode.active {
        border-color: rgba(47, 134, 255, 0.45);
        background: rgba(47, 134, 255, 0.14);
        color: #65aeff;
    }

    .lock {
        display: grid;
        place-items: center;
    }

    .mute {
        justify-self: center;
        width: 5rem;
    }

    .meter-strip {
        display: flex;
        align-items: center;
        justify-content: space-between;
        min-height: 2.05rem;
        margin: 0 -0.25rem -0.25rem;
        padding: 0 0.8rem;
        border-top: 1px solid var(--color-panel-line);
        color: var(--color-text-muted);
        font-size: 0.68rem;
        font-weight: 740;
        letter-spacing: 0.03em;
    }

    .meter-strip i {
        width: 0.48rem;
        aspect-ratio: 1;
        border-radius: 50%;
        background: var(--color-accent-green);
        box-shadow: 0 0 9px rgba(22, 215, 161, 0.42);
    }

    .meter-strip.stereo {
        justify-content: flex-start;
        gap: 1.1rem;
        font-size: 0.78rem;
    }

    @media (max-width: 980px) {
        .meter-card {
            width: min(100%, 28rem);
            min-height: 12rem;
            grid-template-rows: auto auto 1fr auto;
            grid-template-columns: 1fr auto;
            align-items: center;
        }

        .meter-label,
        .clip-row {
            grid-column: 1 / -1;
        }

        .meter-body {
            min-height: 7rem;
        }

        .meter-actions,
        .mute,
        .meter-strip {
            grid-column: 2;
        }
    }

    @media (max-width: 620px) {
        .meter-card {
            grid-template-columns: 1fr;
            width: 100%;
        }

        .meter-actions,
        .mute,
        .meter-strip {
            grid-column: auto;
        }
    }
</style>

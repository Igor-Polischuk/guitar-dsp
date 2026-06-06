<script lang="ts">
    export let label: string;
    export let value: string;
    export let activeBars = 23;

    const bars = Array.from({ length: 34 });
    const scale = ["0", "-6", "-12", "-18", "-24", "-30", "-36", "-42", "-48", "-54", "-60"];
</script>

<aside class="meter-card" aria-label={`${label} level meter`}>
    <div class="meter-label">{label}</div>
    <div class="meter-body">
        <div class="bar-stack" aria-hidden="true">
            {#each bars as _, index}
                <span
                    class:lit={index < activeBars}
                    class:warm={index >= 20}
                    class:hot={index >= 27}
                ></span>
            {/each}
        </div>
        <div class="scale" aria-hidden="true">
            {#each scale as tick}
                <span>{tick}</span>
            {/each}
        </div>
    </div>
    <div class="meter-value">{value}</div>
    <button class="mute" type="button">MUTE</button>
</aside>

<style>
    .meter-card {
        display: grid;
        grid-template-rows: auto minmax(20rem, 1fr) auto auto;
        gap: 0.6rem;
        width: 7.3rem;
        min-height: 31rem;
        padding: 1.35rem 1rem 1rem;
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
        font-size: 0.9rem;
        font-weight: 680;
        letter-spacing: 0.06em;
        text-align: center;
    }

    .meter-body {
        display: grid;
        grid-template-columns: 1fr auto;
        gap: 0.76rem;
        align-items: stretch;
        min-height: 0;
    }

    .bar-stack {
        display: flex;
        flex-direction: column-reverse;
        justify-content: flex-start;
        gap: 0.24rem;
        padding: 0.1rem 0;
    }

    .bar-stack span {
        display: block;
        width: 1.35rem;
        height: 0.18rem;
        border-radius: 999px;
        background: rgba(71, 88, 101, 0.32);
    }

    .bar-stack span.lit {
        background: linear-gradient(90deg, var(--color-accent-green), var(--color-meter-lime));
        box-shadow: 0 0 10px rgba(22, 215, 161, 0.16);
    }

    .bar-stack span.lit.warm {
        background: linear-gradient(90deg, var(--color-meter-lime), var(--color-meter-yellow));
        box-shadow: 0 0 10px rgba(246, 241, 70, 0.18);
    }

    .bar-stack span.lit.hot {
        background: var(--color-meter-yellow);
        box-shadow: 0 0 12px rgba(246, 241, 70, 0.24);
    }

    .scale {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        color: var(--color-text-muted);
        font-size: 0.78rem;
        line-height: 1;
        text-align: right;
    }

    .meter-value {
        color: var(--color-accent-green);
        font-size: 0.92rem;
        font-weight: 760;
        text-align: center;
    }

    .mute {
        justify-self: center;
        width: 4.8rem;
        min-height: 2rem;
        border: 1px solid var(--color-control-border);
        border-radius: var(--radius-control);
        background: rgba(20, 29, 39, 0.72);
        color: var(--color-text-soft);
        cursor: pointer;
        font-size: 0.78rem;
        font-weight: 720;
        letter-spacing: 0.08em;
    }

    .mute:hover {
        background: var(--color-control-hover);
        color: var(--color-text);
    }

    @media (max-width: 980px) {
        .meter-card {
            width: min(100%, 25rem);
            min-height: 10.8rem;
            grid-template-rows: auto 1fr auto;
            grid-template-columns: 1fr auto;
            align-items: center;
        }

        .meter-label {
            grid-column: 1 / -1;
        }

        .meter-body {
            min-height: 6.6rem;
        }

        .mute {
            grid-column: 2;
            grid-row: 2 / span 2;
        }
    }

    @media (max-width: 620px) {
        .meter-card {
            grid-template-columns: 1fr;
            width: 100%;
        }

        .mute {
            grid-column: auto;
            grid-row: auto;
        }
    }
</style>

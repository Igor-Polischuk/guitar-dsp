<script lang="ts">
    import { ChevronDown, Play } from "@lucide/svelte";
</script>

<section class="utility-dock" aria-label="Preset, tuner, metronome and output controls">
    <div class="preset tile">
        <span class="eyebrow">PRESET</span>
        <button type="button" class="preset-select">
            <span>Default Preset</span>
            <ChevronDown size={18} />
        </button>
    </div>

    <div class="tuner tile">
        <span class="eyebrow">TUNER</span>
        <strong>E</strong>
        <span>82.4 Hz</span>
    </div>

    <div class="metronome tile">
        <span class="eyebrow">METRONOME</span>
        <div class="metronome-row">
            <button class="play" type="button" aria-label="Start metronome" title="Start metronome">
                <Play size={22} fill="currentColor" />
            </button>
            <div>
                <strong>120</strong>
                <span>BPM</span>
            </div>
            <div>
                <strong>4/4</strong>
                <span>TIME</span>
            </div>
        </div>
    </div>

    <div class="mini-control tile">
        <span class="eyebrow">NOISE GATE</span>
        <div class="mini-row">
            <span class="mini-knob" aria-hidden="true"></span>
            <div>
                <strong>-60.0 dB</strong>
                <span>THRESH</span>
            </div>
        </div>
    </div>

    <div class="mini-control tile">
        <span class="eyebrow">OUTPUT LEVEL</span>
        <div class="mini-row">
            <span class="mini-knob low" aria-hidden="true"></span>
            <div>
                <strong>-1.2 dB</strong>
                <span>LEVEL</span>
            </div>
        </div>
    </div>
</section>

<style>
    .utility-dock {
        display: grid;
        grid-template-columns: minmax(12rem, 1.35fr) minmax(7rem, 0.7fr) minmax(12rem, 1.4fr) minmax(11rem, 1fr) minmax(11rem, 1fr);
        min-height: 8rem;
        border: var(--border-panel);
        border-radius: var(--radius-panel);
        overflow: hidden;
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.03), transparent),
            var(--color-panel-deep);
        box-shadow:
            inset 0 1px 0 rgba(255, 255, 255, 0.035),
            0 12px 30px rgba(0, 0, 0, 0.26);
    }

    .tile {
        display: grid;
        align-content: center;
        gap: 0.65rem;
        min-width: 0;
        padding: 1.1rem 1.55rem;
        border-left: 1px solid rgba(119, 145, 164, 0.12);
    }

    .tile:first-child {
        border-left: 0;
    }

    .eyebrow {
        color: var(--color-text-faint);
        font-size: 0.76rem;
        font-weight: 720;
        letter-spacing: 0.06em;
    }

    .preset-select {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.75rem;
        width: 100%;
        min-width: 0;
        background: transparent;
        color: var(--color-text);
        cursor: pointer;
        font-size: 0.92rem;
        font-weight: 560;
        line-height: 1.2;
        text-align: left;
    }

    .preset-select span {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .tuner {
        justify-items: center;
        text-align: center;
    }

    .tuner strong {
        color: var(--color-accent-green);
        font-size: 2rem;
        font-weight: 500;
        line-height: 1;
    }

    .tuner span:last-child,
    .metronome span,
    .mini-control span:not(.mini-knob) {
        color: var(--color-text-muted);
        font-size: 0.79rem;
        font-weight: 620;
        letter-spacing: 0.04em;
    }

    .metronome-row,
    .mini-row {
        display: flex;
        align-items: center;
        gap: 1.2rem;
        min-width: 0;
    }

    .metronome-row > div,
    .mini-row > div {
        display: grid;
        gap: 0.15rem;
        min-width: 0;
    }

    .metronome strong,
    .mini-control strong {
        color: var(--color-text-soft);
        font-size: 0.95rem;
        font-weight: 620;
        line-height: 1.2;
        white-space: nowrap;
    }

    .play {
        display: inline-grid;
        flex: 0 0 auto;
        width: 3.35rem;
        aspect-ratio: 1;
        place-items: center;
        border: 1px solid var(--color-control-border);
        border-radius: 50%;
        background: rgba(18, 27, 36, 0.72);
        color: var(--color-text-soft);
        cursor: pointer;
    }

    .play:hover {
        background: var(--color-control-hover);
        color: var(--color-text);
    }

    .mini-knob {
        flex: 0 0 auto;
        width: 3.5rem;
        aspect-ratio: 1;
        border-radius: 50%;
        background:
            radial-gradient(circle at 39% 34%, var(--color-accent-blue) 0 0.18rem, transparent 0.22rem),
            conic-gradient(from 220deg, var(--color-accent-blue) 0deg 78deg, rgba(95, 115, 132, 0.22) 78deg 280deg, transparent 280deg),
            linear-gradient(145deg, #303b49, #141c25 70%);
        box-shadow:
            inset 0 1px 4px rgba(255, 255, 255, 0.1),
            inset 0 -9px 15px rgba(0, 0, 0, 0.42),
            0 9px 16px rgba(0, 0, 0, 0.28);
    }

    .mini-knob.low {
        background:
            radial-gradient(circle at 32% 56%, var(--color-accent-blue) 0 0.18rem, transparent 0.22rem),
            conic-gradient(from 220deg, var(--color-accent-blue) 0deg 58deg, rgba(95, 115, 132, 0.22) 58deg 280deg, transparent 280deg),
            linear-gradient(145deg, #303b49, #141c25 70%);
    }

    @media (max-width: 1120px) {
        .utility-dock {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }

        .tile:nth-child(odd) {
            border-left: 0;
        }

        .tile:nth-child(n + 3) {
            border-top: 1px solid rgba(119, 145, 164, 0.12);
        }
    }

    @media (max-width: 640px) {
        .utility-dock {
            grid-template-columns: 1fr;
        }

        .tile {
            border-left: 0;
            border-top: 1px solid rgba(119, 145, 164, 0.12);
            padding: 1rem;
        }

        .tile:first-child {
            border-top: 0;
        }

        .metronome-row,
        .mini-row {
            gap: 0.9rem;
        }
    }
</style>

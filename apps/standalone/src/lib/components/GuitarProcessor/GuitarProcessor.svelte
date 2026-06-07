<script lang="ts">
    import AnalyzerDisplay from "$lib/components/AnalyzerDisplay/AnalyzerDisplay.svelte";
    import HeaderBar from "$lib/components/HeaderBar/HeaderBar.svelte";
    import KnobControl from "$lib/components/KnobControl/KnobControl.svelte";
    import LevelMeter from "$lib/components/LevelMeter/LevelMeter.svelte";
    import StatusBar from "$lib/components/StatusBar/StatusBar.svelte";
    import UtilityDock from "$lib/components/UtilityDock/UtilityDock.svelte";
    import { invoke } from "@tauri-apps/api/core";

    invoke("start_audio");

    type ControlConfig = {
        label: string;
        value: number;
        min: number;
        max: number;
        step: number;
        accent: "green" | "cyan" | "blue" | "purple";
        formatter: (value: number) => string;
    };

    function formatDb(value: number) {
        return `${value.toFixed(1)} dB`;
    }

    function formatSignedDb(value: number) {
        const rounded = Number(value.toFixed(1));
        return `${rounded > 0 ? "+" : ""}${rounded.toFixed(1)} dB`;
    }

    function formatHz(value: number) {
        return `${Math.round(value)} Hz`;
    }

    function formatKhz(value: number) {
        return `${value.toFixed(1)} kHz`;
    }

    const controls = [
        {
            label: "GAIN",
            value: 5.2,
            min: 1,
            max: 250,
            step: 20,
            accent: "green",
            formatter: formatDb,
        },
        {
            label: "BASS",
            value: 2.1,
            min: -12,
            max: 12,
            step: 0.1,
            accent: "green",
            formatter: formatSignedDb,
        },
        {
            label: "MID",
            value: -0.4,
            min: -12,
            max: 12,
            step: 0.1,
            accent: "green",
            formatter: formatSignedDb,
        },
        {
            label: "TREBLE",
            value: 3.3,
            min: -12,
            max: 12,
            step: 0.1,
            accent: "green",
            formatter: formatSignedDb,
        },
        {
            label: "HPF",
            value: 80,
            min: 20,
            max: 500,
            step: 5,
            accent: "blue",
            formatter: formatHz,
        },
        {
            label: "LPF",
            value: 8,
            min: 1,
            max: 16,
            step: 0.1,
            accent: "blue",
            formatter: formatKhz,
        },
        {
            label: "MASTER",
            value: -1.2,
            min: -24,
            max: 6,
            step: 0.1,
            accent: "green",
            formatter: formatDb,
        },
    ] satisfies ControlConfig[];
</script>

<main class="screen" aria-label="Guitar processor mock interface">
    <div class="processor-frame">
        <HeaderBar />

        <section class="workspace" aria-label="Processor workspace">
            <LevelMeter label="INPUT" value="-8.7 dB" activeBars={1000} />

            <section class="center-panel">
                <div class="knob-row" aria-label="Tone and filter controls">
                    {#each controls as control}
                        <KnobControl {...control} />
                    {/each}
                </div>

                <AnalyzerDisplay />
                <UtilityDock />
            </section>

            <LevelMeter label="OUTPUT" value="-7.3 dB" activeBars={26} />
        </section>

        <StatusBar />
    </div>
</main>

<style>
    .screen {
        display: grid;
        width: 100%;
        min-height: 100vh;
        place-items: stretch;
        background:
            radial-gradient(
                circle at 17% 0,
                rgba(22, 215, 161, 0.06),
                transparent 26rem
            ),
            radial-gradient(
                circle at 83% 8%,
                rgba(168, 92, 244, 0.06),
                transparent 28rem
            );
    }

    .processor-frame {
        display: grid;
        grid-template-rows: auto minmax(0, 1fr) auto;
        width: 100%;
        min-height: 100vh;
        overflow: hidden;
        border-block: 1px solid rgba(119, 145, 164, 0.14);
        background:
            linear-gradient(
                180deg,
                rgba(255, 255, 255, 0.04),
                transparent 11rem
            ),
            var(--color-panel);
    }

    .workspace {
        display: grid;
        grid-template-columns: auto minmax(0, 1fr) auto;
        gap: clamp(1.35rem, 1.9vw, 2rem);
        min-width: 0;
        padding: clamp(1.4rem, 2.1vw, 2rem) clamp(1.35rem, 1.65vw, 1.6rem)
            0.85rem;
    }

    .center-panel {
        display: grid;
        grid-template-rows: auto minmax(18rem, 1fr) auto;
        gap: 0.95rem;
        min-width: 0;
    }

    .knob-row {
        display: grid;
        grid-template-columns: repeat(7, minmax(5.7rem, 1fr));
        align-items: start;
        gap: clamp(0.7rem, 1.75vw, 2.25rem);
        min-width: 0;
        padding: 0.05rem clamp(0.25rem, 0.7vw, 0.75rem) 0.7rem;
    }

    @media (max-width: 1160px) {
        .knob-row {
            grid-template-columns: repeat(4, minmax(5.7rem, 1fr));
            row-gap: 1.1rem;
        }
    }

    @media (max-width: 980px) {
        .workspace {
            grid-template-columns: 1fr 1fr;
            align-items: stretch;
        }

        .center-panel {
            grid-column: 1 / -1;
            grid-row: 1;
        }
    }

    @media (max-width: 680px) {
        .workspace {
            grid-template-columns: 1fr;
            padding: 1rem;
        }

        .center-panel {
            grid-row: auto;
        }

        .knob-row {
            grid-template-columns: repeat(2, minmax(0, 1fr));
            gap: 1rem 0.7rem;
        }
    }
</style>

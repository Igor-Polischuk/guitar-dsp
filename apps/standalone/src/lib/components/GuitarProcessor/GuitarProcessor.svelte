<script lang="ts">
    import AmpPanel from "$lib/components/AmpPanel/AmpPanel.svelte";
    import AnalyzerDisplay from "$lib/components/AnalyzerDisplay/AnalyzerDisplay.svelte";
    import AudioSetupDialog from "$lib/components/AudioSetupDialog/AudioSetupDialog.svelte";
    import HeaderBar from "$lib/components/HeaderBar/HeaderBar.svelte";
    import LevelMeter from "$lib/components/LevelMeter/LevelMeter.svelte";
    import SignalChain from "$lib/components/SignalChain/SignalChain.svelte";
    import StatusBar from "$lib/components/StatusBar/StatusBar.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onDestroy, onMount } from "svelte";

    type PeakMeters = {
        input_db: number;
        output_db: number;
    };

    let audioSetupOpen = false;
    let inputDb = -60;
    let outputDb = -60;
    let meterFrame = 0;
    let lastMeterUpdate = 0;
    let metersRequestInFlight = false;

    const meterMin = -60;
    const meterMax = 0;
    const meterUpdateIntervalMs = 33;

    onMount(async () => {
        if ("__TAURI_INTERNALS__" in window) {
            try {
                const isSelectedDevice = await invoke<boolean>(
                    "is_devices_selected",
                );
                audioSetupOpen = !isSelectedDevice;
            } catch {
                audioSetupOpen = true;
            }

            meterFrame = requestAnimationFrame(updateMeters);
        }
    });

    onDestroy(() => {
        if (meterFrame) {
            cancelAnimationFrame(meterFrame);
        }
    });

    async function updateMeters(timestamp: number) {
        meterFrame = requestAnimationFrame(updateMeters);

        if (
            metersRequestInFlight ||
            timestamp - lastMeterUpdate < meterUpdateIntervalMs
        ) {
            return;
        }

        lastMeterUpdate = timestamp;
        metersRequestInFlight = true;

        try {
            const meters = await invoke<PeakMeters>("get_peak_dbs");
            inputDb = Math.max(meterMin, meters.input_db);
            outputDb = Math.max(meterMin, meters.output_db);
        } catch {
            // Meters can be unavailable before the engine is fully started.
        } finally {
            metersRequestInFlight = false;
        }
    }
</script>

<main class="screen" aria-label="Guitar processor mock interface">
    <div class="processor-frame">
        <HeaderBar />
        <AmpPanel />

        <section class="workspace" aria-label="Spectrum workspace">
            <LevelMeter
                label="INPUT"
                value={inputDb}
                min={meterMin}
                max={meterMax}
                clip={inputDb >= meterMax}
            />
            <AnalyzerDisplay />
            <LevelMeter
                label="OUTPUT"
                value={outputDb}
                min={meterMin}
                max={meterMax}
                clip={outputDb >= meterMax}
            />
        </section>

        <SignalChain />
        <StatusBar />
    </div>

    <AudioSetupDialog
        open={audioSetupOpen}
        onClose={() => (audioSetupOpen = false)}
    />
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
        grid-template-rows: auto auto minmax(22rem, 1fr) auto auto;
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
        gap: 0.85rem;
        min-width: 0;
        padding: 0.75rem 1.25rem 0.75rem;
    }

    @media (max-width: 980px) {
        .workspace {
            grid-template-columns: 1fr 1fr;
            align-items: stretch;
        }
    }

    @media (max-width: 680px) {
        .workspace {
            grid-template-columns: 1fr;
            padding: 1rem;
        }
    }
</style>

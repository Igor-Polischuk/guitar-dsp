<script lang="ts">
    import AmpPanel from "$lib/components/AmpPanel/AmpPanel.svelte";
    import AnalyzerDisplay from "$lib/components/AnalyzerDisplay/AnalyzerDisplay.svelte";
    import HeaderBar from "$lib/components/HeaderBar/HeaderBar.svelte";
    import LevelMeter from "$lib/components/LevelMeter/LevelMeter.svelte";
    import SignalChain from "$lib/components/SignalChain/SignalChain.svelte";
    import StatusBar from "$lib/components/StatusBar/StatusBar.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    onMount(() => {
        if ("__TAURI_INTERNALS__" in window) {
            invoke("start_audio");
        }
    });
</script>

<main class="screen" aria-label="Guitar processor mock interface">
    <div class="processor-frame">
        <HeaderBar />
        <AmpPanel />

        <section class="workspace" aria-label="Spectrum workspace">
            <LevelMeter label="INPUT" value="-8.7 dB" activeBars={27} secondaryBars={22} />
            <AnalyzerDisplay />
            <LevelMeter label="OUTPUT" value="-7.3 dB" activeBars={25} secondaryBars={21} />
        </section>

        <SignalChain />
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

<script lang="ts">
    import KnobControl from "$lib/components/KnobControl/KnobControl.svelte";
    import { ChevronDown, EllipsisVertical } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { on } from "svelte/events";

    type ControlConfig = {
        id: string;
        label: string;
        value: number;
        min: number;
        max: number;
        step: number;
        accent: "green" | "cyan" | "blue" | "purple";
        formatter: (value: number) => string;
    };

    type AmpInputResponse = {
        id: string;
        label: string;
    };

    type AmpKnobsResponse = {
        id: string;
        label: string;
        max: number;
        min: number;
        step: number;
        default: number;
        unit: null | null;
    };

    function formatAmpValue(value: number) {
        return value.toFixed(1);
    }

    invoke("get_current_amplifier_knobs").then((response) => {
        console.log(response);
    });

    // const controls = [
    //     {
    //         label: "GAIN",
    //         value: 6.4,
    //         min: 0,
    //         max: 10,
    //         step: 0.1,
    //         accent: "blue",
    //         formatter: formatAmpValue,
    //     },
    //     {
    //         label: "BASS",
    //         value: 5.2,
    //         min: 0,
    //         max: 10,
    //         step: 0.1,
    //         accent: "blue",
    //         formatter: formatAmpValue,
    //     },
    //     {
    //         label: "MIDDLE",
    //         value: 7.1,
    //         min: 0,
    //         max: 10,
    //         step: 0.1,
    //         accent: "blue",
    //         formatter: formatAmpValue,
    //     },
    //     {
    //         label: "TREBLE",
    //         value: 6.3,
    //         min: 0,
    //         max: 10,
    //         step: 0.1,
    //         accent: "blue",
    //         formatter: formatAmpValue,
    //     },
    //     {
    //         label: "PRESENCE",
    //         value: 5.0,
    //         min: 0,
    //         max: 10,
    //         step: 0.1,
    //         accent: "blue",
    //         formatter: formatAmpValue,
    //     },
    //     {
    //         label: "MASTER",
    //         value: 6.8,
    //         min: 0,
    //         max: 10,
    //         step: 0.1,
    //         accent: "blue",
    //         formatter: formatAmpValue,
    //     },
    // ] satisfies ControlConfig[];

    let controls: ControlConfig[] | null = null;
    let inputs: AmpInputResponse[] | null = null;
    let activeInputId: string | null = null;

    async function fetchControls() {
        const knobs = await invoke<AmpKnobsResponse[]>(
            "get_current_amplifier_knobs",
        );
        inputs = await invoke<AmpInputResponse[]>(
            "get_current_amplifier_inputs",
        );
        activeInputId = await invoke<string>(
            "get_current_amplifier_active_input",
        );

        controls = knobs.map((knob, index) => {
            return {
                id: knob.id,
                label: knob.label,
                value: knob.default,
                min: knob.min,
                max: knob.max,
                step: knob.step,
                accent: ["green", "cyan", "blue", "purple"][
                    index % 4
                ] as ControlConfig["accent"],
                formatter: formatAmpValue,
            };
        });
    }

    async function setActiveInput(inputId: string) {
        await invoke("set_active_amp_input", { inputId });
        activeInputId = inputId;
    }

    onMount(() => {
        fetchControls();
    });
</script>

<section class="amp-panel" aria-label="Amplifier model controls">
    <div class="amp-preview" aria-hidden="true">
        <div class="amp-head">
            <span class="brand-script">Marshall</span>
            <div class="faceplate">
                {#each Array.from({ length: 7 }) as _}
                    <i></i>
                {/each}
                <b></b>
                <b></b>
            </div>
        </div>
    </div>

    <div class="amp-model">
        <span>AMPLIFIER</span>
        <button type="button" class="model-select">
            British 800
            <ChevronDown size={17} />
        </button>
        <button type="button" class="change-model">CHANGE MODEL</button>
    </div>

    <div class="amp-controls" aria-label="Amplifier tone controls">
        {#each controls as control}
            <KnobControl {...control} />
        {/each}
    </div>

    <div class="channel">
        <span>CHANNEL</span>
        {#each inputs as input}
            <button
                type="button"
                class="channel-button {input.id === activeInputId
                    ? 'active'
                    : ''}"
                on:click={() => setActiveInput(input.id)}
            >
                {input.label.toUpperCase()}
            </button>
        {/each}
    </div>

    <button
        class="menu"
        type="button"
        aria-label="Amplifier options"
        title="Amplifier options"
    >
        <EllipsisVertical size={19} />
    </button>
</section>

<style>
    .amp-panel {
        display: grid;
        grid-template-columns:
            minmax(13rem, 17.5rem) minmax(12rem, 1fr) minmax(30rem, 3.6fr)
            minmax(7.5rem, 0.7fr) auto;
        align-items: center;
        gap: 1.35rem;
        min-height: 9.5rem;
        padding: 1.1rem 1rem 1rem 1.45rem;
        border-bottom: var(--border-panel);
        background:
            radial-gradient(
                circle at 20% 0,
                rgba(47, 134, 255, 0.08),
                transparent 26rem
            ),
            linear-gradient(180deg, rgba(255, 255, 255, 0.025), transparent),
            var(--color-panel);
    }

    .amp-preview {
        min-width: 0;
    }

    .amp-head {
        position: relative;
        width: 100%;
        max-width: 15.2rem;
        aspect-ratio: 3.2 / 1.18;
        overflow: hidden;
        border: 1px solid rgba(179, 147, 82, 0.56);
        border-radius: 0.32rem;
        background:
            linear-gradient(135deg, rgba(255, 255, 255, 0.11), transparent 23%),
            repeating-linear-gradient(
                0deg,
                rgba(255, 255, 255, 0.035) 0 1px,
                transparent 1px 5px
            ),
            #161a1b;
        box-shadow:
            inset 0 0 0 3px rgba(161, 119, 48, 0.22),
            0 12px 22px rgba(0, 0, 0, 0.4);
    }

    .amp-head::before {
        content: "";
        position: absolute;
        inset: 0.58rem 0.9rem auto;
        height: 0.16rem;
        border-radius: 999px;
        background: rgba(186, 151, 82, 0.38);
    }

    .brand-script {
        position: absolute;
        top: 29%;
        left: 50%;
        color: #ece6cf;
        font-family: Georgia, serif;
        font-size: 0.92rem;
        font-style: italic;
        font-weight: 700;
        transform: translateX(-50%);
    }

    .faceplate {
        position: absolute;
        right: 1.1rem;
        bottom: 0.9rem;
        left: 4.35rem;
        display: flex;
        align-items: center;
        gap: 0.42rem;
        min-height: 1.15rem;
        padding: 0 0.55rem;
        border-radius: 0.15rem;
        background: linear-gradient(180deg, #d7b96f, #9d7c35);
        box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.28);
    }

    .faceplate i,
    .faceplate b {
        width: 0.31rem;
        height: 0.31rem;
        border-radius: 50%;
        background: #20242a;
        box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.28);
    }

    .faceplate b {
        margin-left: 0.05rem;
        background: #0d1115;
    }

    .amp-model {
        display: grid;
        gap: 0.55rem;
        min-width: 0;
        padding-right: 1.15rem;
        border-right: 1px solid var(--color-panel-line);
    }

    .amp-model span,
    .channel span {
        color: var(--color-accent-blue-strong);
        font-size: 0.76rem;
        font-weight: 730;
        letter-spacing: 0.03em;
    }

    .model-select {
        display: inline-flex;
        align-items: center;
        gap: 0.45rem;
        min-width: 0;
        width: fit-content;
        max-width: 100%;
        padding: 0;
        background: transparent;
        color: var(--color-text);
        cursor: pointer;
        font-size: 1.08rem;
        font-weight: 560;
        text-align: left;
    }

    .change-model {
        width: fit-content;
        min-height: 2.15rem;
        padding: 0 1.15rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(17, 25, 34, 0.72);
        color: var(--color-text-soft);
        cursor: pointer;
        font-size: 0.73rem;
        font-weight: 720;
        letter-spacing: 0.02em;
    }

    .amp-controls {
        display: grid;
        grid-template-columns: repeat(6, minmax(4.8rem, 1fr));
        align-items: start;
        gap: clamp(0.6rem, 1.15vw, 1.35rem);
        min-width: 0;
        --knob-size: clamp(4.15rem, 4.8vw, 4.85rem);
    }

    .channel {
        display: grid;
        gap: 0.55rem;
        min-width: 0;
        align-self: center;
    }

    .channel span {
        color: var(--color-text-soft);
        text-align: center;
    }

    .channel-button {
        min-height: 2rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(7, 13, 20, 0.46);
        color: var(--color-text-soft);
        cursor: pointer;
        font-size: 0.76rem;
        font-weight: 730;
    }

    .channel-button.active {
        border-color: rgba(47, 134, 255, 0.46);
        background: rgba(47, 134, 255, 0.17);
        color: #61a9ff;
        box-shadow: inset 0 0 0 1px rgba(47, 134, 255, 0.12);
    }

    .menu {
        align-self: start;
        display: grid;
        width: 2.35rem;
        aspect-ratio: 1;
        place-items: center;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(17, 25, 34, 0.76);
        color: var(--color-text-muted);
        cursor: pointer;
    }

    @media (max-width: 1220px) {
        .amp-panel {
            grid-template-columns: minmax(12rem, 17rem) minmax(12rem, 1fr) auto;
            grid-template-areas:
                "preview model menu"
                "controls controls channel";
        }

        .amp-preview {
            grid-area: preview;
        }

        .amp-model {
            grid-area: model;
            border-right: 0;
        }

        .amp-controls {
            grid-area: controls;
            grid-template-columns: repeat(3, minmax(4.8rem, 1fr));
        }

        .channel {
            grid-area: channel;
        }

        .menu {
            grid-area: menu;
        }
    }

    @media (max-width: 700px) {
        .amp-panel {
            grid-template-columns: 1fr auto;
            grid-template-areas:
                "preview menu"
                "model model"
                "controls controls"
                "channel channel";
            padding: 1rem;
        }

        .amp-controls {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }

        .channel {
            grid-template-columns: 1fr 1fr;
        }

        .channel span {
            grid-column: 1 / -1;
            text-align: left;
        }
    }
</style>

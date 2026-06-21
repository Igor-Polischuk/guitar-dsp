<script lang="ts">
    import { Link, Volume2, Waves } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Modal from "$lib/components/Modal/Modal.svelte";
    import UiButton from "$lib/components/UiButton/UiButton.svelte";
    import UiSelect from "$lib/components/UiSelect/UiSelect.svelte";

    type AudioDevice = {
        name: string;
        is_input: boolean;
        is_output: boolean;
    };

    export let open = false;
    export let onClose: (() => void) | undefined = undefined;

    let devices: AudioDevice[] = [];
    let loading = false;
    let submitting = false;
    let errorMessage = "";
    let selectedInput = "";
    let selectedOutput = "";
    let devicesRefreshing = false;
    let deviceRefreshTimer: ReturnType<typeof setInterval> | undefined;

    $: inputOptions = devices
        .filter((device) => device.is_input)
        .map((device) => ({ value: device.name, label: device.name }));
    $: outputOptions = devices
        .filter((device) => device.is_output)
        .map((device) => ({ value: device.name, label: device.name }));
    $: if (
        inputOptions.length > 0 &&
        !inputOptions.some((option) => option.value === selectedInput)
    ) {
        selectedInput = inputOptions[0].value;
    }
    $: if (inputOptions.length === 0 && selectedInput) {
        selectedInput = "";
    }
    $: if (
        outputOptions.length > 0 &&
        !outputOptions.some((option) => option.value === selectedOutput)
    ) {
        selectedOutput = outputOptions[0].value;
    }
    $: if (outputOptions.length === 0 && selectedOutput) {
        selectedOutput = "";
    }
    $: canContinue = Boolean(
        selectedInput && selectedOutput && !loading && !submitting,
    );

    const inputChannels = [{ value: "Input 1", label: "Input 1" }];
    const outputChannels = [
        { value: "Output 1 - 2 (Stereo)", label: "Output 1 - 2 (Stereo)" },
    ];
    const sampleRates = [{ value: "48.0 kHz", label: "48.0 kHz" }];
    const bufferSizes = [
        { value: "128 samples (2.7 ms)", label: "128 samples (2.7 ms)" },
    ];

    $: if (open) {
        void loadDevices();
        startDeviceRefresh();
    }

    $: if (!open) {
        stopDeviceRefresh();
    }

    function startDeviceRefresh() {
        if (
            deviceRefreshTimer ||
            typeof window === "undefined" ||
            !("__TAURI_INTERNALS__" in window)
        ) {
            return;
        }

        deviceRefreshTimer = setInterval(() => {
            void refreshDevices();
        }, 2500);
    }

    function stopDeviceRefresh() {
        if (!deviceRefreshTimer) {
            return;
        }

        clearInterval(deviceRefreshTimer);
        deviceRefreshTimer = undefined;
    }

    async function loadDevices() {
        if (
            devices.length > 0 ||
            loading ||
            devicesRefreshing ||
            typeof window === "undefined" ||
            !("__TAURI_INTERNALS__" in window)
        ) {
            return;
        }

        loading = true;
        errorMessage = "";

        try {
            devices = await invoke<AudioDevice[]>("list_audio_devices");
        } catch (error) {
            errorMessage = getErrorMessage(error);
        } finally {
            loading = false;
        }
    }

    async function refreshDevices() {
        if (
            loading ||
            submitting ||
            devicesRefreshing ||
            typeof window === "undefined" ||
            !("__TAURI_INTERNALS__" in window)
        ) {
            return;
        }

        devicesRefreshing = true;

        try {
            devices = await invoke<AudioDevice[]>("list_audio_devices");
        } catch (error) {
            errorMessage = getErrorMessage(error);
        } finally {
            devicesRefreshing = false;
        }
    }

    async function handleConfirm() {
        if (!canContinue) {
            return;
        }

        submitting = true;
        errorMessage = "";

        try {
            await invoke("set_devices", {
                input: selectedInput,
                output: selectedOutput,
            });
            await invoke("start_audio");
            onClose?.();
        } catch (error) {
            errorMessage = getErrorMessage(error);
        } finally {
            submitting = false;
        }
    }

    function getErrorMessage(error: unknown) {
        if (typeof error === "string") {
            return error;
        }

        if (error instanceof Error) {
            return error.message;
        }

        return "Unable to configure audio devices.";
    }
</script>

<Modal
    {open}
    title="AUDIO SETUP"
    subtitle="Select your audio devices to get started"
    closeDisabled={submitting}
    {onClose}
>
    <div class="setup-grid">
        <section class="device-column">
            <div class="section-title input">
                <Waves size={18} />
                <span>INPUT DEVICE</span>
            </div>
            <UiSelect
                id="audio-input-device"
                label=""
                description="Select the audio device for input"
                value={selectedInput}
                options={inputOptions}
                disabled={loading || submitting || inputOptions.length === 0}
                onChange={(value) => (selectedInput = value)}
                onOpen={refreshDevices}
            />

            <UiSelect
                id="audio-input-channel"
                label="Input Channels"
                value={inputChannels[0].value}
                options={inputChannels}
                disabled
            />

            <div class="level-row">
                <span>Input Level</span>
                <div class="meter" aria-hidden="true"><i></i></div>
                <strong>-12.4 dB</strong>
            </div>

            <div class="segmented" aria-label="Input mode">
                <button class="active" type="button" disabled
                    >Hi-Z (Instrument)</button
                >
                <button type="button" disabled>Line Level</button>
            </div>

            <label class="checkbox">
                <input type="checkbox" disabled />
                <span>Enable Noise Gate on Input</span>
            </label>
        </section>

        <div class="link-column" aria-hidden="true">
            <span><Link size={18} /></span>
        </div>

        <section class="device-column">
            <div class="section-title output">
                <Volume2 size={18} />
                <span>OUTPUT DEVICE</span>
            </div>
            <UiSelect
                id="audio-output-device"
                label=""
                description="Select the audio device for output"
                value={selectedOutput}
                options={outputOptions}
                disabled={loading || submitting || outputOptions.length === 0}
                onChange={(value) => (selectedOutput = value)}
                onOpen={refreshDevices}
            />

            <UiSelect
                id="audio-output-channel"
                label="Output Channels"
                value={outputChannels[0].value}
                options={outputChannels}
                disabled
            />

            <div class="level-row">
                <span>Output Level</span>
                <div class="meter" aria-hidden="true"><i></i></div>
                <strong>-10.6 dB</strong>
            </div>

            <div class="segmented" aria-label="Output mode">
                <button class="active" type="button" disabled
                    >Studio (Flat)</button
                >
                <button type="button" disabled>Live (Enhanced)</button>
            </div>

            <label class="checkbox">
                <input type="checkbox" disabled />
                <span>Enable Cabinet Simulation for Headphones</span>
            </label>
        </section>
    </div>

    <div class="footer-settings">
        <UiSelect
            id="sample-rate"
            label="Sample Rate"
            value={sampleRates[0].value}
            options={sampleRates}
            disabled
        />
        <UiSelect
            id="buffer-size"
            label="Buffer Size"
            value={bufferSizes[0].value}
            options={bufferSizes}
            disabled
        />
    </div>

    <div class="latency">Detected Latency: 2.7 ms</div>

    {#if loading}
        <p class="message">Loading audio devices...</p>
    {:else if inputOptions.length === 0 || outputOptions.length === 0}
        <p class="message error">
            No compatible input/output device pair was found.
        </p>
    {:else if errorMessage}
        <p class="message error">{errorMessage}</p>
    {/if}

    <div class="actions">
        <UiButton disabled={!canContinue} onClick={handleConfirm}>
            {submitting ? "CONNECTING..." : "CONTINUE"}
        </UiButton>
    </div>
</Modal>

<style>
    .setup-grid {
        display: grid;
        grid-template-columns: minmax(0, 1fr) 2.7rem minmax(0, 1fr);
        gap: 1.25rem;
    }

    .device-column {
        display: grid;
        align-content: start;
        gap: 0.9rem;
        min-width: 0;
    }

    .section-title {
        display: inline-flex;
        align-items: center;
        gap: 0.55rem;
        color: var(--color-text-soft);
        font-size: 0.78rem;
        font-weight: 780;
        letter-spacing: 0.03em;
    }

    .section-title.input {
        color: #6db4ff;
    }

    .section-title.output {
        color: #8fc5ff;
    }

    .link-column {
        display: grid;
        place-items: center;
        padding-top: 8.9rem;
    }

    .link-column span {
        display: grid;
        width: 2.3rem;
        aspect-ratio: 1;
        place-items: center;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(17, 25, 34, 0.72);
        color: var(--color-text-muted);
    }

    .level-row {
        display: grid;
        grid-template-columns: auto minmax(0, 1fr) auto;
        align-items: center;
        gap: 0.75rem;
        color: var(--color-text-muted);
        font-size: 0.72rem;
        font-weight: 620;
    }

    .level-row span {
        grid-column: 1 / -1;
    }

    .level-row strong {
        color: var(--color-text-soft);
        font-size: 0.72rem;
        font-weight: 650;
    }

    .meter {
        height: 0.32rem;
        overflow: hidden;
        border-radius: 999px;
        background: repeating-linear-gradient(
            90deg,
            rgba(100, 117, 130, 0.28) 0 0.12rem,
            transparent 0.12rem 0.22rem
        );
    }

    .meter i {
        display: block;
        width: 58%;
        height: 100%;
        border-radius: inherit;
        background: repeating-linear-gradient(
            90deg,
            var(--color-accent-green) 0 0.12rem,
            transparent 0.12rem 0.22rem
        );
        box-shadow: 0 0 9px rgba(22, 215, 161, 0.24);
    }

    .segmented {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 0.15rem;
        overflow: hidden;
        padding: 0.15rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(12, 18, 26, 0.55);
    }

    .segmented button {
        min-height: 2rem;
        border: 1px solid transparent;
        border-radius: calc(var(--radius-control) - 2px);
        background: transparent;
        color: var(--color-text-muted);
        font-size: 0.72rem;
        font-weight: 660;
    }

    .segmented button.active {
        border-color: rgba(47, 134, 255, 0.52);
        color: #6db4ff;
        background: rgba(47, 134, 255, 0.16);
        box-shadow: inset 0 0 0 1px rgba(47, 134, 255, 0.12);
    }

    .checkbox {
        display: inline-flex;
        align-items: center;
        gap: 0.55rem;
        color: var(--color-text-muted);
        font-size: 0.72rem;
        font-weight: 610;
    }

    .checkbox input {
        width: 0.95rem;
        aspect-ratio: 1;
        accent-color: var(--color-accent-blue-strong);
    }

    .footer-settings {
        display: grid;
        grid-template-columns: minmax(9rem, 12rem) minmax(11rem, 14rem);
        justify-content: center;
        gap: 1.2rem;
        margin-top: 1.6rem;
        padding-top: 1.1rem;
        border-top: 1px solid var(--color-panel-line);
    }

    .latency,
    .message {
        margin: 0.85rem 0 0;
        color: var(--color-text-muted);
        font-size: 0.74rem;
        font-weight: 620;
        text-align: center;
    }

    .message.error {
        color: var(--color-accent-red);
    }

    .actions {
        width: min(16.5rem, 100%);
        margin: 1.2rem auto 0;
    }

    @media (max-width: 760px) {
        .setup-grid {
            grid-template-columns: 1fr;
        }

        .link-column {
            display: none;
        }

        .footer-settings {
            grid-template-columns: 1fr;
        }
    }
</style>

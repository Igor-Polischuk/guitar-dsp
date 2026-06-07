<script lang="ts">
    import {
        Activity,
        ChevronDown,
        ChevronLeft,
        ChevronRight,
        Circle,
        Cpu,
        Redo2,
        Save,
        Settings,
        Undo2
    } from "@lucide/svelte";
</script>

<header class="header-bar">
    <div class="brand" aria-label="GTR Processor">
        <Activity size={31} strokeWidth={1.7} />
        <span>GTR PROCESSOR</span>
    </div>

    <div class="transport" aria-label="Preset navigation">
        <button class="icon-button" type="button" aria-label="Previous preset" title="Previous preset">
            <ChevronLeft size={21} />
        </button>
        <button class="icon-button" type="button" aria-label="Next preset" title="Next preset">
            <ChevronRight size={21} />
        </button>
    </div>

    <button class="preset-button" type="button" aria-label="Selected preset">
        <span class="bank">01A</span>
        <span>Plexi Lead</span>
        <ChevronDown size={18} />
    </button>

    <div class="actions" aria-label="Processor actions">
        <button class="icon-button" type="button" aria-label="Save preset" title="Save preset">
            <Save size={18} />
        </button>
        <span class="split-actions">
            <button class="icon-button" type="button" aria-label="Undo" title="Undo">
                <Undo2 size={18} />
            </button>
            <button class="icon-button muted" type="button" aria-label="Redo" title="Redo">
                <Redo2 size={18} />
            </button>
        </span>
        <button class="icon-button" type="button" aria-label="Settings" title="Settings">
            <Settings size={18} />
        </button>
        <button class="bypass" type="button" aria-pressed="false">BYPASS</button>
    </div>

    <div class="cpu" aria-label="CPU usage">
        <Cpu size={15} />
        <span>CPU 34%</span>
        <i aria-hidden="true"><b></b></i>
    </div>

    <div class="device" aria-label="Audio device">
        <Circle size={10} fill="currentColor" strokeWidth={0} />
        <div>
            <strong>Focusrite Scarlett 2i2</strong>
            <span>48.0 kHz&nbsp;&nbsp;&nbsp;128 samples</span>
        </div>
    </div>
</header>

<style>
    .header-bar {
        display: grid;
        grid-template-columns: minmax(14rem, 1.1fr) auto minmax(16rem, 1.45fr) auto minmax(10rem, 0.58fr) minmax(15rem, 0.95fr);
        align-items: center;
        gap: 0.9rem;
        min-height: 5rem;
        padding: 0.6rem 0.75rem 0.6rem 1.35rem;
        border-bottom: var(--border-panel);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.045), transparent),
            #0d1219;
        box-shadow:
            var(--shadow-inset),
            inset 0 -1px 0 rgba(0, 0, 0, 0.42);
    }

    .brand {
        display: flex;
        align-items: center;
        gap: 0.9rem;
        min-width: 0;
        color: var(--color-text);
    }

    .brand :global(svg) {
        color: var(--color-text);
        filter: drop-shadow(0 0 9px rgba(255, 255, 255, 0.08));
        flex: 0 0 auto;
    }

    .brand span {
        overflow: hidden;
        color: var(--color-text);
        font-size: 1.15rem;
        font-weight: 720;
        letter-spacing: 0.03em;
        line-height: 1.1;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .transport,
    .split-actions {
        display: grid;
        grid-auto-flow: column;
        overflow: hidden;
        border-radius: var(--radius-control);
        border: 1px solid var(--color-panel-line);
        background: rgba(15, 22, 31, 0.8);
    }

    .preset-button,
    .icon-button,
    .bypass {
        min-height: 2.6rem;
        background: rgba(17, 25, 34, 0.82);
        color: var(--color-text-soft);
        cursor: pointer;
        transition:
            background 140ms ease,
            color 140ms ease,
            border-color 140ms ease;
    }

    .preset-button:hover,
    .icon-button:hover,
    .bypass:hover {
        background: var(--color-control-hover);
        color: var(--color-text);
    }

    .icon-button {
        display: inline-grid;
        place-items: center;
        width: 3.25rem;
        border-radius: var(--radius-control);
        border: 1px solid var(--color-panel-line);
    }

    .transport .icon-button,
    .split-actions .icon-button {
        border: 0;
        border-radius: 0;
        background: transparent;
    }

    .transport .icon-button + .icon-button,
    .split-actions .icon-button + .icon-button {
        border-left: 1px solid var(--color-panel-line);
    }

    .icon-button.muted {
        color: var(--color-text-faint);
    }

    .preset-button {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.8rem;
        min-width: 0;
        padding: 0 1rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        font-size: 0.95rem;
        font-weight: 640;
    }

    .preset-button span {
        overflow: hidden;
        min-width: 0;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .preset-button .bank {
        flex: 0 0 auto;
        color: var(--color-text);
        font-size: 1rem;
        font-weight: 720;
    }

    .actions {
        display: flex;
        gap: 0.65rem;
        min-width: 0;
        padding-left: 0.75rem;
        border-left: 1px solid rgba(122, 145, 166, 0.1);
    }

    .bypass {
        width: 7.1rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        font-size: 0.78rem;
        font-weight: 720;
        letter-spacing: 0.04em;
    }

    .cpu {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 0.55rem;
        min-width: 0;
        color: var(--color-text-muted);
        font-size: 0.76rem;
        font-weight: 680;
        white-space: nowrap;
    }

    .cpu i {
        width: 4.4rem;
        height: 0.28rem;
        overflow: hidden;
        border-radius: 999px;
        background: rgba(95, 115, 132, 0.16);
    }

    .cpu b {
        display: block;
        width: 34%;
        height: 100%;
        border-radius: inherit;
        background: var(--color-accent-green);
        box-shadow: 0 0 9px rgba(22, 215, 161, 0.34);
    }

    .device {
        display: flex;
        align-items: flex-start;
        gap: 0.65rem;
        min-width: 0;
        padding: 0.6rem 0.75rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(18, 27, 36, 0.74);
        color: var(--color-accent-green);
    }

    .device div {
        display: grid;
        gap: 0.15rem;
        min-width: 0;
    }

    .device strong,
    .device span {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .device strong {
        color: var(--color-text-soft);
        font-size: 0.76rem;
        font-weight: 720;
    }

    .device span {
        color: var(--color-text-muted);
        font-size: 0.72rem;
        font-weight: 590;
    }

    @media (max-width: 1180px) {
        .header-bar {
            grid-template-columns: minmax(0, 1fr) auto minmax(16rem, 1.2fr);
            grid-template-areas:
                "brand actions actions"
                "transport preset device"
                "cpu cpu cpu";
            gap: 0.7rem;
            padding: 0.75rem 1rem;
        }

        .brand {
            grid-area: brand;
        }

        .transport {
            grid-area: transport;
        }

        .preset-button {
            grid-area: preset;
        }

        .actions {
            grid-area: actions;
        }

        .cpu {
            grid-area: cpu;
            justify-content: flex-start;
        }

        .device {
            grid-area: device;
        }
    }

    @media (max-width: 720px) {
        .header-bar {
            grid-template-columns: 1fr;
            grid-template-areas:
                "brand"
                "preset"
                "actions"
                "cpu"
                "device";
            padding: 0.8rem;
        }

        .transport {
            display: none;
        }

        .brand span {
            font-size: 0.86rem;
        }

        .actions {
            padding-left: 0;
            border-left: 0;
            gap: 0.32rem;
        }

        .icon-button {
            width: 2.8rem;
        }

        .bypass {
            width: 5.4rem;
            font-size: 0.68rem;
        }
    }
</style>

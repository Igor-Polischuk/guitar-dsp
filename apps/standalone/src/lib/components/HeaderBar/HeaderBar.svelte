<script lang="ts">
    import {
        ChevronDown,
        ChevronLeft,
        Guitar,
        Save,
        Settings,
        Undo2
    } from "@lucide/svelte";
</script>

<header class="header-bar">
    <div class="brand" aria-label="Guitar Processor">
        <Guitar size={27} strokeWidth={2.3} />
        <span>GUITAR PROCESSOR</span>
    </div>

    <div class="preset-shell" aria-label="Preset controls">
        <button class="icon-button back" type="button" aria-label="Previous preset" title="Previous preset">
            <ChevronLeft size={21} />
        </button>
        <button class="preset-button" type="button" aria-label="Selected preset">
            <span>Default Preset</span>
            <ChevronDown size={18} />
        </button>
    </div>

    <div class="actions" aria-label="Processor actions">
        <button class="icon-button" type="button" aria-label="Save preset" title="Save preset">
            <Save size={18} />
        </button>
        <button class="icon-button" type="button" aria-label="Undo" title="Undo">
            <Undo2 size={19} />
        </button>
        <button class="icon-button" type="button" aria-label="Settings" title="Settings">
            <Settings size={18} />
        </button>
        <button class="bypass" type="button" aria-pressed="false">BYPASS</button>
    </div>
</header>

<style>
    .header-bar {
        display: grid;
        grid-template-columns: minmax(13rem, 1fr) minmax(18rem, 31rem) minmax(17rem, 1fr);
        align-items: center;
        gap: 0.75rem;
        min-height: 4.5rem;
        padding: 0.65rem 1.45rem;
        border-bottom: var(--border-panel);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.045), transparent),
            var(--color-bg-soft);
        box-shadow: var(--shadow-inset);
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
        filter: drop-shadow(0 0 10px rgba(22, 215, 161, 0.22));
        flex: 0 0 auto;
    }

    .brand span {
        overflow: hidden;
        color: var(--color-text-soft);
        font-size: 1.08rem;
        font-weight: 560;
        letter-spacing: 0.08em;
        line-height: 1.1;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .preset-shell {
        display: grid;
        grid-template-columns: 3.1rem minmax(0, 1fr);
        min-width: 0;
        border: 1px solid rgba(119, 145, 164, 0.1);
        border-radius: var(--radius-control);
        background: rgba(8, 14, 20, 0.28);
        box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.035);
    }

    .preset-button,
    .icon-button,
    .bypass {
        min-height: 2.55rem;
        border-left: 1px solid rgba(119, 145, 164, 0.1);
        background: rgba(18, 27, 36, 0.76);
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
        width: 3.65rem;
        border-radius: var(--radius-control);
        border: 1px solid rgba(119, 145, 164, 0.11);
    }

    .icon-button.back {
        width: auto;
        border: 0;
        border-right: 1px solid rgba(119, 145, 164, 0.1);
        border-radius: var(--radius-control) 0 0 var(--radius-control);
        background: transparent;
    }

    .preset-button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.85rem;
        min-width: 0;
        border: 0;
        border-radius: 0 var(--radius-control) var(--radius-control) 0;
        font-size: 0.92rem;
        font-weight: 560;
    }

    .preset-button span {
        overflow: hidden;
        min-width: 0;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.42rem;
        min-width: 0;
    }

    .bypass {
        width: min(6.8rem, 30vw);
        border: 1px solid rgba(119, 145, 164, 0.14);
        border-radius: var(--radius-control);
        font-size: 0.78rem;
        font-weight: 720;
        letter-spacing: 0.08em;
    }

    @media (max-width: 920px) {
        .header-bar {
            grid-template-columns: minmax(0, 1fr) auto;
            grid-template-areas:
                "brand actions"
                "preset preset";
            gap: 0.7rem;
        }

        .brand {
            grid-area: brand;
        }

        .preset-shell {
            grid-area: preset;
        }

        .actions {
            grid-area: actions;
        }
    }

    @media (max-width: 560px) {
        .header-bar {
            padding: 0.8rem;
        }

        .brand span {
            font-size: 0.86rem;
        }

        .actions {
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

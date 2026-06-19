<script lang="ts">
    import { X } from "@lucide/svelte";

    export let open = false;
    export let title = "";
    export let subtitle = "";
    export let closeLabel = "Close";
    export let closeDisabled = false;
    export let onClose: (() => void) | undefined = undefined;

    function close() {
        if (!closeDisabled) {
            onClose?.();
        }
    }
</script>

{#if open}
    <div class="modal-root" role="presentation">
        <button
            class="backdrop"
            type="button"
            aria-label={closeLabel}
            disabled={closeDisabled}
            onclick={close}
        ></button>

        <div
            class="dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="modal-title"
            aria-describedby={subtitle ? "modal-subtitle" : undefined}
        >
            <header class="dialog-header">
                <div>
                    <h2 id="modal-title">{title}</h2>
                    {#if subtitle}
                        <p id="modal-subtitle">{subtitle}</p>
                    {/if}
                </div>
                <button
                    class="close"
                    type="button"
                    aria-label={closeLabel}
                    disabled={closeDisabled}
                    onclick={close}
                >
                    <X size={18} />
                </button>
            </header>

            <div class="dialog-body">
                <slot />
            </div>
        </div>
    </div>
{/if}

<style>
    .modal-root {
        position: fixed;
        inset: 0;
        z-index: 100;
        display: grid;
        padding: clamp(1rem, 4vw, 3rem);
        place-items: center;
    }

    .backdrop {
        position: absolute;
        inset: 0;
        background:
            radial-gradient(circle at 50% 25%, rgba(42, 71, 94, 0.2), transparent 30rem),
            rgba(3, 7, 11, 0.78);
        backdrop-filter: blur(5px);
        cursor: default;
    }

    .dialog {
        position: relative;
        z-index: 1;
        width: min(46rem, 100%);
        overflow: hidden;
        border: 1px solid rgba(122, 145, 166, 0.18);
        border-radius: var(--radius-panel);
        background:
            radial-gradient(circle at 50% 0, rgba(66, 96, 119, 0.16), transparent 24rem),
            linear-gradient(180deg, rgba(255, 255, 255, 0.035), transparent),
            #101821;
        box-shadow:
            0 28px 80px rgba(0, 0, 0, 0.62),
            inset 0 1px 0 rgba(255, 255, 255, 0.06);
    }

    .dialog-header {
        display: flex;
        justify-content: space-between;
        gap: 1rem;
        padding: 1.35rem 1.85rem 0.8rem;
        border-bottom: 1px solid var(--color-panel-line);
    }

    h2,
    p {
        margin: 0;
    }

    h2 {
        color: var(--color-text);
        font-size: 0.92rem;
        font-weight: 820;
        letter-spacing: 0.02em;
    }

    p {
        margin-top: 0.45rem;
        color: var(--color-text-muted);
        font-size: 0.75rem;
        font-weight: 560;
    }

    .close {
        display: grid;
        flex: 0 0 auto;
        width: 2rem;
        aspect-ratio: 1;
        place-items: center;
        border-radius: 50%;
        background: transparent;
        color: var(--color-text-muted);
        cursor: pointer;
    }

    .close:hover:not(:disabled) {
        color: var(--color-text);
        background: rgba(255, 255, 255, 0.04);
    }

    .close:disabled,
    .backdrop:disabled {
        cursor: not-allowed;
    }

    .dialog-body {
        padding: 1.25rem 1.85rem 1.6rem;
    }

    @media (max-width: 640px) {
        .modal-root {
            align-items: stretch;
            padding: 0;
        }

        .dialog {
            width: 100%;
            min-height: 100vh;
            border-radius: 0;
        }

        .dialog-header,
        .dialog-body {
            padding-inline: 1rem;
        }
    }
</style>

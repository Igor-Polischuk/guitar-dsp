<script lang="ts">
    export let type: "button" | "submit" = "button";
    export let variant: "primary" | "secondary" = "primary";
    export let disabled = false;
    export let onClick: (() => void) | undefined = undefined;
</script>

<button
    class="ui-button {variant}"
    {type}
    {disabled}
    onclick={onClick}
>
    <slot />
</button>

<style>
    .ui-button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        width: 100%;
        min-height: 2.35rem;
        padding: 0 1.2rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        cursor: pointer;
        font-size: 0.78rem;
        font-weight: 780;
        letter-spacing: 0.04em;
        transition:
            background 140ms ease,
            border-color 140ms ease,
            color 140ms ease,
            opacity 140ms ease;
    }

    .ui-button.primary {
        border-color: rgba(47, 134, 255, 0.62);
        background: linear-gradient(180deg, #2f86ff, #2168d8);
        color: #f4f8ff;
        box-shadow: 0 10px 24px rgba(47, 134, 255, 0.22);
    }

    .ui-button.secondary {
        background: rgba(17, 25, 34, 0.82);
        color: var(--color-text-soft);
    }

    .ui-button:not(:disabled):hover {
        border-color: rgba(99, 166, 255, 0.78);
        color: var(--color-text);
    }

    .ui-button:disabled {
        cursor: not-allowed;
        opacity: 0.52;
    }
</style>

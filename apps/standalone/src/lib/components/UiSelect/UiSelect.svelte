<script lang="ts">
    import { ChevronDown } from "@lucide/svelte";

    type SelectOption = {
        value: string;
        label: string;
    };

    export let id: string;
    export let label: string;
    export let description = "";
    export let value = "";
    export let options: SelectOption[] = [];
    export let disabled = false;
    export let onChange: ((value: string) => void) | undefined = undefined;
    export let onOpen: (() => void | Promise<void>) | undefined = undefined;

    let expanded = false;

    $: selectedOption = options.find((option) => option.value === value);

    function toggleDropdown() {
        if (disabled) {
            return;
        }

        if (!expanded) {
            onOpen?.();
        }

        expanded = !expanded;
    }

    function closeDropdown() {
        expanded = false;
    }

    function selectOption(nextValue: string) {
        onChange?.(nextValue);
        closeDropdown();
    }

    function handleFocusOut(event: FocusEvent) {
        const nextTarget = event.relatedTarget;

        if (nextTarget instanceof Node && event.currentTarget instanceof HTMLElement) {
            if (event.currentTarget.contains(nextTarget)) {
                return;
            }
        }

        closeDropdown();
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Escape") {
            event.preventDefault();
            closeDropdown();
        }
    }
</script>

<div class="select-field" onfocusout={handleFocusOut}>
    {#if label}
        <span class="label" id={`${id}-label`}>{label}</span>
    {/if}
    {#if description}
        <span class="description" id={`${id}-description`}>{description}</span>
    {/if}

    <div class="select-wrap">
        <button
            class="select-shell"
            class:expanded
            type="button"
            id={id}
            aria-haspopup="listbox"
            aria-expanded={expanded}
            aria-labelledby={label ? `${id}-label ${id}` : undefined}
            aria-describedby={description ? `${id}-description` : undefined}
            {disabled}
            onclick={toggleDropdown}
            onkeydown={handleKeydown}
        >
            <i aria-hidden="true"></i>
            <span class="selected">{selectedOption?.label ?? "Select device"}</span>
            <ChevronDown size={17} />
        </button>

        {#if expanded}
            <div class="options" role="listbox" aria-labelledby={label ? `${id}-label` : undefined}>
                {#each options as option}
                    <button
                        class:selected={option.value === value}
                        type="button"
                        role="option"
                        aria-selected={option.value === value}
                        onclick={() => selectOption(option.value)}
                        onkeydown={handleKeydown}
                    >
                        <span>{option.label}</span>
                    </button>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .select-field {
        display: grid;
        gap: 0.45rem;
        min-width: 0;
        font-family: inherit;
    }

    .label {
        color: var(--color-text-soft);
        font-size: 0.75rem;
        font-weight: 760;
        letter-spacing: 0.03em;
    }

    .description {
        color: var(--color-text-muted);
        font-size: 0.74rem;
        font-weight: 560;
        line-height: 1.25;
    }

    .select-wrap {
        position: relative;
        min-width: 0;
    }

    .select-shell {
        display: grid;
        grid-template-columns: auto minmax(0, 1fr) auto;
        align-items: center;
        width: 100%;
        min-height: 2.35rem;
        padding: 0 0.72rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(12, 18, 26, 0.68);
        color: var(--color-text-soft);
        cursor: pointer;
        font-family: inherit;
        text-align: left;
    }

    .select-shell.expanded {
        border-color: rgba(47, 134, 255, 0.48);
        box-shadow: 0 0 0 1px rgba(47, 134, 255, 0.08);
    }

    .select-shell:disabled {
        cursor: not-allowed;
        color: var(--color-text-muted);
        opacity: 0.68;
    }

    .select-shell i {
        width: 0.72rem;
        aspect-ratio: 1;
        margin-right: 0.55rem;
        border-radius: 50%;
        background: var(--color-accent-blue-strong);
        box-shadow: 0 0 10px rgba(47, 134, 255, 0.42);
    }

    .selected {
        overflow: hidden;
        min-width: 0;
        color: var(--color-text);
        font-family: inherit;
        font-size: 0.78rem;
        font-weight: 650;
        line-height: 1.2;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .select-shell :global(svg) {
        color: var(--color-text-muted);
        pointer-events: none;
    }

    .options {
        position: absolute;
        z-index: 20;
        top: calc(100% + 0.35rem);
        right: 0;
        left: 0;
        display: grid;
        max-height: 13.5rem;
        overflow-y: auto;
        padding: 0.35rem;
        border: 1px solid rgba(122, 145, 166, 0.18);
        border-radius: var(--radius-control);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.035), transparent),
            #0c131b;
        box-shadow: 0 16px 38px rgba(0, 0, 0, 0.46);
    }

    .options button {
        display: flex;
        align-items: center;
        min-height: 2rem;
        min-width: 0;
        padding: 0 0.55rem;
        border-radius: 0.35rem;
        background: transparent;
        color: var(--color-text-soft);
        cursor: pointer;
        font-family: inherit;
        font-size: 0.78rem;
        font-weight: 640;
        text-align: left;
    }

    .options button:hover,
    .options button.selected {
        background: rgba(47, 134, 255, 0.14);
        color: var(--color-text);
    }

    .options button span {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
</style>

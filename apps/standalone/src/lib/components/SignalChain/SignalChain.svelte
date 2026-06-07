<script lang="ts">
    import {
        Activity,
        CircleGauge,
        CirclePlus,
        Cuboid,
        Gauge,
        Package,
        Volume2,
        Zap
    } from "@lucide/svelte";

    const blocks = [
        { label: "INPUT", icon: Activity },
        { label: "NOISE GATE", icon: Activity },
        { label: "COMPRESSOR", icon: Gauge },
        { label: "TUBE SCREAMER", icon: Zap, pedal: true },
        { label: "MARSHALL PLEXI", icon: Volume2, amp: true, active: true },
        { label: "4X12 GREENBACK", icon: Package, cab: true },
        { label: "DELAY", icon: CircleGauge, delay: true },
        { label: "REVERB", icon: Cuboid },
        { label: "OUTPUT", icon: Activity }
    ];
</script>

<section class="signal-chain" aria-label="Signal chain">
    <div class="chain-scroll">
        {#each blocks as block, index}
            {#if index > 0}
                <div class="connector" aria-hidden="true"><i></i></div>
            {/if}

            <button class:active={block.active} class="block" type="button">
                <div class="visual">
                    {#if block.pedal}
                        <span class="pedal-art">
                            <i></i><i></i><i></i>
                        </span>
                    {:else if block.amp}
                        <span class="amp-art"></span>
                    {:else if block.cab}
                        <span class="cab-art"></span>
                    {:else if block.delay}
                        <span class="delay-art"></span>
                    {:else}
                        <svelte:component this={block.icon} size={34} strokeWidth={1.6} />
                    {/if}
                    <span class="menu-dots" aria-hidden="true"></span>
                </div>
                <span>{block.label}</span>
            </button>
        {/each}
    </div>

    <button class="add-block" type="button">
        <CirclePlus size={16} />
        ADD BLOCK
    </button>
</section>

<style>
    .signal-chain {
        display: grid;
        gap: 0.75rem;
        min-height: 12.4rem;
        padding: 0.75rem 1.45rem 1rem;
        border-top: var(--border-panel);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.018), transparent),
            var(--color-panel-deep);
    }

    .chain-scroll {
        display: grid;
        grid-template-columns: repeat(17, auto);
        align-items: center;
        min-width: 0;
        overflow-x: auto;
        padding: 0.1rem 0 0.25rem;
        scrollbar-width: thin;
    }

    .block {
        display: grid;
        align-content: center;
        justify-items: center;
        gap: 0.45rem;
        width: clamp(5.6rem, 8.2vw, 8.8rem);
        min-height: 7.1rem;
        padding: 0.55rem 0.55rem 0.65rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-panel);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.035), transparent),
            rgba(20, 29, 39, 0.86);
        color: var(--color-text-soft);
        cursor: pointer;
        box-shadow:
            inset 0 1px 0 rgba(255, 255, 255, 0.04),
            0 10px 20px rgba(0, 0, 0, 0.22);
    }

    .block.active {
        border-color: rgba(47, 134, 255, 0.74);
        background:
            linear-gradient(180deg, rgba(47, 134, 255, 0.1), transparent),
            rgba(22, 33, 45, 0.94);
        box-shadow:
            inset 0 0 0 1px rgba(47, 134, 255, 0.22),
            0 0 18px rgba(47, 134, 255, 0.17);
    }

    .visual {
        position: relative;
        display: grid;
        width: 100%;
        min-height: 3.8rem;
        place-items: center;
        color: var(--color-text-muted);
    }

    .block span:last-child {
        overflow: hidden;
        width: 100%;
        color: var(--color-text-soft);
        font-size: 0.72rem;
        font-weight: 740;
        letter-spacing: 0.02em;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .menu-dots {
        position: absolute;
        top: 0.1rem;
        right: 0.12rem;
        width: 0.22rem;
        height: 1.1rem;
        background:
            radial-gradient(circle, var(--color-text-muted) 0 0.1rem, transparent 0.11rem) 0 0 / 100% 0.36rem;
        opacity: 0.75;
    }

    .pedal-art {
        position: relative;
        width: 3.6rem;
        height: 3.4rem;
        border-radius: 0.32rem;
        background: linear-gradient(180deg, #1ca66a, #0c7146);
        box-shadow:
            inset 0 1px 0 rgba(255, 255, 255, 0.22),
            inset 0 -10px 16px rgba(0, 0, 0, 0.18);
    }

    .pedal-art i {
        position: absolute;
        top: 0.72rem;
        width: 0.7rem;
        aspect-ratio: 1;
        border-radius: 50%;
        background: #10161d;
        box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.22);
    }

    .pedal-art i:nth-child(1) {
        left: 0.52rem;
    }

    .pedal-art i:nth-child(2) {
        left: 1.45rem;
    }

    .pedal-art i:nth-child(3) {
        left: 2.38rem;
    }

    .amp-art,
    .cab-art {
        display: block;
        width: 5.5rem;
        height: 2.65rem;
        border-radius: 0.2rem;
        background:
            linear-gradient(180deg, transparent 0 48%, #b79148 48% 76%, transparent 76%),
            linear-gradient(135deg, rgba(255, 255, 255, 0.09), transparent 28%),
            #171a1c;
        border: 1px solid rgba(184, 145, 72, 0.42);
        box-shadow:
            inset 0 0 0 2px rgba(184, 145, 72, 0.12),
            0 8px 16px rgba(0, 0, 0, 0.34);
    }

    .cab-art {
        width: 4.8rem;
        height: 3.6rem;
        border-color: rgba(116, 132, 143, 0.2);
        background:
            repeating-linear-gradient(0deg, rgba(255, 255, 255, 0.025) 0 1px, transparent 1px 5px),
            #171b1f;
    }

    .delay-art {
        display: block;
        width: 3rem;
        aspect-ratio: 1;
        border: 3px dotted rgba(117, 134, 150, 0.6);
        border-radius: 50%;
        background:
            conic-gradient(var(--color-accent-green) 0 122deg, transparent 122deg),
            #101820;
        box-shadow: inset 0 0 0 0.45rem #17212c;
    }

    .connector {
        position: relative;
        display: grid;
        width: clamp(1.3rem, 2.2vw, 2.25rem);
        place-items: center;
    }

    .connector::before {
        content: "";
        width: 100%;
        height: 2px;
        background: rgba(168, 179, 189, 0.72);
    }

    .connector i {
        position: absolute;
        width: 0.68rem;
        aspect-ratio: 1;
        border: 2px solid rgba(218, 225, 232, 0.7);
        border-radius: 50%;
        background: var(--color-panel-deep);
        box-shadow: 0 0 7px rgba(255, 255, 255, 0.12);
    }

    .add-block {
        justify-self: center;
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        min-height: 2.2rem;
        padding: 0 1.45rem;
        border: 1px solid var(--color-panel-line);
        border-radius: var(--radius-control);
        background: rgba(17, 25, 34, 0.75);
        color: var(--color-text-soft);
        cursor: pointer;
        font-size: 0.74rem;
        font-weight: 760;
        letter-spacing: 0.02em;
    }

    @media (max-width: 760px) {
        .signal-chain {
            padding: 0.75rem 1rem 1rem;
        }

        .block {
            width: 7rem;
        }
    }
</style>

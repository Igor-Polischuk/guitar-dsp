<script lang="ts">
    import { Maximize2 } from "@lucide/svelte";

    const inputPath =
        "M0 215 C22 202 28 156 55 145 C82 134 91 166 113 158 C134 149 132 128 154 134 C175 141 175 107 202 115 C229 123 231 96 260 103 C290 110 283 82 312 90 C341 98 342 121 369 112 C395 104 397 130 421 118 C446 107 452 139 478 128 C504 116 503 150 529 145 C555 140 555 162 582 149 C609 136 604 166 631 157 C657 147 662 124 688 132 C714 140 711 162 737 151 C762 140 763 164 790 152 C817 140 813 168 841 161 C870 155 866 177 896 162 C925 147 923 173 951 160 C979 147 981 172 1008 159 C1036 146 1035 176 1064 164 C1092 152 1095 181 1124 170 C1152 159 1169 192 1200 202";
    const outputPath =
        "M0 285 C28 277 31 248 59 239 C87 229 92 250 119 236 C147 222 145 205 174 214 C203 223 200 254 230 246 C260 238 259 223 289 215 C319 207 319 237 349 234 C379 230 381 206 411 210 C441 214 438 231 469 222 C500 213 500 235 530 225 C560 216 559 235 590 224 C621 213 620 237 651 226 C683 215 681 241 712 230 C742 220 745 240 775 232 C806 224 806 245 837 235 C868 226 865 251 897 241 C929 231 931 253 962 245 C994 237 997 259 1028 249 C1060 239 1061 258 1092 251 C1124 244 1135 259 1166 270 C1182 275 1192 287 1200 292";

    const frequencyTicks = ["20", "50", "100", "200", "500", "1k", "2k", "5k", "10k", "20k"];
    const levelTicks = ["+12", "+6", "0", "-6", "-12", "-18", "-24", "-30", "-42", "-60"];
</script>

<section class="analyzer" aria-label="Input and output spectrum analyzer">
    <div class="tabs" role="tablist" aria-label="Analyzer views">
        <button class="tab active" type="button" role="tab" aria-selected="true">SPECTRUM</button>
        <button class="tab" type="button" role="tab">EQ RESPONSE</button>
        <button class="tab" type="button" role="tab">CAB RESPONSE</button>
        <button class="tab" type="button" role="tab">GAIN REDUCTION</button>
        <button class="tab" type="button" role="tab">STEREO FIELD</button>
        <div class="tools">
            <span>Hold</span>
            <button class="toggle" type="button" aria-label="Hold spectrum"><i></i></button>
            <button class="speed" type="button">Fast</button>
            <button class="expand" type="button" aria-label="Expand analyzer" title="Expand analyzer">
                <Maximize2 size={18} />
            </button>
        </div>
    </div>

    <div class="scope">
        <div class="legend">
            <span class="legend-item input"><i></i>INPUT</span>
            <span class="legend-item output"><i></i>OUTPUT</span>
            <span class="legend-item peak"><i></i>PEAK</span>
        </div>

        <svg viewBox="0 0 1200 360" preserveAspectRatio="none" aria-hidden="true">
            <defs>
                <linearGradient id="inputFill" x1="0" x2="0" y1="0" y2="1">
                    <stop offset="0%" stop-color="var(--color-accent-green)" stop-opacity="0.4" />
                    <stop offset="100%" stop-color="var(--color-accent-green)" stop-opacity="0" />
                </linearGradient>
                <linearGradient id="outputFill" x1="0" x2="0" y1="0" y2="1">
                    <stop offset="0%" stop-color="var(--color-accent-purple)" stop-opacity="0.42" />
                    <stop offset="100%" stop-color="var(--color-accent-purple)" stop-opacity="0" />
                </linearGradient>
            </defs>

            <path class="fill input-fill" d={`${inputPath} L1200 330 L0 330 Z`} />
            <path class="line input-line" d={inputPath} />
            <path class="fill output-fill" d={`${outputPath} L1200 338 L0 338 Z`} />
            <path class="line output-line" d={outputPath} />
        </svg>

        <div class="frequency-axis" aria-hidden="true">
            {#each frequencyTicks as tick}
                <span>{tick}</span>
            {/each}
        </div>
        <div class="level-axis" aria-hidden="true">
            {#each levelTicks as tick}
                <span>{tick}</span>
            {/each}
        </div>
    </div>
</section>

<style>
    .analyzer {
        display: grid;
        grid-template-rows: 2.7rem minmax(18rem, 1fr);
        min-height: clamp(22rem, 34vw, 31rem);
        border: var(--border-panel);
        border-radius: var(--radius-panel);
        overflow: hidden;
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.025), transparent),
            var(--color-panel-deep);
        box-shadow:
            inset 0 1px 0 rgba(255, 255, 255, 0.035),
            0 15px 34px rgba(0, 0, 0, 0.28);
    }

    .tabs {
        display: grid;
        grid-template-columns: repeat(5, minmax(8.5rem, auto)) minmax(16rem, 1fr);
        align-items: stretch;
        border-bottom: 1px solid var(--color-panel-line);
        background: rgba(17, 25, 34, 0.78);
    }

    .tab {
        padding: 0 1.15rem;
        border-right: 1px solid rgba(122, 145, 166, 0.08);
        background: transparent;
        color: var(--color-text-muted);
        cursor: pointer;
        font-size: 0.74rem;
        font-weight: 690;
        white-space: nowrap;
    }

    .tab.active {
        border-bottom: 2px solid var(--color-accent-blue-strong);
        background: linear-gradient(180deg, rgba(47, 134, 255, 0.16), rgba(47, 134, 255, 0.03));
        color: var(--color-text);
    }

    .tools {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 0.85rem;
        min-width: 0;
        padding: 0 0.95rem;
        color: var(--color-text-muted);
        font-size: 0.74rem;
        font-weight: 650;
    }

    .toggle {
        position: relative;
        width: 3rem;
        height: 1.55rem;
        border: 1px solid var(--color-panel-line);
        border-radius: 999px;
        background: rgba(8, 13, 20, 0.74);
        cursor: pointer;
    }

    .toggle i {
        position: absolute;
        top: 0.22rem;
        left: 0.24rem;
        width: 1.05rem;
        aspect-ratio: 1;
        border-radius: 50%;
        background: var(--color-text-soft);
        box-shadow: 0 2px 7px rgba(0, 0, 0, 0.45);
    }

    .speed,
    .expand {
        min-height: 1.9rem;
        border: 1px solid var(--color-panel-line);
        border-radius: 0.38rem;
        background: rgba(12, 18, 26, 0.6);
        color: var(--color-text-soft);
        cursor: pointer;
    }

    .speed {
        min-width: 5rem;
        padding: 0 0.85rem;
        text-align: left;
    }

    .expand {
        display: inline-grid;
        width: 2rem;
        place-items: center;
        border-color: transparent;
        background: transparent;
    }

    .scope {
        position: relative;
        min-height: 0;
        background-image:
            linear-gradient(var(--color-grid) 1px, transparent 1px),
            linear-gradient(90deg, var(--color-grid) 1px, transparent 1px),
            linear-gradient(var(--color-grid-strong) 1px, transparent 1px);
        background-position: center;
        background-size:
            3.2rem 3.2rem,
            3.2rem 3.2rem,
            100% 50%;
    }

    .scope::after {
        content: "";
        position: absolute;
        inset: 0;
        background:
            linear-gradient(180deg, transparent 0 74%, rgba(4, 8, 12, 0.28)),
            radial-gradient(circle at 50% 0, rgba(52, 85, 102, 0.1), transparent 38rem);
        pointer-events: none;
    }

    .legend {
        position: absolute;
        z-index: 3;
        top: 1rem;
        left: 1.4rem;
        display: flex;
        gap: 1.55rem;
    }

    .legend-item {
        display: inline-flex;
        align-items: center;
        gap: 0.55rem;
        color: var(--color-text-muted);
        font-size: 0.71rem;
        font-weight: 730;
        letter-spacing: 0.04em;
    }

    .legend-item i {
        width: 0.78rem;
        height: 0.18rem;
        border-radius: 999px;
    }

    .legend-item.input i {
        background: var(--color-accent-green);
        box-shadow: 0 0 10px rgba(22, 215, 161, 0.45);
    }

    .legend-item.output i {
        background: var(--color-accent-purple);
        box-shadow: 0 0 10px rgba(168, 92, 244, 0.45);
    }

    .legend-item.peak i {
        background: rgba(146, 157, 169, 0.65);
    }

    svg {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
    }

    .line {
        fill: none;
        stroke-width: 2;
        vector-effect: non-scaling-stroke;
    }

    .fill {
        stroke: none;
    }

    .input-line {
        stroke: var(--color-accent-green);
        filter: drop-shadow(0 0 7px rgba(22, 215, 161, 0.18));
    }

    .output-line {
        stroke: var(--color-accent-purple);
        filter: drop-shadow(0 0 7px rgba(168, 92, 244, 0.2));
    }

    .input-fill {
        fill: url("#inputFill");
    }

    .output-fill {
        fill: url("#outputFill");
    }

    .frequency-axis {
        position: absolute;
        right: 2.9rem;
        bottom: 1.15rem;
        left: 1.95rem;
        display: flex;
        justify-content: space-between;
        color: var(--color-text-muted);
        font-size: 0.72rem;
        font-weight: 640;
        pointer-events: none;
    }

    .level-axis {
        position: absolute;
        top: 1.05rem;
        right: 0.75rem;
        bottom: 2.05rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        color: var(--color-text-muted);
        font-size: 0.68rem;
        font-weight: 640;
        text-align: right;
        pointer-events: none;
    }

    @media (max-width: 980px) {
        .analyzer {
            grid-template-rows: auto minmax(20rem, 1fr);
        }

        .tabs {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }

        .tools {
            grid-column: 1 / -1;
            justify-content: flex-start;
            min-height: 2.6rem;
        }
    }

    @media (max-width: 680px) {
        .analyzer {
            min-height: 21rem;
        }

        .legend {
            left: 0.9rem;
            gap: 0.7rem;
        }

        .tab {
            padding: 0.7rem 0.55rem;
            font-size: 0.68rem;
        }
    }
</style>

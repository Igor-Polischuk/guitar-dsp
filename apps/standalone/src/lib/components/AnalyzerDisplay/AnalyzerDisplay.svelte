<script lang="ts">
    const inputPath =
        "M0 137 C26 129 40 77 70 90 C103 103 98 139 122 139 C145 138 148 105 169 112 C187 118 190 151 213 149 C236 146 235 132 255 134 C277 136 273 148 297 139 C321 129 337 122 354 102 C376 77 399 128 420 142 C441 156 454 111 474 114 C493 116 493 85 519 92 C545 100 542 140 568 145 C594 151 595 121 619 126 C644 131 643 106 666 110 C691 114 693 128 719 116 C744 105 750 87 773 94 C797 101 795 143 821 145 C847 147 851 107 879 91 C907 75 918 129 948 145 C977 161 988 116 1013 99 C1037 83 1048 126 1071 129 C1092 132 1102 118 1122 109 C1146 98 1155 153 1182 111 C1191 98 1198 90 1200 88";
    const outputPath =
        "M0 266 C28 264 37 236 63 241 C87 247 93 198 126 209 C158 220 148 263 179 270 C210 277 229 245 253 239 C278 232 287 257 310 258 C335 259 339 219 367 213 C394 207 404 236 428 258 C451 281 461 243 482 240 C506 237 505 200 534 202 C564 204 560 247 588 244 C617 241 620 220 648 229 C676 238 679 241 704 230 C731 218 735 194 761 198 C787 201 795 252 822 263 C848 275 856 238 879 238 C904 239 910 217 935 226 C958 235 961 270 989 263 C1018 255 1025 214 1054 206 C1083 199 1091 255 1119 261 C1147 267 1157 226 1181 228 C1193 229 1199 238 1200 238";
</script>

<section class="analyzer" aria-label="Input and output signal analyzer">
    <div class="legend">
        <span class="legend-item input"><i></i>INPUT</span>
        <span class="legend-item output"><i></i>OUTPUT</span>
    </div>

    <div class="scope">
        <span class="track-label input">INPUT</span>
        <span class="track-label output">OUTPUT</span>
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

            <path class="fill input-fill" d={`${inputPath} L1200 180 L0 180 Z`} />
            <path class="line input-line" d={inputPath} />
            <path class="fill output-fill" d={`${outputPath} L1200 305 L0 305 Z`} />
            <path class="line output-line" d={outputPath} />
        </svg>
    </div>
</section>

<style>
    .analyzer {
        position: relative;
        min-height: clamp(18rem, 33vw, 28rem);
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

    .legend {
        position: absolute;
        z-index: 3;
        top: 0.7rem;
        right: 0.9rem;
        display: flex;
        gap: 0.4rem;
    }

    .legend-item {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        min-height: 1.95rem;
        padding: 0 0.9rem;
        border: 1px solid rgba(119, 145, 164, 0.07);
        border-radius: 0.42rem;
        background: rgba(22, 31, 41, 0.82);
        color: var(--color-text-muted);
        font-size: 0.75rem;
        font-weight: 680;
        letter-spacing: 0.04em;
    }

    .legend-item i {
        width: 0.58rem;
        height: 0.58rem;
        border-radius: 50%;
    }

    .legend-item.input i {
        background: var(--color-accent-green);
        box-shadow: 0 0 10px rgba(22, 215, 161, 0.45);
    }

    .legend-item.output i {
        background: var(--color-accent-purple);
        box-shadow: 0 0 10px rgba(168, 92, 244, 0.45);
    }

    .scope {
        position: absolute;
        inset: 0;
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

    svg {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
    }

    .track-label {
        position: absolute;
        left: 2.35rem;
        z-index: 2;
        font-size: 0.74rem;
        font-weight: 760;
        letter-spacing: 0.05em;
    }

    .track-label.input {
        top: 2.15rem;
        color: var(--color-accent-green);
    }

    .track-label.output {
        top: 52%;
        color: var(--color-accent-purple);
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

    @media (max-width: 680px) {
        .analyzer {
            min-height: 21rem;
        }

        .legend {
            right: 0.55rem;
            gap: 0.28rem;
        }

        .legend-item {
            min-height: 1.75rem;
            padding: 0 0.55rem;
            font-size: 0.68rem;
        }

        .track-label {
            left: 1rem;
        }
    }
</style>

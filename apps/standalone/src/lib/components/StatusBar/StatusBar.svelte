<footer class="status-bar" aria-label="Audio engine status">
    <div class="status-cell">
        <span>INPUT MODE</span>
        <strong>Instrument</strong>
    </div>
    <div class="status-cell">
        <span>INPUT IMPEDANCE</span>
        <strong>Hi-Z</strong>
    </div>
    <div class="status-cell">
        <span>GUITAR MATCH</span>
        <strong>Off</strong>
    </div>
    <div class="status-cell tempo">
        <span>TEMPO</span>
        <strong>120.0 BPM</strong>
        <button type="button">TAP</button>
    </div>
    <div class="status-cell audio">
        <span>AUDIO</span>
        <strong>48.0 kHz</strong>
        <strong>24 bit</strong>
    </div>
    <div class="status-cell">
        <span>BUFFER SIZE</span>
        <strong>128 samples</strong>
    </div>
    <div class="status-cell">
        <span>LATENCY</span>
        <strong>2.7 ms</strong>
    </div>
    <div class="status-cell cpu">
        <span>CPU</span>
        <strong>34%</strong>
        <i aria-hidden="true"><b></b></i>
    </div>
    <div class="status-cell">
        <span>DRIVER</span>
        <strong>ASIO</strong>
    </div>
    <div class="status-cell device">
        <span>DEVICE</span>
        <strong>Focusrite Scarlett 2i2</strong>
    </div>
</footer>

<style>
    .status-bar {
        display: grid;
        grid-template-columns:
            minmax(7.8rem, 1fr) minmax(8.5rem, 1fr) minmax(7rem, 0.9fr)
            minmax(10rem, 1.2fr) minmax(11rem, 1.25fr) minmax(8.2rem, 1fr)
            minmax(7rem, 0.8fr) minmax(8.5rem, 1fr) minmax(6.5rem, 0.75fr)
            minmax(11rem, 1.4fr);
        align-items: stretch;
        min-height: 4.2rem;
        border-top: var(--border-panel);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.025), transparent),
            #0f171f;
        color: var(--color-text-muted);
    }

    .status-cell {
        display: grid;
        align-content: center;
        gap: 0.25rem;
        min-width: 0;
        padding: 0.65rem 1.1rem;
        border-right: 1px solid rgba(119, 145, 164, 0.09);
    }

    .status-cell span {
        overflow: hidden;
        color: var(--color-text-faint);
        font-size: 0.62rem;
        font-weight: 760;
        letter-spacing: 0.04em;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .status-cell strong {
        overflow: hidden;
        color: var(--color-text-soft);
        font-size: 0.78rem;
        font-weight: 610;
        line-height: 1.2;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .tempo,
    .audio,
    .cpu {
        grid-template-columns: auto auto;
        column-gap: 0.65rem;
    }

    .tempo span,
    .audio span,
    .cpu span {
        grid-column: 1 / -1;
    }

    .tempo button {
        min-height: 1.8rem;
        padding: 0 0.8rem;
        border: 1px solid var(--color-panel-line);
        border-radius: 0.36rem;
        background: rgba(17, 25, 34, 0.82);
        color: var(--color-text-muted);
        cursor: pointer;
        font-size: 0.68rem;
        font-weight: 740;
    }

    .cpu {
        grid-template-columns: auto 1fr;
    }

    .cpu i {
        align-self: center;
        display: block;
        width: 4.3rem;
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
        box-shadow: 0 0 9px rgba(22, 215, 161, 0.32);
    }

    @media (max-width: 1160px) {
        .status-bar {
            grid-template-columns: repeat(5, minmax(0, 1fr));
        }
    }

    @media (max-width: 720px) {
        .status-bar {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }

        .status-cell {
            min-height: 3.25rem;
            padding: 0.55rem 0.8rem;
        }
    }
</style>

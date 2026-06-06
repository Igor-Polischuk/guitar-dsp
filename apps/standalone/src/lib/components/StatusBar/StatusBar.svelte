<footer class="status-bar" aria-label="Audio engine status">
    <div class="status-group input">
        <span>INPUT</span>
        <strong>Hi-Z</strong>
    </div>
    <div class="status-group">
        <span>44.1 kHz</span>
    </div>
    <div class="status-group">
        <span>24 bit</span>
    </div>
    <div class="cpu">
        <span>CPU: 12%</span>
        <i aria-hidden="true"><b></b></i>
    </div>
    <div class="version">v1.0.0</div>
</footer>

<style>
    .status-bar {
        display: grid;
        grid-template-columns: auto auto auto minmax(9rem, 1fr) auto;
        align-items: center;
        min-height: 3.35rem;
        border-top: var(--border-panel);
        background:
            linear-gradient(180deg, rgba(255, 255, 255, 0.025), transparent),
            #101821;
        color: var(--color-text-muted);
    }

    .status-group {
        display: flex;
        align-items: center;
        gap: 0.7rem;
        min-height: 3.35rem;
        padding: 0 1.4rem;
        border-right: 1px solid rgba(119, 145, 164, 0.09);
        font-size: 0.86rem;
        font-weight: 570;
        white-space: nowrap;
    }

    .status-group strong {
        padding: 0.35rem 0.8rem;
        border-radius: 0.42rem;
        background: rgba(22, 215, 161, 0.1);
        color: var(--color-accent-green);
        font-size: 0.82rem;
        font-weight: 780;
    }

    .cpu {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 0.9rem;
        min-width: 0;
        padding: 0 1.5rem;
        font-size: 0.78rem;
        font-weight: 620;
        white-space: nowrap;
    }

    .cpu i {
        display: block;
        width: clamp(5rem, 9vw, 7rem);
        height: 0.36rem;
        border-radius: 999px;
        background: rgba(95, 115, 132, 0.18);
        overflow: hidden;
    }

    .cpu b {
        display: block;
        width: 24%;
        height: 100%;
        border-radius: inherit;
        background: var(--color-accent-green);
        box-shadow: 0 0 10px rgba(22, 215, 161, 0.25);
    }

    .version {
        padding: 0 1.45rem 0 0.9rem;
        color: var(--color-text-muted);
        font-size: 0.78rem;
        font-weight: 620;
        white-space: nowrap;
    }

    @media (max-width: 720px) {
        .status-bar {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }

        .status-group,
        .cpu,
        .version {
            min-height: 2.75rem;
            padding: 0 0.9rem;
            border-top: 1px solid rgba(119, 145, 164, 0.09);
            font-size: 0.75rem;
        }

        .cpu,
        .version {
            justify-content: flex-start;
        }
    }
</style>

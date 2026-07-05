use std::fs;
use std::io;
use std::path::Path;

const DEFAULT_OUT_DIR: &str = "dist";
const CUSTOM_DOMAIN: &str = "onix-os.com";

fn main() -> io::Result<()> {
    let out_dir = std::env::var("ONIX_SITE_OUT").unwrap_or_else(|_| DEFAULT_OUT_DIR.to_owned());
    let out = Path::new(&out_dir);
    let assets = out.join("assets");

    fs::create_dir_all(&assets)?;
    fs::write(out.join("index.html"), html())?;
    fs::write(out.join("styles.css"), css())?;
    fs::write(out.join(".nojekyll"), "")?;
    fs::write(out.join("CNAME"), format!("{CUSTOM_DOMAIN}\n"))?;
    fs::copy("profile/onix.svg", assets.join("onix.svg"))?;
    fs::copy("profile/onix.png", assets.join("onix.png"))?;

    println!("Built {}/index.html", out.display());
    Ok(())
}

fn html() -> String {
    let contract_rows = [
        ("/usr", "moss", "atomic machine state"),
        ("/.moss", "moss", "content store + fstx history"),
        ("/nix", "Nix", "persistent toolbox and profiles"),
        (
            "/run/opengl-driver",
            "ONIX seam",
            "graphics bridge for Nix GUI apps",
        ),
        (
            "setuid + kernel modules",
            "moss only",
            "no toolbox ownership",
        ),
    ];

    let phase_cards = [
        (
            "00",
            "Forge",
            "Alpine musl VM builds moss, boulder, and the first stones.",
            "active",
        ),
        (
            "01",
            "Base Stones",
            "Author the smallest musl userland and local onix moss repo.",
            "",
        ),
        (
            "02",
            "First Boot",
            "Boot ONIX in QEMU with BLS entries and moss state history.",
            "",
        ),
        (
            "03",
            "Nix Plane",
            "Prove multi-user Nix and moss rollbacks remain independent.",
            "",
        ),
        (
            "05",
            "Desktop",
            "Make Nix GL apps render through the machine-owned graphics stack.",
            "",
        ),
    ];

    let contract = contract_rows
        .iter()
        .map(|(surface, owner, note)| {
            format!(
                r#"<div class="contract-row">
                    <code>{surface}</code>
                    <strong>{owner}</strong>
                    <span>{note}</span>
                </div>"#
            )
        })
        .collect::<String>();

    let phases = phase_cards
        .iter()
        .map(|(num, title, body, state)| {
            format!(
                r#"<article class="phase {state}">
                    <span>{num}</span>
                    <h3>{title}</h3>
                    <p>{body}</p>
                </article>"#
            )
        })
        .collect::<String>();

    format!(
        r##"<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>ONIX - Atomic musl base, persistent Nix toolbox</title>
    <meta name="description" content="ONIX is an atomic musl operating system built from scratch with moss and boulder, with a persistent Nix toolbox on top.">
    <link rel="icon" href="assets/onix.svg" type="image/svg+xml">
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="ambient" aria-hidden="true">
        <span class="beam beam-a"></span>
        <span class="beam beam-b"></span>
        <span class="beam beam-c"></span>
    </div>

    <header class="site-header">
        <a class="brand" href="#top" aria-label="ONIX home">
            <img src="assets/onix.svg" alt="" width="38" height="38">
            <span>ONIX</span>
        </a>
        <nav aria-label="Primary">
            <a href="#architecture">Architecture</a>
            <a href="#contract">Contract</a>
            <a href="#roadmap">Roadmap</a>
            <a href="#deploy">Deploy</a>
        </nav>
    </header>

    <main id="top">
        <section class="hero">
            <div class="hero-copy">
                <p class="eyebrow">scratch-built musl OS / atomic base / persistent Nix</p>
                <h1>ONIX</h1>
                <p class="lede">The hard machine layer stays moss-managed and atomic. The developer world grows in a persistent Nix toolbox. Roll one back without corrupting the other.</p>
                <div class="hero-actions" aria-label="Project focus">
                    <a class="button primary" href="#architecture">Trace the stack</a>
                    <a class="button ghost" href="#deploy">Publish path</a>
                </div>
            </div>

            <div class="hero-console" aria-label="ONIX build console preview">
                <div class="console-top">
                    <span></span>
                    <span></span>
                    <span></span>
                    <strong>quarry:/build/onix</strong>
                </div>
                <div class="console-body">
                    <p><span class="prompt">$</span> make phase 005</p>
                    <p class="ok">stone:onix-hello built, indexed, installed</p>
                    <p><span class="prompt">$</span> moss state list</p>
                    <p>active <b>6649-a17c</b> / previous <b>6649-93fd</b></p>
                    <p><span class="prompt">$</span> onix status --planes</p>
                    <div class="status-bars">
                        <i style="--value: 74%"></i>
                        <i style="--value: 91%"></i>
                        <i style="--value: 58%"></i>
                    </div>
                    <p class="ok">machine coherent / nix untouched / opengl bridge ready</p>
                </div>
                <div class="console-orbit" aria-hidden="true">
                    <img src="assets/onix.svg" alt="">
                </div>
            </div>
        </section>

        <section class="ticker" aria-label="ONIX project markers">
            <div>
                <span>moss owns /usr</span>
                <span>Nix owns /nix</span>
                <span>musl base</span>
                <span>glibc apps ride above</span>
                <span>rollback drills required</span>
                <span>6649 is the magic number</span>
            </div>
        </section>

        <section class="section architecture" id="architecture">
            <div class="section-heading">
                <p class="eyebrow">Architecture</p>
                <h2>Three layers with one rule: no silent ownership drift.</h2>
            </div>
            <div class="stack-grid">
                <article class="stack-card toolbox">
                    <span>01</span>
                    <h3>User toolbox plane</h3>
                    <p>Dev shells, profiles, flakes, language stacks, GUI leaf apps, and the long tail live in persistent Nix.</p>
                    <code>/nix/store -> user velocity</code>
                </article>
                <article class="stack-card seam">
                    <span>02</span>
                    <h3>Integration seam</h3>
                    <p>ONIX stones seed nix-daemon, nixbld users, defaults, runtime dirs, locale glue, and the OpenGL bridge.</p>
                    <code>/run/opengl-driver -> coherent GUI</code>
                </article>
                <article class="stack-card machine">
                    <span>03</span>
                    <h3>Machine plane</h3>
                    <p>musl base, kernel, initrd, boot entries, firmware, Mesa, PipeWire, portals, and compositor are moss state.</p>
                    <code>/.moss -> rollback memory</code>
                </article>
            </div>
        </section>

        <section class="split-lab" id="contract">
            <div class="lab-copy">
                <p class="eyebrow">Constitution</p>
                <h2>The OS is boring where it must be, sharp where it matters.</h2>
                <p>ONIX is built to make the contract visible. Every surface has a single owner. Every rollback has a blast radius. Every escape hatch is explicit.</p>
            </div>
            <div class="contract-table" aria-label="Ownership contract">
                {contract}
            </div>
        </section>

        <section class="section feature-band">
            <article>
                <span class="chip">atomic</span>
                <h3>Machine updates are transactions.</h3>
                <p>moss swaps active state, preserves history, and gives the boot menu a memory of working systems.</p>
            </article>
            <article>
                <span class="chip">persistent</span>
                <h3>The toolbox survives the ground shift.</h3>
                <p>Nix profiles, stores, and dev environments stay on /persist and do not belong to the machine manager.</p>
            </article>
            <article>
                <span class="chip">developer-first</span>
                <h3>The CLI says which plane it touches.</h3>
                <p><code>onix update</code>, <code>onix rollback</code>, <code>onix doctor</code>, and <code>onix gc</code> keep the boundary legible.</p>
            </article>
        </section>

        <section class="section matrix">
            <div class="section-heading">
                <p class="eyebrow">Phase 3 gate</p>
                <h2>The composition matrix is the product.</h2>
            </div>
            <div class="matrix-grid">
                <div class="matrix-cell pass">
                    <span>PASS</span>
                    <strong>nix profile install ripgrep -> reboot</strong>
                    <p>tool still on PATH</p>
                </div>
                <div class="matrix-cell pass">
                    <span>PASS</span>
                    <strong>moss rollback -> reboot</strong>
                    <p>Nix profile untouched</p>
                </div>
                <div class="matrix-cell warn">
                    <span>RISK</span>
                    <strong>musl Mesa meets glibc app</strong>
                    <p>solve at OpenGL seam</p>
                </div>
                <div class="matrix-cell pass">
                    <span>PASS</span>
                    <strong>moss prune + nix store gc</strong>
                    <p>no cross-corruption</p>
                </div>
            </div>
        </section>

        <section class="roadmap" id="roadmap">
            <div class="section-heading">
                <p class="eyebrow">Roadmap</p>
                <h2>Phase gates over vibes.</h2>
            </div>
            <div class="phase-track">
                {phases}
            </div>
        </section>

        <section class="deploy" id="deploy">
            <div>
                <p class="eyebrow">Static deploy</p>
                <h2>Rust generates the site. GitHub Pages serves the artifact.</h2>
                <p>The website is plain files: HTML, CSS, CNAME, .nojekyll, and logo assets. The workflow builds with Nix, publishes <code>dist/</code> to <code>gh-pages</code>, and keeps <code>onix-os.com</code> attached.</p>
            </div>
            <pre aria-label="GitHub Pages deploy command"><code>nix develop --command just build
git push origin main

# action:
#   cargo run --release
#   publish ./dist -> gh-pages
#   serve onix-os.com</code></pre>
        </section>
    </main>

    <footer>
        <a class="brand" href="#top" aria-label="ONIX home">
            <img src="assets/onix.svg" alt="" width="30" height="30">
            <span>ONIX</span>
        </a>
        <p>Atomic musl base. Persistent Nix toolbox. Built from scratch with moss and boulder.</p>
    </footer>
</body>
</html>
"##
    )
}

fn css() -> &'static str {
    r#":root {
    color-scheme: dark;
    --bg: #070a0b;
    --bg-2: #0b1113;
    --panel: rgba(16, 24, 26, 0.78);
    --panel-strong: rgba(21, 31, 34, 0.94);
    --ink: #eef7ef;
    --muted: #9aa9a4;
    --line: rgba(205, 239, 222, 0.14);
    --steel: #6fa2d7;
    --steel-deep: #4f6e91;
    --ember: #e7590f;
    --ember-hot: #ff9a3d;
    --moss: #8ab06e;
    --cyan: #62e7d3;
    --violet: #bca7ff;
    --yellow: #f3cf65;
    --shadow: rgba(0, 0, 0, 0.45);
}

* {
    box-sizing: border-box;
}

html {
    scroll-behavior: smooth;
}

body {
    min-width: 320px;
    margin: 0;
    color: var(--ink);
    background:
        radial-gradient(circle at 12% 8%, rgba(231, 89, 15, 0.2), transparent 28rem),
        radial-gradient(circle at 82% 12%, rgba(98, 231, 211, 0.14), transparent 30rem),
        radial-gradient(circle at 76% 86%, rgba(111, 162, 215, 0.18), transparent 36rem),
        linear-gradient(135deg, #070a0b 0%, #0c1113 42%, #091010 100%);
    font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    overflow-x: hidden;
}

body::before {
    position: fixed;
    inset: 0;
    z-index: -3;
    pointer-events: none;
    content: "";
    background-image:
        linear-gradient(rgba(255, 255, 255, 0.035) 1px, transparent 1px),
        linear-gradient(90deg, rgba(255, 255, 255, 0.035) 1px, transparent 1px);
    background-size: 42px 42px;
    mask-image: linear-gradient(to bottom, black, transparent 86%);
}

body::after {
    position: fixed;
    inset: 0;
    z-index: -2;
    pointer-events: none;
    content: "";
    background: repeating-linear-gradient(
        to bottom,
        transparent 0,
        transparent 5px,
        rgba(255, 255, 255, 0.025) 6px
    );
    opacity: 0.38;
}

a {
    color: inherit;
    text-decoration: none;
}

code,
pre {
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
}

h1,
h2,
h3,
p {
    margin-top: 0;
}

h1 {
    margin-bottom: 22px;
    font-size: clamp(5.4rem, 19vw, 15rem);
    line-height: 0.78;
    letter-spacing: 0;
}

h2 {
    margin-bottom: 20px;
    font-size: clamp(2.2rem, 5.4vw, 5.7rem);
    line-height: 0.94;
    letter-spacing: 0;
}

h3 {
    margin-bottom: 12px;
    font-size: clamp(1.22rem, 2.4vw, 1.75rem);
    line-height: 1.05;
    letter-spacing: 0;
}

p {
    color: var(--muted);
    line-height: 1.68;
}

.ambient {
    position: fixed;
    inset: 0;
    z-index: -1;
    overflow: hidden;
    pointer-events: none;
}

.beam {
    position: absolute;
    display: block;
    width: 42vw;
    height: 1px;
    opacity: 0.62;
    transform-origin: center;
    animation: beam-sweep 13s linear infinite;
}

.beam-a {
    top: 18%;
    left: -12vw;
    background: linear-gradient(90deg, transparent, var(--ember), transparent);
}

.beam-b {
    top: 58%;
    right: -16vw;
    background: linear-gradient(90deg, transparent, var(--cyan), transparent);
    animation-delay: -4s;
}

.beam-c {
    bottom: 16%;
    left: 22vw;
    background: linear-gradient(90deg, transparent, var(--steel), transparent);
    animation-delay: -8s;
}

.site-header {
    position: sticky;
    top: 0;
    z-index: 20;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    min-height: 76px;
    padding: 16px clamp(20px, 5vw, 74px);
    border-bottom: 1px solid var(--line);
    background: rgba(7, 10, 11, 0.72);
    backdrop-filter: blur(22px);
}

.brand {
    display: inline-flex;
    align-items: center;
    gap: 12px;
    font-weight: 900;
    letter-spacing: 0;
}

.brand img {
    display: block;
    filter: drop-shadow(0 0 16px rgba(231, 89, 15, 0.28));
}

nav {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 8px 24px;
    color: var(--muted);
    font-size: 0.94rem;
}

nav a {
    position: relative;
}

nav a::after {
    position: absolute;
    right: 0;
    bottom: -7px;
    left: 0;
    height: 1px;
    content: "";
    background: linear-gradient(90deg, var(--ember), var(--cyan));
    transform: scaleX(0);
    transform-origin: left;
    transition: transform 180ms ease;
}

nav a:hover {
    color: var(--ink);
}

nav a:hover::after {
    transform: scaleX(1);
}

.hero {
    position: relative;
    min-height: calc(100svh - 76px);
    display: grid;
    grid-template-columns: minmax(0, 0.95fr) minmax(340px, 0.78fr);
    align-items: center;
    gap: clamp(36px, 7vw, 104px);
    padding: clamp(62px, 9vw, 124px) clamp(20px, 5vw, 74px) clamp(58px, 8vw, 92px);
}

.hero::after {
    position: absolute;
    right: clamp(20px, 5vw, 74px);
    bottom: 24px;
    left: clamp(20px, 5vw, 74px);
    height: 1px;
    content: "";
    background: linear-gradient(90deg, transparent, var(--line), transparent);
}

.hero-copy {
    max-width: 880px;
}

.eyebrow {
    margin-bottom: 16px;
    color: var(--cyan);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.78rem;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

.lede {
    max-width: 810px;
    color: #d5ddd8;
    font-size: clamp(1.22rem, 2.25vw, 1.75rem);
}

.hero-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    margin-top: 34px;
}

.button {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 48px;
    padding: 14px 19px;
    border: 1px solid rgba(255, 255, 255, 0.18);
    border-radius: 7px;
    overflow: hidden;
    font-weight: 900;
}

.button::before {
    position: absolute;
    inset: 0;
    content: "";
    background: linear-gradient(110deg, transparent, rgba(255, 255, 255, 0.2), transparent);
    transform: translateX(-120%);
}

.button:hover::before {
    animation: shimmer 740ms ease;
}

.button.primary {
    color: #07100c;
    background: linear-gradient(135deg, var(--cyan), #b7ffb1);
    box-shadow: 0 16px 46px rgba(98, 231, 211, 0.18);
}

.button.ghost {
    background: rgba(255, 255, 255, 0.045);
}

.hero-console {
    position: relative;
    isolation: isolate;
    border: 1px solid rgba(255, 255, 255, 0.16);
    border-radius: 14px;
    background:
        linear-gradient(180deg, rgba(255, 255, 255, 0.08), rgba(255, 255, 255, 0.025)),
        rgba(9, 15, 17, 0.88);
    box-shadow: 0 26px 90px var(--shadow);
    transform: perspective(1200px) rotateY(-7deg) rotateX(2deg);
    animation: console-float 7s ease-in-out infinite;
}

.hero-console::before {
    position: absolute;
    inset: -1px;
    z-index: -1;
    border-radius: inherit;
    content: "";
    background: linear-gradient(145deg, rgba(231, 89, 15, 0.54), rgba(98, 231, 211, 0.4), rgba(111, 162, 215, 0.36));
    filter: blur(24px);
    opacity: 0.45;
}

.console-top {
    display: flex;
    align-items: center;
    gap: 8px;
    min-height: 44px;
    padding: 0 16px;
    border-bottom: 1px solid var(--line);
    color: var(--muted);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.78rem;
}

.console-top span {
    width: 10px;
    height: 10px;
    border-radius: 99px;
    background: var(--ember);
}

.console-top span:nth-child(2) {
    background: var(--yellow);
}

.console-top span:nth-child(3) {
    margin-right: 7px;
    background: var(--moss);
}

.console-body {
    position: relative;
    min-height: 360px;
    padding: clamp(22px, 4vw, 34px);
    overflow: hidden;
}

.console-body::after {
    position: absolute;
    inset: 0;
    pointer-events: none;
    content: "";
    background: linear-gradient(to bottom, transparent, rgba(98, 231, 211, 0.1), transparent);
    transform: translateY(-100%);
    animation: scan 5s linear infinite;
}

.console-body p {
    margin-bottom: 14px;
    color: #dce9e1;
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: clamp(0.84rem, 1.5vw, 1rem);
}

.console-body b {
    color: var(--yellow);
}

.prompt {
    color: var(--ember-hot);
}

.ok {
    color: #a9ffbf !important;
}

.status-bars {
    display: grid;
    gap: 10px;
    margin: 22px 0;
}

.status-bars i {
    display: block;
    width: 100%;
    height: 10px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.08);
    overflow: hidden;
}

.status-bars i::before {
    display: block;
    width: var(--value);
    height: 100%;
    content: "";
    border-radius: inherit;
    background: linear-gradient(90deg, var(--ember), var(--cyan));
    animation: load-bar 2.8s ease both;
}

.console-orbit {
    position: absolute;
    right: -40px;
    bottom: -48px;
    display: grid;
    place-items: center;
    width: 170px;
    aspect-ratio: 1;
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 999px;
    background: rgba(7, 10, 11, 0.76);
    backdrop-filter: blur(18px);
    animation: orbit-pulse 4s ease-in-out infinite;
}

.console-orbit img {
    width: 108px;
    animation: mark-turn 18s linear infinite;
}

.ticker {
    border-block: 1px solid var(--line);
    overflow: hidden;
    background: rgba(255, 255, 255, 0.035);
}

.ticker div {
    display: flex;
    gap: 40px;
    width: max-content;
    padding: 16px 0;
    color: var(--muted);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.9rem;
    animation: ticker 24s linear infinite;
}

.ticker span {
    white-space: nowrap;
}

.section,
.split-lab,
.roadmap,
.deploy {
    padding: clamp(58px, 9vw, 118px) clamp(20px, 5vw, 74px);
}

.section-heading {
    max-width: 980px;
    margin-bottom: clamp(28px, 5vw, 48px);
}

.stack-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 16px;
}

.stack-card,
.feature-band article,
.phase,
.matrix-cell,
.contract-table,
.deploy pre {
    border: 1px solid var(--line);
    background: var(--panel);
    box-shadow: 0 18px 60px rgba(0, 0, 0, 0.22);
    backdrop-filter: blur(18px);
}

.stack-card {
    position: relative;
    min-height: 360px;
    padding: clamp(24px, 3.4vw, 36px);
    border-radius: 12px;
    overflow: hidden;
}

.stack-card::before {
    position: absolute;
    inset: 0 0 auto;
    height: 5px;
    content: "";
}

.stack-card.toolbox::before {
    background: var(--moss);
}

.stack-card.seam::before {
    background: var(--ember);
}

.stack-card.machine::before {
    background: var(--steel);
}

.stack-card > span,
.phase > span,
.matrix-cell > span {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 42px;
    height: 26px;
    margin-bottom: 84px;
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 999px;
    color: var(--cyan);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.75rem;
    font-weight: 900;
}

.stack-card p {
    min-height: 108px;
}

.stack-card code {
    display: block;
    padding: 12px;
    border: 1px solid var(--line);
    border-radius: 8px;
    color: var(--yellow);
    background: rgba(0, 0, 0, 0.26);
}

.split-lab {
    display: grid;
    grid-template-columns: minmax(0, 0.72fr) minmax(360px, 1fr);
    gap: clamp(30px, 7vw, 96px);
    align-items: center;
    border-block: 1px solid var(--line);
    background:
        linear-gradient(135deg, rgba(231, 89, 15, 0.12), transparent 42%),
        linear-gradient(315deg, rgba(98, 231, 211, 0.09), transparent 38%);
}

.lab-copy {
    max-width: 760px;
}

.contract-table {
    border-radius: 12px;
    overflow: hidden;
}

.contract-row {
    display: grid;
    grid-template-columns: minmax(120px, 0.7fr) minmax(88px, 0.34fr) minmax(0, 1fr);
    gap: 16px;
    align-items: center;
    min-height: 66px;
    padding: 16px 18px;
    border-bottom: 1px solid var(--line);
}

.contract-row:last-child {
    border-bottom: 0;
}

.contract-row code {
    color: var(--yellow);
}

.contract-row strong {
    color: var(--cyan);
}

.contract-row span {
    color: var(--muted);
}

.feature-band {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 16px;
}

.feature-band article {
    min-height: 280px;
    padding: 28px;
    border-radius: 12px;
}

.chip {
    display: inline-flex;
    margin-bottom: 54px;
    padding: 6px 10px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 999px;
    color: #07100c;
    background: linear-gradient(135deg, var(--ember-hot), var(--yellow));
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.74rem;
    font-weight: 900;
}

.matrix {
    border-block: 1px solid var(--line);
    background: rgba(255, 255, 255, 0.025);
}

.matrix-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 12px;
}

.matrix-cell {
    min-height: 220px;
    padding: 22px;
    border-radius: 12px;
}

.matrix-cell span {
    margin-bottom: 52px;
}

.matrix-cell strong {
    display: block;
    margin-bottom: 12px;
    line-height: 1.25;
}

.matrix-cell.pass span {
    color: #a9ffbf;
}

.matrix-cell.warn span {
    color: var(--ember-hot);
}

.roadmap {
    position: relative;
}

.phase-track {
    position: relative;
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: 14px;
}

.phase-track::before {
    position: absolute;
    top: 42px;
    right: 8%;
    left: 8%;
    height: 2px;
    content: "";
    background: linear-gradient(90deg, var(--ember), var(--cyan), var(--steel));
    opacity: 0.5;
}

.phase {
    position: relative;
    min-height: 250px;
    padding: 24px;
    border-radius: 12px;
    overflow: hidden;
}

.phase.active::after {
    position: absolute;
    top: 0;
    right: 0;
    left: 0;
    height: 3px;
    content: "";
    background: linear-gradient(90deg, var(--ember), var(--cyan));
    animation: phase-line 1.8s ease-in-out infinite alternate;
}

.phase span {
    margin-bottom: 58px;
    background: rgba(7, 10, 11, 0.84);
}

.deploy {
    display: grid;
    grid-template-columns: minmax(0, 0.8fr) minmax(340px, 0.72fr);
    gap: clamp(28px, 7vw, 92px);
    align-items: center;
    border-top: 1px solid var(--line);
    background: linear-gradient(135deg, rgba(111, 162, 215, 0.13), transparent 42%);
}

.deploy p {
    max-width: 780px;
}

.deploy pre {
    margin: 0;
    padding: clamp(22px, 4vw, 34px);
    border-radius: 12px;
    overflow: auto;
}

.deploy code {
    color: #dce9e1;
    line-height: 1.75;
}

footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 18px;
    padding: 30px clamp(20px, 5vw, 74px);
    border-top: 1px solid var(--line);
}

footer p {
    max-width: 720px;
    margin: 0;
    text-align: right;
}

@keyframes beam-sweep {
    0% {
        transform: translateX(-18vw) rotate(-14deg);
    }
    100% {
        transform: translateX(130vw) rotate(-14deg);
    }
}

@keyframes console-float {
    0%,
    100% {
        transform: perspective(1200px) rotateY(-7deg) rotateX(2deg) translateY(0);
    }
    50% {
        transform: perspective(1200px) rotateY(-4deg) rotateX(3deg) translateY(-14px);
    }
}

@keyframes scan {
    0% {
        transform: translateY(-100%);
    }
    100% {
        transform: translateY(100%);
    }
}

@keyframes load-bar {
    from {
        width: 0;
    }
}

@keyframes orbit-pulse {
    0%,
    100% {
        box-shadow: 0 0 0 0 rgba(98, 231, 211, 0.14);
    }
    50% {
        box-shadow: 0 0 0 18px rgba(98, 231, 211, 0);
    }
}

@keyframes mark-turn {
    to {
        transform: rotate(360deg);
    }
}

@keyframes ticker {
    from {
        transform: translateX(100vw);
    }
    to {
        transform: translateX(-100%);
    }
}

@keyframes shimmer {
    to {
        transform: translateX(120%);
    }
}

@keyframes phase-line {
    from {
        opacity: 0.35;
    }
    to {
        opacity: 1;
    }
}

@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        scroll-behavior: auto !important;
        animation-duration: 0.001ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.001ms !important;
    }
}

@media (max-width: 1080px) {
    .hero,
    .split-lab,
    .deploy {
        grid-template-columns: 1fr;
    }

    .hero {
        min-height: auto;
    }

    .hero-console {
        max-width: 720px;
        transform: none;
    }

    .stack-grid,
    .feature-band,
    .matrix-grid,
    .phase-track {
        grid-template-columns: 1fr;
    }

    .phase-track::before {
        display: none;
    }
}

@media (max-width: 720px) {
    .site-header {
        position: static;
        align-items: flex-start;
        flex-direction: column;
    }

    nav {
        justify-content: flex-start;
        gap: 10px 18px;
    }

    h1 {
        font-size: clamp(4.6rem, 24vw, 8rem);
    }

    .button {
        width: 100%;
    }

    .console-orbit {
        right: 16px;
        bottom: -56px;
        width: 124px;
    }

    .console-orbit img {
        width: 82px;
    }

    .contract-row {
        grid-template-columns: 1fr;
        gap: 7px;
    }

    footer {
        align-items: flex-start;
        flex-direction: column;
    }

    footer p {
        text-align: left;
    }
}
"#
}

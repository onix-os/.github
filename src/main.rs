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
        ("/usr", "moss", "stateless machine payload"),
        ("/.moss", "moss", "content store and transaction history"),
        ("/boot", "moss", "kernel, initrd, BLS entries"),
        (
            "/etc/nix",
            "onix-nix-integration",
            "declared defaults, no installer drift",
        ),
        ("/nix", "Nix", "persistent store, daemon, profiles"),
        (
            "/run/opengl-driver",
            "ONIX seam",
            "host graphics bridge for Nix apps",
        ),
    ];

    let roadmap = [
        (
            "0",
            "Forge",
            "moss and boulder running on the Alpine musl quarry",
        ),
        (
            "1",
            "Base",
            "first self-owned musl stone set and local repo",
        ),
        ("2", "Image", "bootable ONIX VM with moss state rollback"),
        (
            "3",
            "Nix",
            "multi-user Nix plane with independent rollback tests",
        ),
        (
            "5",
            "Desktop",
            "Wayland, Mesa, portals, and the OpenGL bridge",
        ),
    ];

    let contract = contract_rows
        .iter()
        .map(|(path, owner, note)| {
            format!(
                r#"<tr>
                    <td><code>{path}</code></td>
                    <td>{owner}</td>
                    <td>{note}</td>
                </tr>"#
            )
        })
        .collect::<String>();

    let roadmap_items = roadmap
        .iter()
        .map(|(num, title, body)| {
            format!(
                r#"<article class="phase">
                    <span>phase {num}</span>
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
    <header class="topbar">
        <a class="brand" href="#top" aria-label="ONIX home">
            <img src="assets/onix.svg" alt="" width="34" height="34">
            <span>ONIX</span>
        </a>
        <nav aria-label="Primary">
            <a href="#architecture">Architecture</a>
            <a href="#contract">Contract</a>
            <a href="#validation">Validation</a>
            <a href="#roadmap">Roadmap</a>
        </nav>
    </header>

    <main id="top">
        <section class="hero">
            <div class="hero-copy">
                <p class="kicker">atomic musl base / persistent Nix toolbox</p>
                <h1>ONIX</h1>
                <p class="lead">A small operating system built from scratch where moss owns the machine and Nix owns the toolbox.</p>
                <p class="summary">The base is musl, transactional, and auditable. The software long tail lives above it in a persistent multi-user Nix plane. The point is not novelty for its own sake; the point is a machine that can move forward and roll back without dragging your working environment through the blast radius.</p>
                <div class="actions">
                    <a class="button primary" href="#architecture">Read the model</a>
                    <a class="button secondary" href="#contract">Inspect ownership</a>
                </div>
            </div>
            <aside class="identity-panel" aria-label="ONIX identity">
                <img src="assets/onix.svg" alt="ONIX logo">
                <dl>
                    <div>
                        <dt>libc</dt>
                        <dd>musl</dd>
                    </div>
                    <div>
                        <dt>machine</dt>
                        <dd>moss</dd>
                    </div>
                    <div>
                        <dt>toolbox</dt>
                        <dd>Nix</dd>
                    </div>
                    <div>
                        <dt>magic</dt>
                        <dd>6649</dd>
                    </div>
                </dl>
            </aside>
        </section>

        <section class="rule">
            <span>core rule</span>
            <strong>moss controls the machine. Nix controls the toolbox.</strong>
        </section>

        <section class="section" id="architecture">
            <div class="section-title">
                <p class="kicker">Architecture</p>
                <h2>Two planes and a deliberately narrow seam.</h2>
            </div>
            <div class="planes">
                <article>
                    <span class="label blue">machine plane</span>
                    <h3>Atomic state</h3>
                    <p>moss owns <code>/usr</code>, the kernel, initrd, boot entries, firmware, Mesa, PipeWire, portals, and the compositor. It is the hard layer.</p>
                </article>
                <article>
                    <span class="label orange">integration seam</span>
                    <h3>Declared glue</h3>
                    <p><code>onix-nix-integration</code> seeds nix-daemon, nixbld users, defaults, shell hooks, runtime dirs, and graphics bridge state.</p>
                </article>
                <article>
                    <span class="label blue">toolbox plane</span>
                    <h3>Persistent work</h3>
                    <p>Nix owns <code>/nix</code>, user profiles, dev shells, flakes, language stacks, and GUI leaf apps. It is the living workspace.</p>
                </article>
            </div>
        </section>

        <section class="section split" id="contract">
            <div>
                <p class="kicker">Ownership contract</p>
                <h2>No surface has two owners.</h2>
                <p>ONIX should make ownership boring and visible. If a rollback happens, you should know exactly which plane moved and which one stayed still.</p>
            </div>
            <div class="table-wrap">
                <table>
                    <thead>
                        <tr>
                            <th>Surface</th>
                            <th>Owner</th>
                            <th>Reason</th>
                        </tr>
                    </thead>
                    <tbody>
                        {contract}
                    </tbody>
                </table>
            </div>
        </section>

        <section class="section developer">
            <div class="manual-card">
                <p class="kicker">CLI shape</p>
                <pre><code>$ onix status
active fstx: 6649-a17c
boot entry:  onix-6649-a17c.conf
nix daemon:  healthy
etc drift:   2 local overrides
opengl:      coherent

$ onix rollback
plane:       machine
/nix:        untouched</code></pre>
            </div>
            <div class="notes">
                <article>
                    <h3>Alpine is the forge</h3>
                    <p>The quarry host is scaffolding: build moss, boulder, and the first stones there, then discard it.</p>
                </article>
                <article>
                    <h3>The base stays short</h3>
                    <p>Busybox first, uutils after proof, and only the essentials. Nix covers the long tail.</p>
                </article>
                <article>
                    <h3>Graphics is a system boundary</h3>
                    <p><code>/run/opengl-driver</code> is the bridge where Nix GUI apps meet the active machine stack.</p>
                </article>
            </div>
        </section>

        <section class="section" id="validation">
            <div class="section-title">
                <p class="kicker">Validation</p>
                <h2>The composition matrix is the real release gate.</h2>
            </div>
            <div class="checks">
                <article>
                    <strong>01</strong>
                    <h3>Nix tool survives reboot</h3>
                    <p><code>nix profile install nixpkgs#ripgrep</code>, reboot, and confirm it remains on PATH.</p>
                </article>
                <article>
                    <strong>02</strong>
                    <h3>moss rollback leaves Nix alone</h3>
                    <p>Roll machine state back and verify profiles, store, and daemon remain consistent.</p>
                </article>
                <article>
                    <strong>03</strong>
                    <h3>GC boundaries hold</h3>
                    <p><code>moss state prune</code> and <code>nix store gc</code> run back-to-back without cross-corruption.</p>
                </article>
                <article>
                    <strong>04</strong>
                    <h3>OpenGL bridge coheres</h3>
                    <p>Rollback Mesa and confirm Nix GUI apps render against the previous active stack.</p>
                </article>
            </div>
        </section>

        <section class="section roadmap" id="roadmap">
            <div class="section-title">
                <p class="kicker">Roadmap</p>
                <h2>Small gates. Real exits.</h2>
            </div>
            <div class="phases">
                {roadmap_items}
            </div>
        </section>

        <section class="deploy">
            <div>
                <p class="kicker">Website</p>
                <h2>Static by construction.</h2>
                <p>The Rust generator emits plain files to <code>dist/</code>. GitHub Actions publishes that directory to <code>gh-pages</code> with <code>CNAME</code> set to <code>onix-os.com</code>.</p>
            </div>
            <pre><code>nix develop
just build
just serve</code></pre>
        </section>
    </main>

    <footer>
        <a class="brand" href="#top" aria-label="ONIX home">
            <img src="assets/onix.svg" alt="" width="28" height="28">
            <span>ONIX</span>
        </a>
        <p>Atomic musl base. Persistent Nix toolbox. Built with moss and boulder.</p>
    </footer>
</body>
</html>
"##
    )
}

fn css() -> &'static str {
    r#":root {
    color-scheme: light;
    --paper: #f5f2ed;
    --paper-2: #ebe4da;
    --ink: #171a1c;
    --muted: #5f6669;
    --line: #d6ccc0;
    --blue: #4f6e91;
    --blue-2: #2d425c;
    --blue-3: #dbe5ee;
    --orange: #e7590f;
    --orange-2: #9e3b0d;
    --orange-3: #f7dccb;
    --white: #fffaf3;
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
        linear-gradient(90deg, rgba(79, 110, 145, 0.08) 1px, transparent 1px),
        linear-gradient(rgba(79, 110, 145, 0.07) 1px, transparent 1px),
        var(--paper);
    background-size: 36px 36px;
    font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}

a {
    color: inherit;
    text-decoration: none;
}

code,
pre,
.kicker,
table {
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
}

h1,
h2,
h3,
p {
    margin-top: 0;
}

h1 {
    margin-bottom: 24px;
    color: var(--blue-2);
    font-size: clamp(5.2rem, 18vw, 14rem);
    line-height: 0.78;
    letter-spacing: 0;
}

h2 {
    margin-bottom: 18px;
    color: var(--blue-2);
    font-size: clamp(2.2rem, 5vw, 5rem);
    line-height: 0.96;
    letter-spacing: 0;
}

h3 {
    margin-bottom: 10px;
    color: var(--ink);
    font-size: clamp(1.2rem, 2vw, 1.55rem);
    line-height: 1.12;
    letter-spacing: 0;
}

p {
    color: var(--muted);
    line-height: 1.68;
}

.topbar {
    position: sticky;
    top: 0;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 22px;
    min-height: 72px;
    padding: 16px clamp(20px, 5vw, 72px);
    border-bottom: 2px solid var(--blue-2);
    background: rgba(245, 242, 237, 0.94);
    backdrop-filter: blur(14px);
}

.brand {
    display: inline-flex;
    align-items: center;
    gap: 11px;
    color: var(--blue-2);
    font-weight: 900;
}

.brand img {
    display: block;
}

nav {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 8px 22px;
    color: var(--blue-2);
    font-size: 0.95rem;
    font-weight: 700;
}

nav a {
    border-bottom: 2px solid transparent;
}

nav a:hover {
    border-color: var(--orange);
}

.hero {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(300px, 0.52fr);
    gap: clamp(32px, 7vw, 90px);
    align-items: stretch;
    padding: clamp(54px, 8vw, 104px) clamp(20px, 5vw, 72px);
}

.hero-copy {
    padding: clamp(18px, 3vw, 30px) 0;
}

.kicker {
    margin-bottom: 14px;
    color: var(--orange-2);
    font-size: 0.78rem;
    font-weight: 900;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

.lead {
    max-width: 820px;
    color: var(--ink);
    font-size: clamp(1.45rem, 2.8vw, 2.3rem);
    line-height: 1.22;
}

.summary {
    max-width: 780px;
    font-size: 1.06rem;
}

.actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    margin-top: 30px;
}

.button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 46px;
    padding: 12px 17px;
    border: 2px solid var(--blue-2);
    border-radius: 4px;
    font-weight: 900;
}

.button.primary {
    color: var(--white);
    background: var(--blue-2);
}

.button.secondary {
    color: var(--blue-2);
    background: var(--orange-3);
}

.identity-panel {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    min-height: 560px;
    padding: clamp(24px, 4vw, 38px);
    border: 2px solid var(--blue-2);
    border-radius: 8px;
    background:
        linear-gradient(135deg, var(--blue-3), transparent 55%),
        var(--white);
    box-shadow: 12px 12px 0 var(--orange);
}

.identity-panel img {
    width: min(100%, 360px);
    align-self: center;
}

.identity-panel dl {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1px;
    margin: 34px 0 0;
    border: 1px solid var(--blue-2);
    background: var(--blue-2);
}

.identity-panel div {
    padding: 14px;
    background: var(--white);
}

.identity-panel dt {
    color: var(--orange-2);
    font-size: 0.76rem;
    font-weight: 900;
    text-transform: uppercase;
}

.identity-panel dd {
    margin: 4px 0 0;
    color: var(--blue-2);
    font-size: 1.15rem;
    font-weight: 900;
}

.rule {
    display: grid;
    grid-template-columns: minmax(130px, 0.22fr) minmax(0, 1fr);
    gap: 24px;
    align-items: baseline;
    padding: clamp(30px, 5vw, 54px) clamp(20px, 5vw, 72px);
    color: var(--white);
    background: var(--blue-2);
}

.rule span {
    color: var(--orange-3);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.8rem;
    font-weight: 900;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

.rule strong {
    font-size: clamp(1.8rem, 4.6vw, 4.8rem);
    line-height: 1;
}

.section,
.deploy {
    padding: clamp(54px, 8vw, 104px) clamp(20px, 5vw, 72px);
}

.section-title {
    max-width: 980px;
    margin-bottom: 34px;
}

.planes {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 16px;
}

.planes article,
.checks article,
.phase,
.notes article,
.manual-card,
.table-wrap,
.deploy pre {
    border: 2px solid var(--blue-2);
    border-radius: 8px;
    background: var(--white);
}

.planes article {
    min-height: 320px;
    padding: 24px;
}

.label {
    display: inline-flex;
    margin-bottom: 64px;
    padding: 6px 9px;
    border-radius: 3px;
    color: var(--white);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.74rem;
    font-weight: 900;
    text-transform: uppercase;
}

.label.blue {
    background: var(--blue);
}

.label.orange {
    background: var(--orange);
}

.split {
    display: grid;
    grid-template-columns: minmax(0, 0.7fr) minmax(360px, 1fr);
    gap: clamp(28px, 6vw, 82px);
    align-items: start;
    border-block: 2px solid var(--line);
    background: rgba(255, 250, 243, 0.52);
}

.table-wrap {
    overflow: auto;
}

table {
    width: 100%;
    border-collapse: collapse;
    min-width: 640px;
    font-size: 0.9rem;
}

th,
td {
    padding: 16px;
    border-bottom: 1px solid var(--line);
    text-align: left;
    vertical-align: top;
}

th {
    color: var(--white);
    background: var(--blue-2);
}

td:first-child code {
    color: var(--orange-2);
    font-weight: 900;
}

tr:last-child td {
    border-bottom: 0;
}

.developer {
    display: grid;
    grid-template-columns: minmax(320px, 0.85fr) minmax(0, 1fr);
    gap: 16px;
}

.manual-card {
    padding: 24px;
    background: var(--blue-2);
}

.manual-card .kicker {
    color: var(--orange-3);
}

pre {
    margin: 0;
    overflow: auto;
}

pre code {
    color: #f7efe5;
    line-height: 1.7;
}

.notes {
    display: grid;
    gap: 16px;
}

.notes article {
    padding: 22px;
    border-color: var(--orange-2);
}

.checks {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 16px;
}

.checks article {
    min-height: 260px;
    padding: 22px;
}

.checks strong {
    display: inline-flex;
    margin-bottom: 54px;
    color: var(--orange);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 1.05rem;
}

.roadmap {
    background: linear-gradient(180deg, transparent, rgba(79, 110, 145, 0.08));
}

.phases {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: 12px;
}

.phase {
    min-height: 230px;
    padding: 20px;
}

.phase span {
    display: inline-flex;
    margin-bottom: 52px;
    color: var(--orange-2);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.76rem;
    font-weight: 900;
    text-transform: uppercase;
}

.deploy {
    display: grid;
    grid-template-columns: minmax(0, 0.8fr) minmax(280px, 0.5fr);
    gap: clamp(24px, 6vw, 72px);
    align-items: center;
    border-top: 2px solid var(--blue-2);
    background: var(--paper-2);
}

.deploy p {
    max-width: 760px;
}

.deploy pre {
    padding: 22px;
    background: var(--blue-2);
}

footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 18px;
    padding: 28px clamp(20px, 5vw, 72px);
    border-top: 2px solid var(--blue-2);
    background: var(--white);
}

footer p {
    max-width: 680px;
    margin: 0;
    text-align: right;
}

@media (max-width: 1080px) {
    .hero,
    .split,
    .developer,
    .deploy {
        grid-template-columns: 1fr;
    }

    .planes,
    .checks,
    .phases {
        grid-template-columns: 1fr;
    }

    .identity-panel {
        min-height: 0;
    }
}

@media (max-width: 720px) {
    .topbar {
        position: static;
        align-items: flex-start;
        flex-direction: column;
    }

    nav {
        justify-content: flex-start;
    }

    h1 {
        font-size: clamp(4.4rem, 24vw, 8rem);
    }

    .rule {
        grid-template-columns: 1fr;
    }

    .button {
        width: 100%;
    }

    .identity-panel {
        box-shadow: 7px 7px 0 var(--orange);
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

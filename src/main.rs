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
    let phases = [
        (
            "00",
            "Forge",
            "Build moss and boulder on a tiny Alpine musl host.",
        ),
        (
            "01",
            "Base Stones",
            "Cut the smallest musl userland into self-owned .stone packages.",
        ),
        (
            "02",
            "First Boot",
            "Boot a moss-managed ONIX image with atomic state history.",
        ),
        (
            "03",
            "Nix Plane",
            "Prove Nix and moss rollbacks do not corrupt each other.",
        ),
        (
            "05",
            "Desktop",
            "Make Nix GUI apps first-class through /run/opengl-driver.",
        ),
    ];

    let phase_cards = phases
        .iter()
        .map(|(num, title, body)| {
            format!(
                r#"<article class="phase">
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
    <header class="site-header">
        <a class="brand" href="#top" aria-label="ONIX home">
            <img src="assets/onix.svg" alt="" width="36" height="36">
            <span>ONIX</span>
        </a>
        <nav aria-label="Primary">
            <a href="#architecture">Architecture</a>
            <a href="#roadmap">Roadmap</a>
            <a href="#nix">Nix seam</a>
            <a href="#status">Status</a>
        </nav>
    </header>

    <main id="top">
        <section class="hero">
            <div class="hero-copy">
                <p class="eyebrow">Atomic musl base &middot; moss-managed machine &middot; persistent Nix toolbox</p>
                <h1>ONIX</h1>
                <p class="lede">A small, auditable operating system built from scratch: moss controls the machine, Nix controls the toolbox, and both planes roll back without stepping on each other.</p>
                <div class="hero-actions" aria-label="Project focus">
                    <a class="button primary" href="#architecture">See the architecture</a>
                    <a class="button secondary" href="#roadmap">Track the build plan</a>
                </div>
            </div>
            <div class="hero-mark" aria-hidden="true">
                <img src="assets/onix.svg" alt="">
            </div>
        </section>

        <section class="principle band">
            <p>Core rule</p>
            <strong>moss controls the machine. Nix controls the toolbox.</strong>
        </section>

        <section class="section" id="architecture">
            <div class="section-heading">
                <p class="eyebrow">Architecture</p>
                <h2>Two planes, one hard ownership contract.</h2>
            </div>
            <div class="planes">
                <article class="plane toolbox">
                    <span>User toolbox plane</span>
                    <h3>/nix stays persistent</h3>
                    <p>Per-user profiles, dev shells, flakes, GUI leaf apps, and the long tail come from Nix. Nix carries the glibc world without owning the OS.</p>
                </article>
                <article class="plane seam">
                    <span>Integration seam</span>
                    <h3>Declared by onix-* stones</h3>
                    <p>nix-daemon, nixbld users, defaults, font and locale glue, and the OpenGL bridge are shipped by the machine plane, not hand-installed.</p>
                </article>
                <article class="plane machine">
                    <span>Machine plane</span>
                    <h3>/usr is moss-owned</h3>
                    <p>The musl base, kernel, initrd, boot entries, Mesa, PipeWire, portals, and compositor are transactional state managed by moss.</p>
                </article>
            </div>
        </section>

        <section class="section split" id="nix">
            <div>
                <p class="eyebrow">Distinguishing feature</p>
                <h2>Nix GUI apps get a host-matched graphics bridge.</h2>
                <p>ONIX treats <code>/run/opengl-driver</code> as first-class OS infrastructure. The active moss state exports the graphics stack Nix apps should see, so a moss rollback can re-cohere the GUI world on the next boot.</p>
            </div>
            <div class="terminal" aria-label="ONIX command examples">
                <p><span>$</span> onix status</p>
                <p>fstx active: 6649-a17c</p>
                <p>nix-daemon: healthy</p>
                <p>opengl-driver: coherent</p>
                <p><span>$</span> onix rollback</p>
                <p>machine plane rolls back; /nix is untouched</p>
            </div>
        </section>

        <section class="section" id="roadmap">
            <div class="section-heading">
                <p class="eyebrow">Roadmap</p>
                <h2>Phase gates over vibes.</h2>
            </div>
            <div class="phase-grid">
                {phase_cards}
            </div>
        </section>

        <section class="matrix band" id="status">
            <div>
                <p class="eyebrow">Composition matrix</p>
                <h2>Success means independent rollback.</h2>
            </div>
            <dl>
                <div>
                    <dt>Nix tool + reboot</dt>
                    <dd>Still on PATH, still running on the musl base.</dd>
                </div>
                <div>
                    <dt>Moss rollback</dt>
                    <dd>Machine state rolls back; the Nix profile remains intact.</dd>
                </div>
                <div>
                    <dt>Garbage collection</dt>
                    <dd>moss prune and nix store gc never cross-corrupt.</dd>
                </div>
            </dl>
        </section>
    </main>

    <footer>
        <img src="assets/onix.svg" alt="" width="28" height="28">
        <p>ONIX is a scratch-built musl OS project using moss and boulder tooling with a persistent Nix toolbox.</p>
    </footer>
</body>
</html>
"##
    )
}

fn css() -> &'static str {
    r#":root {
    color-scheme: light;
    --paper: #f7f8f6;
    --ink: #111411;
    --muted: #5d6664;
    --line: #d9ded9;
    --steel: #4f6e91;
    --steel-dark: #263d56;
    --ember: #e7590f;
    --moss: #52684d;
    --white: #ffffff;
}

* {
    box-sizing: border-box;
}

html {
    scroll-behavior: smooth;
}

body {
    margin: 0;
    font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    color: var(--ink);
    background: var(--paper);
}

a {
    color: inherit;
    text-decoration: none;
}

code {
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    font-size: 0.94em;
    color: var(--steel-dark);
}

.site-header {
    position: sticky;
    top: 0;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    min-height: 72px;
    padding: 16px clamp(20px, 5vw, 72px);
    border-bottom: 1px solid rgba(17, 20, 17, 0.1);
    background: rgba(247, 248, 246, 0.92);
    backdrop-filter: blur(18px);
}

.brand {
    display: inline-flex;
    align-items: center;
    gap: 11px;
    font-weight: 800;
    letter-spacing: 0;
}

.brand img {
    display: block;
}

nav {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 8px 22px;
    color: var(--muted);
    font-size: 0.95rem;
}

nav a:hover {
    color: var(--ink);
}

.hero {
    min-height: calc(100svh - 72px);
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(280px, 0.78fr);
    align-items: center;
    gap: clamp(28px, 7vw, 96px);
    padding: clamp(54px, 9vw, 116px) clamp(20px, 5vw, 72px) clamp(42px, 7vw, 80px);
    border-bottom: 1px solid var(--line);
}

.hero-copy {
    max-width: 820px;
}

.eyebrow {
    margin: 0 0 14px;
    color: var(--ember);
    font-size: 0.78rem;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

h1,
h2,
h3,
p {
    margin-top: 0;
}

h1 {
    margin-bottom: 18px;
    font-size: clamp(5rem, 18vw, 13rem);
    line-height: 0.82;
    letter-spacing: 0;
}

h2 {
    margin-bottom: 18px;
    font-size: clamp(2rem, 4.8vw, 4.7rem);
    line-height: 0.98;
    letter-spacing: 0;
}

h3 {
    margin-bottom: 10px;
    font-size: clamp(1.12rem, 2vw, 1.45rem);
    line-height: 1.12;
}

.lede {
    max-width: 780px;
    color: #38413f;
    font-size: clamp(1.2rem, 2.2vw, 1.65rem);
    line-height: 1.45;
}

.hero-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    margin-top: 34px;
}

.button {
    display: inline-flex;
    align-items: center;
    min-height: 46px;
    padding: 13px 18px;
    border: 1px solid var(--ink);
    border-radius: 6px;
    font-weight: 800;
}

.button.primary {
    color: var(--white);
    background: var(--ink);
}

.button.secondary {
    color: var(--ink);
    background: transparent;
}

.button:hover {
    transform: translateY(-1px);
}

.hero-mark {
    display: grid;
    place-items: center;
}

.hero-mark img {
    width: min(100%, 560px);
    aspect-ratio: 1;
    filter: drop-shadow(0 22px 34px rgba(17, 20, 17, 0.16));
}

.band,
.section {
    padding: clamp(48px, 8vw, 104px) clamp(20px, 5vw, 72px);
}

.principle {
    display: grid;
    grid-template-columns: minmax(120px, 0.26fr) minmax(0, 1fr);
    gap: 24px;
    align-items: baseline;
    color: var(--white);
    background: var(--ink);
}

.principle p {
    margin: 0;
    color: #aab1ad;
    font-size: 0.88rem;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

.principle strong {
    font-size: clamp(1.8rem, 5vw, 5.4rem);
    line-height: 1;
    letter-spacing: 0;
}

.section-heading {
    max-width: 900px;
    margin-bottom: 34px;
}

.planes {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    border: 1px solid var(--line);
    background: var(--white);
}

.plane {
    min-height: 300px;
    padding: clamp(22px, 3vw, 34px);
    border-right: 1px solid var(--line);
}

.plane:last-child {
    border-right: 0;
}

.plane span,
.phase span {
    display: inline-block;
    margin-bottom: 58px;
    color: var(--muted);
    font-size: 0.78rem;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
}

.plane p,
.phase p,
.split p,
.matrix dd,
footer p {
    color: var(--muted);
    line-height: 1.65;
}

.toolbox {
    border-top: 7px solid var(--moss);
}

.seam {
    border-top: 7px solid var(--ember);
}

.machine {
    border-top: 7px solid var(--steel);
}

.split {
    display: grid;
    grid-template-columns: minmax(0, 0.9fr) minmax(300px, 0.7fr);
    gap: clamp(28px, 6vw, 88px);
    align-items: center;
    border-top: 1px solid var(--line);
    border-bottom: 1px solid var(--line);
    background: #eef2f0;
}

.split > div:first-child {
    max-width: 760px;
}

.split > div:first-child p:last-child {
    font-size: 1.08rem;
}

.terminal {
    padding: clamp(22px, 3vw, 32px);
    border-radius: 8px;
    color: #dce9dd;
    background: #111411;
    box-shadow: 0 18px 42px rgba(17, 20, 17, 0.22);
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
    overflow: auto;
}

.terminal p {
    margin: 0 0 10px;
    color: inherit;
    white-space: nowrap;
}

.terminal p:last-child {
    margin-bottom: 0;
}

.terminal span {
    color: var(--ember);
}

.phase-grid {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    border: 1px solid var(--line);
    background: var(--white);
}

.phase {
    min-height: 260px;
    padding: 24px;
    border-right: 1px solid var(--line);
}

.phase:last-child {
    border-right: 0;
}

.phase span {
    margin-bottom: 46px;
    color: var(--steel);
}

.matrix {
    display: grid;
    grid-template-columns: minmax(0, 0.72fr) minmax(300px, 1fr);
    gap: clamp(28px, 6vw, 80px);
    color: var(--white);
    background: var(--steel-dark);
}

.matrix .eyebrow {
    color: #ff8a45;
}

.matrix dl {
    display: grid;
    gap: 1px;
    margin: 0;
    background: rgba(255, 255, 255, 0.22);
    border: 1px solid rgba(255, 255, 255, 0.22);
}

.matrix dl div {
    padding: 22px;
    background: var(--steel-dark);
}

.matrix dt {
    margin-bottom: 6px;
    font-weight: 800;
}

.matrix dd {
    margin: 0;
    color: #d7e0e5;
}

footer {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 26px clamp(20px, 5vw, 72px);
    border-top: 1px solid var(--line);
}

footer p {
    margin: 0;
    font-size: 0.92rem;
}

@media (max-width: 980px) {
    .hero,
    .split,
    .matrix {
        grid-template-columns: 1fr;
    }

    .hero {
        min-height: auto;
    }

    .hero-mark {
        order: -1;
        justify-content: start;
    }

    .hero-mark img {
        width: min(68vw, 320px);
    }

    .planes,
    .phase-grid {
        grid-template-columns: 1fr;
    }

    .plane,
    .phase {
        min-height: 0;
        border-right: 0;
        border-bottom: 1px solid var(--line);
    }

    .plane:last-child,
    .phase:last-child {
        border-bottom: 0;
    }

    .plane span,
    .phase span {
        margin-bottom: 26px;
    }
}

@media (max-width: 680px) {
    .site-header {
        position: static;
        align-items: flex-start;
        flex-direction: column;
    }

    nav {
        justify-content: flex-start;
    }

    .principle {
        grid-template-columns: 1fr;
    }

    .button {
        width: 100%;
        justify-content: center;
    }

    .terminal {
        font-size: 0.84rem;
    }
}
"#
}

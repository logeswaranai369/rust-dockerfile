# Sample 04 — Rust + Dockerfile (State A: everything detected)

Rust app using the Dockerfile escape hatch. **Exercises wizard State A** — the Dockerfile has a single `EXPOSE 8000`, so the parser fills everything in and the customer just clicks Continue.

## What this exercises

| Step | What happens |
|---|---|
| Detection | `runtime_detection` sees `Dockerfile` → matches `docker` runtime |
| Dockerfile parse | Parser finds `FROM alpine:3.18`, `EXPOSE 8000`, `CMD ["/usr/local/bin/app"]`, `USER app` |
| Wizard state | **State A** (1 EXPOSE port) — green banner "all configuration parsed (high confidence)" |
| Input fields | **None** — customer just clicks Continue |
| Build | `kaniko` Job runs the multi-stage Dockerfile — first stage compiles, second stage ships only the binary |
| Deploy | Harbor digest → K8s Deployment → public URL serves on port 8000 |

## What to verify

1. **Wizard Step 3 shows zero input fields** — that's the entire point of State A.
2. The "Configuration parsed from your Dockerfile" panel shows 5 green ✓ cards (base image: `alpine:3.18`, port: 3000, build steps: 4 RUN commands, start command: `CMD [...]`, runtime user: `app`).
3. Site goes Live within 3–5 minutes (Rust compile takes the longest).
4. Visit the site URL → pink "Built with kaniko" badge.
5. Final image is < 20 MB (Alpine + static binary).

## Files

* `Cargo.toml` — Rust crate manifest, actix-web 4 dep.
* `src/main.rs` — actix-web server. Reads `$PORT` env var, defaults to 8000 matching the Dockerfile's EXPOSE.
* `Dockerfile` — multi-stage: rust:1.74-alpine builds, alpine:3.18 runs. EXPOSE 8000 + CMD + USER all present so the parser detects everything.

## To use

1. Push this folder as a public GitHub repo.
2. `/sites/create` → Dynamic → connect this repo.
3. Detection lands on **State A** — the wizard says "Custom Dockerfile detected — all configuration parsed".
4. Hit Continue (no input needed).
5. Confirm step shows: runtime = Custom Dockerfile, port = 8000.
6. Hit Deploy and watch the kaniko build (slow — Rust compile from scratch ~3 min).

## How to convert this to State B (testing the "missing EXPOSE" flow)

Delete the `EXPOSE 8000` line from the Dockerfile, push, retry. The wizard will land on **State B** — the customer must type the port (8000 in this case, since that's what the app listens on).

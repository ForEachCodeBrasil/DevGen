# Supply-chain security

DevGen is a desktop app with a relatively small dependency surface (Vue + Tauri + Rust). This document records the practices we use to reduce npm/Cargo supply-chain risk.

## Principles

1. **Lockfiles are source of truth** — always commit `package-lock.json` and `Cargo.lock`.
2. **CI installs exactly the lockfile** — use `npm ci`, never bare `npm install` in pipelines.
3. **Prefer known-good minors/patches** — avoid surprise major upgrades without a review.
4. **Audit regularly** — known CVEs are a baseline, not the full threat model.
5. **Minimize blast radius** — fewer deps, fewer install scripts, fewer privileged CI tokens.

## npm defenses in this repo

| Control | Where |
|---------|--------|
| Committed lockfile with integrity hashes | `package-lock.json` |
| Registry pinned to official npm | `.npmrc` (`registry=https://registry.npmjs.org/`) |
| Audit on install (moderate+) | `.npmrc` (`audit=true`, `audit-level=moderate`) |
| `npm ci` in CI/release | `.github/workflows/ci.yml`, `release-macos.yml` |
| `npm audit --audit-level=high` in CI | `.github/workflows/ci.yml` |
| Dependabot weekly PRs (grouped) | `.github/dependabot.yml` |
| Major upgrades blocked for critical tooling | Dependabot `ignore` rules |

### Commands

```bash
# Reproducible install (preferred for local clean installs / CI)
npm ci

# After intentionally changing package.json
npm install
npm audit

# Security-focused refresh within semver ranges
npm update
npm audit fix
```

### What we deliberately do **not** auto-bump

- `vite` major (7/8)
- `typescript` major (6/7)
- `vue-tsc` major
- `@vitejs/plugin-vue` major
- `lucide-vue-next` major

Those need a short migration pass (build, typecheck, manual smoke).

## Cargo / Rust defenses

| Control | Notes |
|---------|--------|
| Committed `Cargo.lock` | Exact crate graph for reproducible builds |
| Prefer crates.io only | No git/path deps for third-party code in production path |
| `cargo update` for patches | Review `Cargo.lock` diff before commit |
| Optional: `cargo audit` | Install with `cargo install cargo-audit` and run in `src-tauri` |

```bash
cd src-tauri
cargo update
cargo test
cargo clippy --all-targets -- -D warnings
# optional:
cargo install cargo-audit
cargo audit
```

## Threat model (practical)

Recent npm campaigns often abuse:

1. **Compromised maintainer account** shipping a malicious patch under a trusted name.
2. **Lifecycle scripts** (`preinstall` / `postinstall`) exfiltrating tokens.
3. **Lockfile injection** / registry redirects when lockfiles are not enforced.
4. **Fresh malware versions** published minutes before install (no cooldown).

Mitigations that matter for this project:

- Always review Dependabot / lockfile diffs (new package names, unexpected maintainers, huge tarballs).
- Prefer `npm ci` so floating ranges cannot silently re-resolve.
- Keep GitHub Actions and npm tokens minimal (least privilege; no long-lived publish tokens in this app).
- Do not commit secrets (`.env` is gitignored; use `.env.example` only).
- For higher assurance installs: `npm ci --ignore-scripts` then run only known-needed scripts (can break packages that require native postinstall; validate before adopting project-wide).

## Incident checklist

If a dependency is reported compromised:

1. Identify versions in `package-lock.json` / `Cargo.lock`.
2. Upgrade or remove the package; regenerate lockfiles.
3. Rotate any credentials that touched machines/CI during the exposure window.
4. Re-run `npm ci`, `npm audit`, frontend tests, `cargo test`, and a local Tauri smoke build.
5. Publish a patched app build if a release was cut from a poisoned tree.

## References

- npm lockfiles & `npm ci`
- npm provenance / `npm audit signatures` (optional deeper checks)
- GitHub Dependabot version updates
- RustSec / `cargo audit`

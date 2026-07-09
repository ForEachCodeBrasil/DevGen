# DevGen

DevGen is a macOS-first Tauri + Vue desktop app for offline fake data generation (documents, people, companies, vehicles, utilities).

## Requirements

- Node.js 22+
- npm 10+
- Rust stable toolchain
- Xcode Command Line Tools (macOS)

## Development

```bash
npm install
npm run tauri dev
```

## Quality Commands

```bash
npm ci
npm audit
npm run typecheck
npm test -- --run
npm run build
cd src-tauri && cargo fmt --check
cd src-tauri && cargo clippy --all-targets -- -D warnings
cd src-tauri && cargo test
```

## Supply-chain security

- Always use the committed lockfiles (`package-lock.json`, `src-tauri/Cargo.lock`).
- Prefer `npm ci` over `npm install` for clean installs and CI.
- Dependabot opens weekly PRs for npm, Cargo, and GitHub Actions.
- Details and incident checklist: [`docs/supply-chain.md`](docs/supply-chain.md).

## Production Build

```bash
npm run tauri build
```

Generated artifacts:

- `src-tauri/target/release/bundle/macos/DevGen.app`
- `src-tauri/target/release/bundle/dmg/DevGen_<version>_aarch64.dmg`

## CI / Release

- CI workflow: `.github/workflows/ci.yml`
- macOS release workflow: `.github/workflows/release-macos.yml`
- Signing and notarization checklist: `docs/release-macos-checklist.md`

## Lycento License Configuration

Set these environment variables before running/building the desktop app:

- `LYCENTO_BASE_URL` (optional override, defaults to `https://lycento.test` in local/dev builds and `https://lycento.tech` in release builds)
- `LYCENTO_API_KEY` (optional, for authenticated requests)
- `LYCENTO_CHECKOUT_URL` (optional override for the purchase button)
- `LYCENTO_GRACE_DAYS` (optional, default: `7`)

Create a `.env` file in the project root with these variables. See `.env.example` for reference.

Environment precedence:

1. Explicit environment variables win.
2. If `LYCENTO_BASE_URL` is not set, local/dev builds use `https://lycento.test`.
3. If `LYCENTO_BASE_URL` is not set, release builds use `https://lycento.tech`.
4. If `LYCENTO_CHECKOUT_URL` is not set, the app derives it from `LYCENTO_BASE_URL` plus `/checkout`.

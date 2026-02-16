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
npm run typecheck
npm test -- --run
npm run build
cd src-tauri && cargo fmt --check
cd src-tauri && cargo clippy --all-targets -- -D warnings
cd src-tauri && cargo test
```

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

- `LYCENTO_BASE_URL` (required, e.g., `https://lycento.test` or `https://api.lycento.com`)
- `LYCENTO_API_KEY` (optional, for authenticated requests)
- `LYCENTO_CHECKOUT_URL` (required for purchase button)
- `LYCENTO_GRACE_DAYS` (optional, default: `7`)

Create a `.env` file in the project root with these variables. See `.env.example` for reference.

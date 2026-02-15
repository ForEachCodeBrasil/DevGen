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

## Lemon License Configuration

Set these environment variables before running/building the desktop app:

- `DEVGEN_LEMON_CHECKOUT_URL` (required for purchase button)
- `DEVGEN_LEMON_STORE_ID` (optional but recommended)
- `DEVGEN_LEMON_PRODUCT_ID` (optional but recommended)
- `DEVGEN_LEMON_VARIANT_ID` (optional)
- `DEVGEN_LEMON_INSTANCE_NAME` (optional, default: `devgen-desktop`)
- `DEVGEN_LEMON_GRACE_DAYS` (optional, default: `7`)

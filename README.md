# DevGen

DevGen is a free, open-source, macOS-first Tauri + Vue desktop app for offline fake data generation (Brazilian documents like CPF/CNPJ/RG, people, companies, vehicles, utilities).

Contributions are welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

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

## License

[MIT](LICENSE)

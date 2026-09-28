# Contributing to DevGen

Obrigado por ajudar! / Thanks for helping!

## Setup

```bash
npm ci
npm run tauri dev
```

Requirements: Node.js 22+, Rust stable, Xcode Command Line Tools (macOS).

## Adding a generator

1. Create `src-tauri/src/generators/impls/<name>.rs` implementing the `Generator` trait (see `cpf.rs` for an example with tests).
2. Export it in `src-tauri/src/generators/impls/mod.rs`.
3. Register it in `run()` in `src-tauri/src/lib.rs`.
4. Add unit tests for any validation rule (check digits, masks, etc.).

Shared datasets live in `src-tauri/src/datasets/data/*.json`.

## Before opening a PR

```bash
npm run typecheck
npm test -- --run
cd src-tauri && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

- Use [Conventional Commits](https://www.conventionalcommits.org/): `feat(scope): ...`, `fix: ...`, `chore: ...`.
- Keep PRs focused. Describe what changed, why, and how you tested it. Add screenshots for UI changes.
- Adding a dependency? Justify it in the PR and commit the updated lockfile. See [`docs/supply-chain.md`](docs/supply-chain.md).

## Reporting bugs

Open an issue with steps to reproduce, expected vs. actual behavior, and your OS/app version.
Security issues: please report privately via GitHub Security Advisories instead of a public issue.

# Repository Guidelines

## Project Structure & Module Organization
- `src/` contains the Vue 3 + TypeScript frontend.
- Key frontend folders: `src/components`, `src/views`, `src/layouts`, `src/stores`, `src/router`, `src/i18n`, and `src/assets`.
- `public/` stores static files served as-is.
- `src-tauri/` contains the Rust/Tauri desktop backend.
- Generator logic lives in `src-tauri/src/generators/impls/`; shared datasets are in `src-tauri/src/datasets/data/*.json`.
- Build artifacts (`dist/`, `src-tauri/target/`) are generated output and should not be edited manually.

## Build, Test, and Development Commands
- `npm install`: install JS dependencies.
- `npm run dev`: start the frontend dev server (Vite).
- `npm run tauri dev`: run the desktop app in Tauri dev mode.
- `npm run build`: type-check and produce a production frontend build.
- `npm run preview`: preview the built frontend.
- `npm run typecheck`: run `vue-tsc --noEmit`.
- `npm test`: run Vitest tests (configured with `happy-dom`).
- `cd src-tauri && cargo test`: run Rust tests (when added).

## Coding Style & Naming Conventions
- Use TypeScript with strict typing; avoid `any` unless justified.
- Vue Single File Components use PascalCase filenames (example: `MainLayout.vue`).
- Pinia stores and TS modules use clear lowercase names (example: `preferences.ts`).
- Rust files/modules use `snake_case` (example: `credit_card.rs`).
- Keep locale files in `src/i18n` using BCP-47-like names such as `en-US.json` and `pt-BR.json`.
- Match the surrounding file’s formatting style and avoid unrelated reformatting.

## Testing Guidelines
- Frontend tests use Vitest; prefer `*.spec.ts` naming near the tested unit (example: `src/components/Example.spec.ts`).
- Add tests for new behavior and bug fixes, especially generator rules, store state, and view interactions.
- Run `npm test` before opening a PR; run `cargo test` for Rust-side logic changes.

## Commit & Pull Request Guidelines
- Follow Conventional Commits as seen in history: `feat(scope): ...`, `fix: ...`, `chore: ...`.
- Keep commits focused and atomic; separate refactors from feature work.
- PRs should include: what changed, why, how it was tested, and related issue links.
- Include screenshots/GIFs for UI changes and list executed commands (for example: `npm test`, `npm run build`).

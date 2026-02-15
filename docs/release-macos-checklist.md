# macOS Release Checklist

Use this checklist before publishing a new production version.

## 1. Quality Gate

- [ ] `npm ci`
- [ ] `npm run typecheck`
- [ ] `npm test -- --run`
- [ ] `npm run build`
- [ ] `cd src-tauri && cargo fmt --check`
- [ ] `cd src-tauri && cargo clippy --all-targets -- -D warnings`
- [ ] `cd src-tauri && cargo test`

## 2. Build Artifacts

- [ ] `npm run tauri build`
- [ ] Confirm generated files:
  - [ ] `src-tauri/target/release/bundle/macos/DevGen.app`
  - [ ] `src-tauri/target/release/bundle/dmg/DevGen_<version>_aarch64.dmg`

## 3. Signing and Notarization

- [ ] Configure GitHub secrets used by `.github/workflows/release-macos.yml`:
  - [ ] `APPLE_CERTIFICATE`
  - [ ] `APPLE_CERTIFICATE_PASSWORD`
  - [ ] `APPLE_SIGNING_IDENTITY`
  - [ ] `APPLE_ID`
  - [ ] `APPLE_PASSWORD` (app-specific password)
  - [ ] `APPLE_TEAM_ID`
- [ ] Run release workflow from a tag (`vX.Y.Z`) or manual dispatch.
- [ ] Verify signature:
  - [ ] `codesign -dv --verbose=4 src-tauri/target/release/bundle/macos/DevGen.app`
  - [ ] Signature is not `adhoc`.
  - [ ] `TeamIdentifier` is set.
- [ ] Verify Gatekeeper:
  - [ ] `spctl --assess --verbose=4 src-tauri/target/release/bundle/macos/DevGen.app`
  - [ ] Output is accepted.

## 4. Final Sanity

- [ ] Install `.dmg` on a clean macOS machine.
- [ ] Confirm tray flow: open, quick generate, close-to-hide, quit.
- [ ] Confirm offline behavior with network disabled.

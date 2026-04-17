# Ona — Release & Distribution TODO

Follow-ups from the initial release setup (v0.1.1 shipped 2026-04-17).

## Shipped — verified working

- [x] crates.io metadata on `Cargo.toml` (homepage, readme, etc.)
- [x] `dist` configured for 7 targets (mac x2, linux-gnu x2, linux-musl x2, windows)
- [x] GitHub Actions `release.yml` building binaries + shell/powershell installers + msi
- [x] GitHub Actions `publish-crate.yml` for crates.io
- [x] Homebrew tap repo `Proteusiq/homebrew-tap` created and initialized
- [x] `HOMEBREW_TAP_TOKEN` secret set (classic PAT, `repo` scope)
- [x] v0.1.1 tagged and released — all channels except crates.io live

## Blocked on tokens — finish when ready

### crates.io publishing

Publishing to crates.io fails until a token is added.

1. Create a token at https://crates.io/settings/tokens/new
   - Name: `ona-github-actions`
   - Scopes: `publish-new`, `publish-update`
   - Crates: `ona`
2. Add it as a repo secret:
   ```bash
   gh secret set CARGO_REGISTRY_TOKEN --repo Proteusiq/ona --body "<token>"
   ```
3. Re-run the failed run:
   ```bash
   gh run rerun --failed --repo Proteusiq/ona \
     $(gh run list --repo Proteusiq/ona --workflow publish-crate.yml --json databaseId --jq '.[0].databaseId')
   ```
4. Verify at https://crates.io/crates/ona
5. Once live, users can install with `cargo install --locked ona`

## Cleanup

- [ ] Revoke the two unused fine-grained PATs at
      https://github.com/settings/personal-access-tokens
      (`ona-homebrew-tap` and `ona-tap-v2` — both were misconfigured and replaced
      by a classic PAT)
- [ ] Rotate the `HOMEBREW_TAP_TOKEN` classic PAT to a fine-grained one when
      possible (lower blast radius — current one has `repo` scope on all repos)

## Nice-to-haves for future releases

- [ ] Add `CHANGELOG.md` — `dist` will include release notes from it automatically
- [ ] Add a GitHub Release badge and install badge to README.md
- [ ] Consider `cargo-release` for automated version bumps + tagging
  (`cargo release patch --execute`)
- [ ] Submit to AUR (`ona-bin` package) once there's demand from Arch users
- [ ] Submit to homebrew-core once the project has ~75 stars
  (lets users drop the tap prefix: `brew install ona`)
- [ ] Add an updater: set `install-updater = true` in `dist-workspace.toml`
      so users can run `ona update`

## Release procedure (for future versions)

```bash
# 1. Bump version
# Edit Cargo.toml → version = "X.Y.Z"
cargo check  # updates Cargo.lock

# 2. Update CHANGELOG.md (if it exists)

# 3. Commit, tag, push
git commit -am "chore: release vX.Y.Z"
git tag vX.Y.Z
git push && git push --tags

# 4. Watch the workflows
gh run watch --repo Proteusiq/ona
```

GitHub Actions will handle:
- Building 7 binaries in parallel
- Creating the GitHub Release with tarballs, installers, checksums
- Updating `Formula/ona.rb` in `Proteusiq/homebrew-tap`
- Publishing to crates.io (once token is set)

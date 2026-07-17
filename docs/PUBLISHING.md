# Publishing

`mediautil` can be distributed through GitHub Releases, Cargo, npm, PyPI, and Homebrew.

## Required Accounts And Secrets

- GitHub repository with Actions enabled
- `CARGO_REGISTRY_TOKEN` for crates.io
- `NPM_TOKEN` for npm
- `PYPI_API_TOKEN` for PyPI
- A Homebrew tap repository if you want a public formula outside this repo

## Cadence

Use a predictable release cycle once the project has users, for example every two weeks or monthly. Cut patch releases sooner for regressions in existing commands.

## Release

1. Update `Cargo.toml`, `packaging/npm/package.json`, and `packaging/pip/pyproject.toml` to the same version.
2. Update `CHANGELOG.md`.
3. Run:

   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   cargo llvm-cov --all-targets --fail-under-lines 80
   cargo audit
   npm pack --dry-run --prefix packaging/npm
   python -m build packaging/pip
   ```

4. Tag and push:

   ```bash
   git tag v0.1.0
   git push origin main --tags
   ```

GitHub Actions builds release assets and publishes to registries when the relevant secrets exist.

Do not publish a release if any platform build fails. Fix the platform issue or explicitly document that the platform is unsupported before tagging.

## Homebrew

After a release is created, replace the `sha256` in `Formula/mediautil.rb` with the checksum of the source tarball or submit the generated formula to a tap.

# copilot-example-movies

A small Rust web app for tracking movies you want to watch and marking them as seen.

## Development

```bash
cargo run
```

The app listens on `http://127.0.0.1:3000`.

## Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

## GitHub Copilot cloud agent

The repository includes:

- `.github/workflows/copilot-setup-steps.yml` to prepare Copilot's cloud environment with the Rust toolchain and cached dependencies.
- `.github/workflows/ci.yml` to build confidence in cloud-based changes by checking formatting, linting, and tests on pushes and pull requests.

# Contributing

- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [Install](#install)
  - [Run](#run)
  - [Test](#test)
- [Code of Conduct](#code-of-conduct)
- [Commit Message Format](#commit-message-format)
- [Developer Certificate of Origin](#developer-certificate-of-origin)

---

## Getting Started

### Requirements

- [mise](https://mise.jdx.dev)
- [Tauri's platform-specific system dependencies](https://tauri.app/start/prerequisites/) (e.g. `webkit2gtk` on Linux) — not mise-managed, install via your OS package manager

### Install

```bash
mise install
eval "$(mise activate bash)"   # or your shell
bun install
```

### Run

```bash
bun run tauri dev
```

### Test

```bash
cargo test --manifest-path src-tauri/Cargo.toml
bun test
```

## Code of Conduct

Help us keep this project open and inclusive. Please read and follow our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Commit Message Format

This repository follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification and
specificaly the [Angular Commit Message Guidelines](https://github.com/angular/angular/blob/main/CONTRIBUTING.md#commit).

## Developer Certificate of Origin

All commits must be signed off with `git commit -s`, which adds a `Signed-off-by` line certifying the
[Developer Certificate of Origin](https://developercertificate.org): you wrote the contribution, or otherwise
have the right to submit it under this project's license.

A CI check blocks pull requests containing unsigned commits. If you forgot to sign off:

- Last commit: `git commit --amend --signoff`
- Whole branch: `git rebase --signoff <base-branch>`

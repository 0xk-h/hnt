## Contributing to HNT

First off, thank you for considering contributing to HNT! We welcome bug reports, feature requests, and pull requests. This document outlines guidelines for contributing, code style, and the development workflow.

---

## Getting Started

Fork the repository and clone your fork:

`git clone https://github.com/0xk-h/hnt.git`
`cd hnt`

Ensure prerequisites:

- Install Rust and Cargo (latest stable via rustup).
- `cargo fmt` and `cargo clippy` (installed by default with Rust) for formatting and linting.
- (Optional) Use an IDE/Editor like VSCode or IntelliJ with Rust support. The `.vscode` directory includes recommended settings (e.g., Rust toolchain, Prettier for Markdown/JS).

Build & run to verify setup:

`cargo build`
`cargo run -- --help`

---

## Code Style & Linting

- **Rust Code:** Follow Rust conventions. Run `cargo fmt` to format code. Use `cargo clippy` to catch errors and enforce best practices. Fix any warnings before committing.
- **Commit Messages:** Use Conventional Commits style:
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `chore:` for maintenance tasks
  - Example:
    `feat: add AI-assisted commit message feature`
- **Markdown & Templates:** Use Prettier for formatting Markdown and any JS/TS code in templates. Ensure readability and correct spelling/grammar in docs.
- **Interactive UI:** HNT uses the Cliclack library for terminal interactions. When modifying UI flow or prompts, keep consistency in style and usability.

---

## Contributing Workflow

1. **Issue Tracking:** Before coding, consider opening an issue to discuss major changes or to report bugs. This helps align expectations.
2. **Branches:** Create a new branch for your work:
   `git checkout -b feature/my-feature`
3. **Implement & Test:** Make your changes. Frequently test with `cargo run`. For CLI changes, run `hnt <command> --help` to ensure help text is updated.
4. **Update Documentation:** If you add features or options, update `README.md` (and other relevant docs) accordingly.
5. **Commit & Push:** Stage your changes and commit with a clear message or use hnt ai assisted commit messages:
   ```
   hnt push -A feature/my-feature
   ```
6. **Pull Request:** Open a PR to the main branch of the upstream repository. Describe your changes and link related issues.

---

## Issue Guidelines

- **Search before creating:** Check existing issues to avoid duplicates.
- **Clear Titles/Descriptions:** Provide a concise title and detailed description. For bugs, include steps to reproduce and expected vs actual behavior.
- **Labeling:** Use appropriate labels (bug, enhancement, question).

---

## Testing and CI

- **Automated Tests:** Currently, HNT does not have an automated test suite. Before merging, manually verify new features.
- **CI Checks:** (Future) PRs should pass lint/format checks (`cargo fmt` and `cargo clippy`).

---

## Development Environment

- **Rust Toolchain:** Keep Rust updated (`rustup update`) to use the latest stable features.
- **Dependencies:** Managed in `Cargo.toml`. When adding crates, run `cargo update` and ensure compatibility.
- **Environment:** The CLI reads configuration from `~/.hnt/config.toml`. For development, you can manually edit this file or let HNT modify it.

---

Thank you for your contributions! Together we can make HNT even better.

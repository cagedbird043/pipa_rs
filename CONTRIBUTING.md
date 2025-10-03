# Contributing to PIPA-rs

First off, thank you for considering contributing to PIPA-rs! We welcome any contributions, from fixing a typo to implementing a major new feature. This document provides a guide to help you get started.

首先，感谢您考虑为 PIPA-rs 做出贡献！我们欢迎任何形式的贡献，无论是修正一个拼写错误，还是实现一个主要的新功能。本文档旨在为您提供一份入门指南。

## 1. Development Environment Setup | 开发环境设置

Our project relies on a few tools to maintain code quality and consistency. Please ensure you have them installed.

我们的项目依赖一些工具来保持代码质量和一致性。请确保您已安装它们。

### 1.1 Rust Toolchain | Rust 工具链

We use the standard Rust toolchain managed by `rustup`. If you don't have it, you can install it from [rustup.rs](https://rustup.rs/).

我们使用 `rustup` 管理的标准 Rust 工具链。如果您尚未安装，可以从 [rustup.rs](https://rustup.rs/) 获取。

```bash
# Installs rustup, cargo, rustc, etc.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 1.2 Pre-commit Hooks | 提交前钩子

We use the `pre-commit` framework to automatically run `rustfmt` and `clippy` before each commit. This ensures that all code entering the repository is formatted and lint-free.

我们使用 `pre-commit` 框架在每次提交前自动运行 `rustfmt` 和 `clippy`。这确保了所有进入仓库的代码都经过了格式化和静态检查。

**Installation:**

- **For Arch Linux users (推荐):**
  ```bash
  sudo pacman -S pre-commit
  ```
- **For other systems (via pip):**
  ```bash
  pip install pre-commit
  ```

**Activation:**
After installing the framework, you need to activate it for this repository. Run this command from the project's root directory:

安装框架后，您需要为本仓库激活它。在项目根目录下运行此命令：

```bash
pre-commit install
```

From now on, `cargo fmt` and `cargo clippy` will run automatically every time you `git commit`.

此后，每次您执行 `git commit` 时，`cargo fmt` 和 `cargo clippy` 都会自动运行。

## 2. Development Workflow | 开发工作流

1.  **Create a Branch**: Start from the `main` branch and create a new feature branch for your work.
    `git checkout -b your-feature-name`

2.  **Write Code & Tests**: Implement your changes. Remember our **High Test Coverage Mandate**—all new logic should be accompanied by comprehensive unit tests.

3.  **Run Local Checks**: Before committing, it's good practice to run checks manually:

    - `cargo test --workspace`: Run all tests.
    - `cargo clippy --workspace -- -D warnings`: Run the linter.
    - `cargo fmt --all`: Format your code.
      (The `pre-commit` hooks will run the linter and formatter for you automatically).

4.  **Check Test Coverage (Locally)**: To see how well your code is tested, you can generate a local coverage report.

    ```bash
    # 1. Install the tool if you haven't already
    cargo install cargo-tarpaulin

    # 2. Run coverage analysis, excluding placeholder crates
    cargo tarpaulin --workspace \
    --exclude pipa_parser \
    --exclude pipa_core \
    --exclude pipad_server \
    --exclude pipa_rs \
    --out Html

    # 3. Open the report in your browser
    # The main file is tarpaulin-report.html in the project root.
    xdg-open tarpaulin-report.html
    ```

5.  **Commit Your Changes**: We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. This helps us maintain a clear commit history and automate releases.

    - Examples: `feat(collector): ...`, `fix(parser): ...`, `docs: ...`, `chore(ci): ...`

6.  **Create a Pull Request**: Push your branch to GitHub and open a Pull Request against the `main` branch. Provide a clear description of your changes.

## 3. Code Style & Quality | 代码风格与质量

- **Formatting**: Enforced by `rustfmt` via `pre-commit`.
- **Linting**: Enforced by `clippy` (`-D warnings`) via `pre-commit` and CI. No warnings are allowed.
- **Comments**: All public APIs (`pub` functions, structs, etc.) must have documentation comments (`///`). We follow Google's open-source comment style, providing both Chinese and English explanations.

感谢您的贡献！
Thank you for your contribution!

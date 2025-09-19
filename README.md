# PIPA-rs

<p align="left">
  <strong>A Native Performance Analytics Toolchain for Linux, built in Rust.</strong>
  <br>
  <em>(一个基于 Rust 构建的、原生的 Linux 性能分析工具链。)</em>
</p>

<p align="left">
  <a href="https://github.com/cagedbird043/pipa_rs#license--许可证">
    <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg" alt="License">
  </a>
  <a href="https://github.com/cagedbird043/pipa_rs/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/cagedbird043/pipa_rs/ci.yml?branch=main" alt="CI Status">
  </a>
</p>

---

## About | 关于

PIPA-rs is a ground-up rewrite of the [PIPA](https://github.com/ZJU-SPAIL/pipa) performance analysis toolchain. It aims to be a **hyper-reliable, high-performance, and dependency-free** toolkit for Linux system performance analysis by interacting directly with kernel interfaces like `perf_event` and the `/proc` filesystem.

PIPA-rs 是对 [PIPA](https://github.com/ZJU-SPAIL/pipa) 性能分析工具链的彻底重写。它旨在通过直接与 Linux 内核接口（如 `perf_event` 和 `/proc` 文件系统）交互，成为一个**超可靠、高性能且无外部依赖**的性能分析工具包。

## Status | 项目状态

🚧 **Coming Soon & Under Heavy Development** 🚧

The project is in its very early stages. The core architecture and roadmap are being established.

项目正处于早期开发阶段。核心架构和开发路线图正在制定中。

Please check the [**ROADMAP.md**](ROADMAP.md) to see our development plan and follow the progress.

请查阅 [**ROADMAP.md**](ROADMAP.md) 来了解我们的开发计划并跟踪项目进展。

## License | 许可证

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

## Acknowledgments | 致谢

This project is inspired by and is a complete rewrite of the original
[PIPA](https://github.com/ZJU-SPAIL/pipa) project. We thank the original
authors for their foundational work.

本项目受到原始 [PIPA](https://github.com/ZJU-SPAIL/pipa) 项目启发，
并是其完全重写版本。我们感谢原作者们的基础性工作。

# PIPA-rs: A Native Performance Analytics Toolchain in Rust - Project Plan

# PIPA-rs：基于 Rust 的原生性能分析工具链 - 项目计划

## 1. Vision & Core Philosophy | 愿景与核心理念

**PIPA-rs** is a ground-up rewrite of the PIPA performance analysis toolchain, built entirely in Rust. It aims to be a **hyper-reliable, high-performance, and dependency-free** toolkit for Linux system performance analysis.

**PIPA-rs** 是对 PIPA 性能分析工具链的全面重写，完全使用 Rust 构建。它旨在成为一个**超可靠、高性能且无依赖**的 Linux 系统性能分析工具包。

Our core design principles are: | 我们的核心设计原则是：

- **Reliability First** | **可靠性优先**: Leverage Rust's type system and error handling to create robust parsers and collectors that never crash on unexpected input. | 利用 Rust 的类型系统和错误处理机制，创建永远不会因意外输入而崩溃的健壮解析器和收集器。
- **Hyper-Automation** | **超自动化**: Eliminate all manual steps. The tool will orchestrate the entire workflow from workload execution to final report generation. | 消除所有手动步骤。该工具将协调从工作负载执行到最终报告生成的整个工作流程。
- **Performance by Default** | **默认高性能**: The tool itself must be lightweight and efficient, minimizing the observer effect. | 工具本身必须轻量且高效，最小化观察者效应。
- **Zero External Binary Dependencies** | **零外部二进制依赖**: PIPA-rs will not call external binaries like `perf` or `sar`. It will interact directly with the Linux kernel's `perf_event` subsystem and `/proc` filesystem, making it a single, self-contained binary. | PIPA-rs 不会调用 `perf` 或 `sar` 等外部二进制文件。它将直接与 Linux 内核的 `perf_event` 子系统和 `/proc` 文件系统交互，使其成为一个单一的、自包含的二进制文件。

## 2. Architecture Overview | 架构概述

The project will be structured as a Cargo Workspace with several key crates:
项目将构建为一个包含多个关键 crate 的 Cargo 工作空间：

- **`pipa_collector` (Library Crate)** | **`pipa_collector`（库 Crate）**: The foundation of the project. This crate is responsible for all raw data collection. | 项目的基础。此 crate 负责所有原始数据收集。
  - `perf_events` module | `perf_events` 模块: Interacts with the `perf_event_open` syscall to collect performance counter data (both counting and sampling). | 与 `perf_event_open` 系统调用交互，收集性能计数器数据（计数和采样）。
  - `system_stats` module | `system_stats` 模块: Parses the `/proc` filesystem to gather system-wide statistics (CPU, memory, disk, network), replacing `sar`. | 解析 `/proc` 文件系统以收集系统级统计信息（CPU、内存、磁盘、网络），替代 `sar`。
- **`pipa_parser` (Library Crate)** | **`pipa_parser`（库 Crate）**: Responsible for parsing the binary data from `pipa_collector` into structured data formats, primarily [Polars](https://pola.rs/) DataFrames. | 负责将来自 `pipa_collector` 的二进制数据解析为结构化数据格式，主要是 [Polars](https://pola.rs/) DataFrames。
- **`pipa_core` (Library Crate)** | **`pipa_core`（库 Crate）**: The analysis engine. It consumes data from `pipa_parser`, performs data fusion, and calculates high-level performance metrics (CPI, Throughput, Path Length, etc.). | 分析引擎。它消费来自 `pipa_parser` 的数据，执行数据融合，并计算高级性能指标（CPI、吞吐量、路径长度等）。
- **`pipa_cli` (Binary Crate)** | **`pipa_cli`（二进制 Crate）**: The user-facing command-line interface. It provides commands like `run`, `analyze`, `generate`, and orchestrates the calls to the other library crates. | 面向用户的命令行界面。它提供 `run`、`analyze`、`generate` 等命令，并协调对其他库 crate 的调用。
- **`pipad_server` (Binary Crate)** | **`pipad_server`（二进制 Crate）**: A high-performance gRPC service for centralized storage and querying of performance data, built with `tonic` and `sqlx`. | 一个高性能的 gRPC 服务，用于集中存储和查询性能数据，使用 `tonic` 和 `sqlx` 构建。

## 3. Development Roadmap | 开发路线图

The project will be developed in incremental, feature-focused milestones.
项目将以增量式、功能导向的里程碑方式进行开发。

### **Milestone 0: Project Initialization and Engineering Foundation (`v0.0.1`)**

### **里程碑 0：项目初始化与工程基础 (`v0.0.1`)**

_Goal: Establish a robust, modern, and automated development environment to ensure high code quality and smooth collaboration from day one._
_目标：建立一个健壮、现代化、自动化的开发环境，从第一天起就确保高质量的代码和顺畅的协作。_

- [x] **Task 0.1: Project Structure and Git Discipline | 任务 0.1：项目结构与 Git 纪律**

  - [x] **Initialize Cargo Workspace**: Create the workspace with the initial crates: `pipa_collector`, `pipa_parser`, `pipa_core`, `pipa_cli`, `pipad_server`. | **初始化 Cargo 工作空间**：创建包含初始 crate 的工作空间。
  - [x] **Define Git Workflow**: Adopt a clear branching model (e.g., GitFlow or a simpler Trunk-Based Development with feature branches). | **定义 Git 工作流**：采用清晰的分支模型（如 GitFlow 或更简单的基于主干的开发模型）。
  - [x] **Establish Commit Message Convention**: Enforce the [Conventional Commits](https://www.conventionalcommits.org/) standard (e.g., `feat:`, `fix:`, `docs:`, `chore:`). Use a tool like `commitizen` or `cocogitto` to standardize this. | **建立提交信息规范**：强制使用“约定式提交”标准。
  - [x] **Setup `.gitignore`**: Create a comprehensive `.gitignore` file for Rust projects. | **设置 `.gitignore`**：为 Rust 项目创建全面的 `.gitignore` 文件。
  - [x] **Create Project Documentation**:
    - [x] `README.md`: High-level project description, quick start, and contribution guidelines.
    - [x] `CONTRIBUTING.md`: Detailed instructions for contributors (setup, workflow, code of conduct).
    - [x] `ROADMAP.md`: This document.
    - [x] Choose a license (e.g., MIT / Apache 2.0).

- [x] **Task 0.2: Code Style and Quality | 任务 0.2：代码风格与质量**

  - [x] **Setup `rustfmt`**: Configure `rustfmt.toml` to define the project's code formatting style. All code must be formatted with `rustfmt` before committing. | **配置 `rustfmt`**：配置 `rustfmt.toml` 文件以定义项目的代码格式化风格。
  - [x] **Setup `clippy`**: Configure `clippy.toml` for linting. Clippy provides a huge number of lints to catch common mistakes and improve code idioms. The CI will enforce zero clippy warnings. | **配置 `clippy`**：配置 `clippy.toml` 进行代码检查。CI 将强制要求零 clippy 警告。
  - [ ] **Add EditorConfig**: Include an `.editorconfig` file to maintain consistent coding styles across different editors and IDEs.

- [x] **Task 0.3: CI/CD Pipeline Setup | 任务 0.3：CI/CD 流水线搭建**

  - [x] **Choose a CI Platform**: Use GitHub Actions (free for public repos and highly recommended) or GitLab CI. | **选择 CI 平台**：使用 GitHub Actions 或 GitLab CI。
  - [x] **Create the Initial CI Workflow (`ci.yml`)**: This workflow will run on every push and pull request.
    - [x] **Job 1: Format & Lint Check**:
      - Run `cargo fmt -- --check` to ensure all code is formatted.
      - Run `cargo clippy -- -D warnings` to fail the build on any warnings.
    - [x] **Job 2: Build & Test**:
      - Run `cargo build --all-targets` to ensure all crates compile.
      - Run `cargo test --all-targets` to execute all unit and integration tests.
    - [x] **(Optional but Recommended) Setup Code Coverage**: Integrate `cargo-tarpaulin` or `grcov` to measure test coverage and upload reports to a service like [Codecov](https://codecov.io/).
  - [ ] **Create a Release Workflow (`release.yml`)**: This workflow will trigger on creating a new Git tag (e.g., `v0.1.0`).
    - [ ] Build release artifacts (optimized, stripped binaries) for different targets (e.g., `x86_64-unknown-linux-gnu`).
    - [ ] Automatically create a GitHub Release.
    - [ ] Upload the binaries to the GitHub Release.
    - [ ] (Future) Publish the library crates to `crates.io`.

- [x] **Task 0.4: Development Tooling | 任务 0.4：开发工具**
  - [x] **Setup Pre-commit Hooks**: Use a tool like `pre-commit` with Rust hooks to automatically run `cargo fmt` and `cargo clippy` before a commit is created. This catches issues locally before they ever reach CI. | **设置 Pre-commit 钩子**：在提交前自动运行 `cargo fmt` 和 `cargo clippy`。

**Acceptance Criteria for M0** | **M0 的验收标准**:

- The project repository is fully set up with all necessary configuration and documentation files.
- The CI pipeline is functional and automatically checks all incoming code for formatting, linting, and correctness.
- A developer can clone the repository, run a simple setup script (if any), and immediately start contributing, confident that their code quality will be automatically verified.
- The development workflow is clear and documented.

### Milestone 1: The Foundation - `pipa_collector` and Basic CLI (`v0.1`)

### 里程碑 1：基础 - `pipa_collector` 和基础 CLI (`v0.1`)

_Goal: Build a robust, dependency-free data collection engine._
_目标：构建一个健壮的、无依赖的数据收集引擎。_

- [ ] **Task 1.1: Setup Cargo Workspace** | **任务 1.1：设置 Cargo 工作空间**: Initialize the project structure with `pipa_collector` and `pipa_cli` crates. | 使用 `pipa_collector` 和 `pipa_cli` crate 初始化项目结构。
- [ ] **Task 1.2: Implement `system_stats` Module (the `sar` replacement)** | **任务 1.2：实现 `system_stats` 模块（`sar` 的替代品）**:
  - [ ] In `pipa_collector`, create a module to parse `/proc/stat` for CPU utilization. | 在 `pipa_collector` 中创建一个模块来解析 `/proc/stat` 以获取 CPU 使用率。
  - [ ] In `pipa_collector`, create a module to parse `/proc/meminfo` for memory stats. | 在 `pipa_collector` 中创建一个模块来解析 `/proc/meminfo` 以获取内存统计信息。
  - [ ] In `pipa_cli`, create a `monitor` subcommand that periodically calls the `system_stats` functions and prints live system info, verifying the collector's functionality. | 在 `pipa_cli` 中创建一个 `monitor` 子命令，定期调用 `system_stats` 函数并打印实时系统信息，验证收集器的功能。
- [ ] **Task 1.3: Implement `perf_events` Counting Mode (the `perf stat` replacement)** | **任务 1.3：实现 `perf_events` 计数模式（`perf stat` 的替代品）**:
  - [ ] Integrate the `perf-event` crate into `pipa_collector`. | 将 `perf-event` crate 集成到 `pipa_collector` 中。
  - [ ] Implement a function to create and manage a group of performance counters for a given process (`pid`) or system-wide (`-1`). | 实现一个函数来为给定进程（`pid`）或系统级（`-1`）创建和管理一组性能计数器。
  - [ ] In `pipa_cli`, create a `stat` subcommand (`pipa-rs stat -- <command>`) that launches a command, collects total `cycles` and `instructions`, and prints the results upon completion. | 在 `pipa_cli` 中创建一个 `stat` 子命令（`pipa-rs stat -- <command>`），启动一个命令，收集总的 `cycles` 和 `instructions`，并在完成时打印结果。
- [ ] **Task 1.4: Implement `perf_events` Sampling Mode (the `perf record` foundation)** | **任务 1.4：实现 `perf_events` 采样模式（`perf record` 的基础）**:
  - [ ] In `pipa_collector`, configure `perf_event` for sampling with a ring buffer (`mmap`). | 在 `pipa_collector` 中配置 `perf_event` 以使用环形缓冲区（`mmap`）进行采样。
  - [ ] Implement the logic to read raw `PERF_RECORD_SAMPLE` events from the ring buffer. | 实现从环形缓冲区读取原始 `PERF_RECORD_SAMPLE` 事件的逻辑。
  - [ ] In `pipa_cli`, add a `record` subcommand that samples a workload and prints raw sample data (e.g., Instruction Pointer, PID, timestamp) to the console. | 在 `pipa_cli` 中添加一个 `record` 子命令，对工作负载进行采样并将原始样本数据（如指令指针、PID、时间戳）打印到控制台。

**Acceptance Criteria for M1** | **M1 的验收标准**:

- `pipa_cli monitor` can display live CPU and memory usage. | `pipa_cli monitor` 能够显示实时的 CPU 和内存使用情况。
- `pipa_cli stat` can correctly count events for a simple command like `ls`. | `pipa_cli stat` 能够正确计算简单命令（如 `ls`）的事件。
- `pipa_cli record` can capture and display raw performance samples from a running process. | `pipa_cli record` 能够捕获并显示运行进程的原始性能样本。
- The entire functionality depends only on the Linux kernel, not on `perf` or `sar` binaries. | 整个功能仅依赖于 Linux 内核，而不依赖于 `perf` 或 `sar` 二进制文件。

---

### Milestone 2: From Raw Data to Insight - Parsing and Analysis (`v0.2`)

### 里程碑 2：从原始数据到洞察 - 解析和分析 (`v0.2`)

_Goal: Transform raw collected data into structured, meaningful metrics._
_目标：将收集的原始数据转换为结构化的、有意义的指标。_

- [ ] **Task 2.1: Develop `pipa_parser` Crate** | **任务 2.1：开发 `pipa_parser` Crate**:
  - [ ] Add `polars` as a core dependency. | 添加 `polars` 作为核心依赖。
  - [ ] Implement a parser that takes the raw binary `PERF_RECORD_*` events from `pipa_collector` and transforms them into a structured Polars DataFrame. | 实现一个解析器，接收来自 `pipa_collector` 的原始二进制 `PERF_RECORD_*` 事件并将其转换为结构化的 Polars DataFrame。
  - [ ] Refine the `system_stats` output into clean DataFrames. | 将 `system_stats` 输出优化为清洁的 DataFrames。
- [ ] **Task 2.2: Develop `pipa_core` Crate** | **任务 2.2：开发 `pipa_core` Crate**:
  - [ ] Create a `Metrics` struct to hold all final analysis results. | 创建一个 `Metrics` 结构体来保存所有最终分析结果。
  - [ ] Implement the `analyze()` function, which takes the parsed DataFrames from `pipa_parser` and a transaction count, and calculates key metrics (CPI, throughput, etc.). | 实现 `analyze()` 函数，接收来自 `pipa_parser` 的解析后 DataFrames 和事务计数，并计算关键指标（CPI、吞吐量等）。
- [ ] **Task 2.3: Enhance `pipa_cli` with Analysis Workflow** | **任务 2.3：使用分析工作流增强 `pipa_cli`**:
  - [ ] Create an `analyze` subcommand that can take data collected from `record` (e.g., from a file) and produce a final metrics report in YAML or JSON format (replaces `pipa dump`). | 创建一个 `analyze` 子命令，可以接收从 `record` 收集的数据（例如从文件中）并生成 YAML 或 JSON 格式的最终指标报告（替代 `pipa dump`）。
- [ ] **Task 2.4: Implement the "Magic" `run` Command** | **任务 2.4：实现"魔法"`run` 命令**:
  - [ ] Create the `pipa-rs run -- <command>` subcommand. | 创建 `pipa-rs run -- <command>` 子命令。
  - [ ] This command will orchestrate the entire workflow: | 此命令将协调整个工作流程：
    1.  Start `pipa_collector` in the background (both `perf_events` and `system_stats` threads). | 在后台启动 `pipa_collector`（`perf_events` 和 `system_stats` 线程）。
    2.  Execute the user's `<command>` as a subprocess. | 作为子进程执行用户的 `<command>`。
    3.  **Crucially, capture the subprocess's stdout/stderr and parse it in real-time to automatically extract key information like transaction counts.** | **关键是，捕获子进程的 stdout/stderr 并实时解析以自动提取关键信息如事务计数。**
    4.  Upon command completion, stop the collectors. | 命令完成后，停止收集器。
    5.  Feed all collected data directly into `pipa_parser` and `pipa_core`. | 将所有收集的数据直接输入到 `pipa_parser` 和 `pipa_core`。
    6.  Print the final, comprehensive performance report. | 打印最终的、综合的性能报告。

**Acceptance Criteria for M2** | **M2 的验收标准**:

- `pipa-rs run -- perf bench futex hash` executes and produces a complete, automated performance report without any manual intervention. | `pipa-rs run -- perf bench futex hash` 执行并生成完整的、自动化的性能报告，无需任何手动干预。
- The output report is structured (YAML/JSON) and contains all key metrics from the original PIPA project. | 输出报告是结构化的（YAML/JSON）并包含原始 PIPA 项目的所有关键指标。

---

### Milestone 3: Data Persistence and Sharing - The `PIPAD` Service (`v0.3`)

### 里程碑 3：数据持久化和共享 - `PIPAD` 服务 (`v0.3`)

_Goal: Implement a robust C/S architecture for centralized performance data management._
_目标：实现一个健壮的客户端/服务器架构，用于集中的性能数据管理。_

- [ ] **Task 3.1: Define gRPC Interface** | **任务 3.1：定义 gRPC 接口**: Finalize the `.proto` file for communication between the CLI and the server. | 确定用于 CLI 和服务器之间通信的 `.proto` 文件。
- [ ] **Task 3.2: Implement `pipad_server`** | **任务 3.2：实现 `pipad_server`**:
  - [ ] Set up a `tonic`-based gRPC server. | 设置基于 `tonic` 的 gRPC 服务器。
  - [ ] Integrate `sqlx` with `rusqlite` to create a typesafe database layer. | 将 `sqlx` 与 `rusqlite` 集成以创建类型安全的数据库层。
  - [ ] Implement the `Deploy` RPC endpoint to receive metrics from the client and store them in the SQLite database. | 实现 `Deploy` RPC 端点，从客户端接收指标并将其存储在 SQLite 数据库中。
  - [ ] Implement the `DownloadFullTable` RPC endpoint. | 实现 `DownloadFullTable` RPC 端点。
- [ ] **Task 3.3: Implement `pipad` Client in `pipa_cli`** | **任务 3.3：在 `pipa_cli` 中实现 `pipad` 客户端**:
  - [ ] Add an `upload` subcommand that sends the results from `analyze` or `run` to a `pipad_server`. | 添加一个 `upload` 子命令，将 `analyze` 或 `run` 的结果发送到 `pipad_server`。
  - [ ] Add a `download` subcommand to fetch data from the server. | 添加一个 `download` 子命令从服务器获取数据。

**Acceptance Criteria for M3** | **M3 的验收标准**:

- `pipad_server` can be run as a standalone, long-running service. | `pipad_server` 可以作为独立的、长期运行的服务运行。
- `pipa-rs upload` can successfully send a performance report to the server, which is then persisted in a SQLite file. | `pipa-rs upload` 可以成功地将性能报告发送到服务器，然后持久化到 SQLite 文件中。
- The stored data is compatible with the existing `ShowMeYourPIPA` Grafana dashboard. | 存储的数据与现有的 `ShowMeYourPIPA` Grafana 仪表板兼容。

---

### Milestone 4: Advanced Analysis and Polish (`v0.4+`)

### 里程碑 4：高级分析和优化 (`v0.4+`)

_Goal: Reach feature parity with advanced PIPA features and enhance usability._
_目标：达到与高级 PIPA 功能的特性对等，并增强可用性。_

- [ ] **Task 4.1: Call Graph and DWARF/ELF Parsing** | **任务 4.1：调用图和 DWARF/ELF 解析**:
  - [ ] Integrate `gimli` and `object` crates into `pipa_parser`. | 将 `gimli` 和 `object` crates 集成到 `pipa_parser` 中。
  - [ ] Implement call stack symbolication: translate instruction pointer addresses from `PERF_RECORD_SAMPLE` into `function+offset` and `file:line`. | 实现调用栈符号化：将 `PERF_RECORD_SAMPLE` 中的指令指针地址转换为 `function+offset` 和 `file:line`。
  - [ ] In `pipa_core`, build a call graph data structure (`petgraph` is a good choice). | 在 `pipa_core` 中构建调用图数据结构（`petgraph` 是一个不错的选择）。
- [ ] **Task 4.2: Script Generation** | **任务 4.2：脚本生成**:
  - [ ] Re-implement the `generate` subcommand in `pipa_cli` using the `Tera` template engine. This is for users who still prefer a script-based workflow. | 使用 `Tera` 模板引擎在 `pipa_cli` 中重新实现 `generate` 子命令。这是为仍然偏好基于脚本工作流程的用户准备的。
- [ ] **Task 4.3: Usability and Output** | **任务 4.3：可用性和输出**:
  - [ ] Add static plot generation using the `plotters` crate (`pipa-rs plot ...`). | 使用 `plotters` crate 添加静态图表生成功能（`pipa-rs plot ...`）。
  - [ ] Enhance `pipa_cli` output with progress bars (`indicatif`) and nicely formatted tables (`comfy-table`). | 使用进度条（`indicatif`）和格式化表格（`comfy-table`）增强 `pipa_cli` 输出。
- [ ] **Task 4.4: (Stretch Goal) eBPF Integration** | **任务 4.4：（延伸目标）eBPF 集成**:
  - [ ] Explore using `aya` or `libbpf-rs` to attach eBPF programs for low-overhead, custom tracing, moving beyond `perf_event`'s capabilities. | 探索使用 `aya` 或 `libbpf-rs` 来附加 eBPF 程序进行低开销、自定义跟踪，超越 `perf_event` 的能力。

---

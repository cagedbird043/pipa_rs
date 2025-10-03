# PIPA-rs Design Document

This document outlines the core architecture and design principles of the PIPA-rs project. It serves as a guide for development and a reference for understanding key technical decisions.

## 1. Core Philosophy

(This section can be expanded from the `ROADMAP.md`.)

- **Reliability First**: Using Rust's strong type system and error handling (`Result`, `Option`) to build a tool that is robust against malformed data and unexpected conditions.
- **Hyper-Automation**: Orchestrating the entire analysis workflow within a single process, eliminating the need for manual steps or intermediate files.
- **Performance by Default**: The tool itself is a native, compiled binary with minimal runtime overhead.
- **Zero External Binary Dependencies**: Interacting directly with kernel interfaces (`perf_event_open`, `/proc` filesystem) to remain self-contained and highly portable across Linux distributions.

## 2. Architecture

(This section can be expanded from the `ROADMAP.md`.)

The project is a Cargo Workspace composed of several loosely coupled crates, each with a single responsibility:

- `pipa_collector`: Raw data acquisition from the kernel.
- `pipa_parser`: Transformation of raw data into structured formats (Polars DataFrames).
- `pipa_core`: High-level metric calculation and analysis logic.
- `pipa_cli`: The user-facing application layer.
- `pipad_server`: The optional data persistence service.

## 3. CLI User Experience (UX) and Interaction Model

The PIPA-rs command-line interface is designed to be **context-aware** and offer **progressive complexity**. The central design pattern is **"Configuration as Core"**: all user intentions are ultimately translated into a single, unified `PipaConfig` struct which drives the execution engine.

### The Core `PipaConfig` Object

A central `PipaConfig` struct (defined in `pipa_core` or `pipa_cli`) represents a complete analysis job. It declaratively defines the target, collection parameters, analysis steps, and output format.

### Interaction Modes

Users can create and execute a `PipaConfig` in three ways, providing a smooth learning curve:

#### Mode 1: Interactive Wizard (for beginners)

- **Command**: `pipa-rs run` (when no config file is found) or `pipa-rs generate-config`.
- **Workflow**: The tool launches an interactive wizard that guides the user through setting up the configuration.
- **Outcome**: At the end of the wizard, the user can choose to:
  1.  **Run & Save**: Execute the analysis and save the generated configuration to a file (e.g., `pipa.toml`). (Default for `run`)
  2.  **Save Only**: Only save the configuration file. (Default for `generate-config`)
  3.  **Run Only**: Execute the analysis without saving the config.
  4.  **Discard**: Exit without doing anything.

#### Mode 2: Command-Line Flags (for everyday use)

- **Command**: `pipa-rs run --events cycles,instructions -- <command>...`
- **Workflow**: `clap` parses all arguments and flags. A `PipaConfig` object is constructed in memory from these flags, using default values for any unspecified options.
- **Feature**: The `--save-config <path>` flag allows users to persist a command-line-driven configuration into a file for future use, bridging the gap between ad-hoc analysis and reproducible workflows.

#### Mode 3: Declarative Configuration File (for automation and experts)

- **Command**: `pipa-rs run --config my_analysis.toml`
- **Workflow**: The tool loads, parses, and deserializes the specified TOML/YAML file into a `PipaConfig` object.
- **Flexibility**: Command-line flags can be used to override specific values in the configuration file, providing maximum flexibility (e.g., `pipa-rs run -c base.toml --cpu 8`).

### The `generate` Command, Redefined

The `generate` command (or a better name like `init` or `generate-config`) is no longer for creating shell scripts. Its sole purpose is to run the interactive wizard and save the resulting declarative configuration file, empowering users to build up their library of reusable analysis recipes.

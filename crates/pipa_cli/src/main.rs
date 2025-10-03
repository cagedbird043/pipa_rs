// Copyright 2025 cagedbird043
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The main command-line interface for PIPA-rs.
//!
//! PIPA-rs 的主命令行界面。

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue, style,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use pipa_collector::system_stats::{CpuStats, MemoryStats};
use std::{
    io::{Stdout, Write, stdout},
    time::Duration,
};

/// A Native Performance Analytics Toolchain for Linux, built in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Periodically monitor and display live system statistics.
    /// 周期性地监控并显示实时系统统计信息。
    Monitor {
        /// The refresh interval in seconds.
        /// 刷新间隔（秒）。
        #[arg(short, long, default_value_t = 1)]
        interval: u64,
    },
}

/// Helper function to set up the terminal for TUI mode.
/// 设置终端进入 TUI 模式的辅助函数。
#[cfg(not(tarpaulin_include))]
fn setup_terminal() -> Result<Stdout> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    Ok(stdout)
}

/// Helper function to restore the terminal to its original state.
/// 恢复终端至原始状态的辅助函数。
#[cfg(not(tarpaulin_include))]
fn restore_terminal(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}

/// Main application logic for the monitor subcommand.
/// `monitor` 子命令的主应用逻辑。
#[cfg(not(tarpaulin_include))]
fn run_monitor(interval: u64) -> Result<()> {
    let mut f = setup_terminal()?;
    let mut prev_stats: Option<CpuStats> = None;
    let tick_rate = Duration::from_millis(interval * 1000);

    loop {
        let current_stats = pipa_collector::system_stats::read_cpu_stats()?;
        let mem_stats = pipa_collector::system_stats::read_memory_stats()?;

        let cpu_usage_percent = if let Some(prev) = prev_stats {
            calculate_cpu_usage(&prev, &current_stats)
        } else {
            0.0
        };
        prev_stats = Some(current_stats);

        // Pass stdout to the drawing function to give it drawing capabilities.
        draw_ui(&mut f, interval, cpu_usage_percent, &mem_stats)?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    restore_terminal(&mut f)?;
    Ok(())
}

/// Renders the UI frame to the terminal using absolute cursor positioning.
/// 使用绝对光标定位将 UI 帧渲染到终端。
#[cfg(not(tarpaulin_include))]
fn draw_ui<W: Write>(
    f: &mut W,
    interval: u64,
    cpu_usage: f64,
    mem_stats: &MemoryStats,
) -> Result<()> {
    let mem_used_gib = (mem_stats.total - mem_stats.available) as f64 / 1024.0 / 1024.0;
    let mem_available_gib = mem_stats.available as f64 / 1024.0 / 1024.0;
    let mem_total_gib = mem_stats.total as f64 / 1024.0 / 1024.0;

    // queue! batches commands for performance, then flush() writes them all at once.
    // queue! 批量处理命令以提高性能，然后 flush() 一次性将它们全部写入。
    queue!(
        f,
        // First, clear the entire screen.
        style::Print("\x1B[2J"),
        // --- Draw Title ---
        cursor::MoveTo(0, 0),
        style::Print(format!(
            "--- PIPA-rs Live Monitor (Interval: {}s, Press 'q' to exit) ---",
            interval
        )),
        // --- Draw CPU Section ---
        cursor::MoveTo(2, 2),
        style::Print("[ CPU Usage ]"),
        cursor::MoveTo(2, 3),
        style::Print(format!(
            "[{:<20}] {:.2}%",
            "█".repeat((cpu_usage / 5.0).round() as usize),
            cpu_usage
        )),
        // --- Draw Memory Section ---
        cursor::MoveTo(2, 5),
        style::Print("[ Memory Usage ]"),
        cursor::MoveTo(2, 6),
        style::Print(format!("{:<12} {:>10.2} GiB", "Used:", mem_used_gib)),
        cursor::MoveTo(2, 7),
        style::Print(format!("{:<12} {:>10.2} GiB", "Available:", mem_available_gib)),
        cursor::MoveTo(2, 8),
        style::Print(format!("{:<12} {:>10.2} GiB", "Total:", mem_total_gib)),
    )?;

    // This is the crucial step that draws everything queued above.
    // 这是绘制上面队列中所有内容的关键步骤。
    f.flush()?;

    Ok(())
}

fn calculate_cpu_usage(prev: &CpuStats, current: &CpuStats) -> f64 {
    let prev_idle = prev.idle + prev.iowait;
    let current_idle = current.idle + current.iowait;

    let prev_non_idle = prev.user + prev.nice + prev.system + prev.irq + prev.softirq + prev.steal;
    let current_non_idle = current.user
        + current.nice
        + current.system
        + current.irq
        + current.softirq
        + current.steal;

    let prev_total = prev_idle + prev_non_idle;
    let current_total = current_idle + current_non_idle;

    let total_delta = (current_total - prev_total) as f64;
    let idle_delta = (current_idle - prev_idle) as f64;

    if total_delta == 0.0 {
        0.0
    } else {
        let usage_percent = (1.0 - idle_delta / total_delta) * 100.0;
        // Clamp between 0 and 100 in case of weird edge cases
        usage_percent.clamp(0.0, 100.0)
    }
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { interval } => {
            run_monitor(interval)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_cpu_usage_basic() {
        let prev = CpuStats {
            user: 100,
            nice: 10,
            system: 50,
            idle: 1000,
            iowait: 20,
            irq: 5,
            softirq: 5,
            steal: 0,
            guest: 0,
            guest_nice: 0,
        };
        let current = CpuStats {
            user: 200,
            nice: 10,
            system: 100,
            idle: 1100,
            iowait: 20,
            irq: 10,
            softirq: 5,
            steal: 0,
            guest: 0,
            guest_nice: 0,
        };
        // Non-idle delta = (200-100) + (100-50) + (10-5) = 100 + 50 + 5 = 155
        // Idle delta = (1100-1000) = 100
        // Total delta = 155 + 100 = 255
        // Usage = (155 / 255) * 100 = ~60.78%
        let usage = calculate_cpu_usage(&prev, &current);
        assert!((usage - 60.78).abs() < 0.01);
    }

    #[test]
    fn test_calculate_cpu_usage_no_change() {
        let prev = CpuStats { idle: 100, ..Default::default() };
        let current = CpuStats { idle: 100, ..Default::default() };
        assert_eq!(calculate_cpu_usage(&prev, &current), 0.0);
    }

    #[test]
    fn test_draw_ui() {
        // 1. Create our in-memory "fake terminal"
        let mut buffer: Vec<u8> = Vec::new();

        // 2. Define the data we want to draw
        let mem_stats = MemoryStats {
            total: 1024 * 1024 * 16,
            available: 1024 * 1024 * 8,
            ..Default::default()
        };

        // 3. Call our drawing function, but give it the fake terminal
        draw_ui(&mut buffer, 1, 50.0, &mem_stats).unwrap();

        // 4. Convert the raw bytes (which include ANSI codes) into a string
        let output = String::from_utf8(buffer).unwrap();

        // 5. Assert that the output string contains the content we expect!
        assert!(output.contains("[ CPU Usage ]"));
        assert!(output.contains("50.00%"));
        assert!(output.contains("[ Memory Usage ]"));
        assert!(output.contains("Used:"));
        // 16 total - 8 available = 8 used
        assert!(output.contains("8.00 GiB"));
        assert!(output.contains("Total:"));
        assert!(output.contains("16.00 GiB"));

        // We could even test for specific ANSI codes if we wanted to be extremely precise
        // For example, does it start with the "clear screen" code?
        assert!(output.starts_with("\x1B[2J"));
    }
}

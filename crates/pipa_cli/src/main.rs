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
use std::{thread, time::Duration};

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

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { interval } => {
            println!("Starting PIPA-rs monitor... Press Ctrl+C to exit.");
            thread::sleep(Duration::from_secs(1)); // Give user time to read the message

            loop {
                // 1. 调用 collector
                let cpu_stats = pipa_collector::system_stats::read_cpu_stats()?;
                let mem_stats = pipa_collector::system_stats::read_memory_stats()?;

                // 2. 清理屏幕 (ANSI escape code)
                // \x1B[2J: 清除整个屏幕
                // \x1B[1;1H: 将光标移动到第 1 行第 1 列
                print!("\x1B[2J\x1B[1;1H");

                // 3. 格式化并打印输出
                println!("--- PIPA-rs Live Monitor (Interval: {}s) ---", interval);
                println!();
                println!("[ CPU Usage (jiffies since boot) ]");
                println!(
                    "  User: {:<12}   System: {:<12}   Idle: {:<12}",
                    cpu_stats.user, cpu_stats.system, cpu_stats.idle
                );
                println!();
                println!("[ Memory Usage (kB) ]");
                println!(
                    "  Total: {:<12}   Available: {:<12}   Free: {:<12}",
                    mem_stats.total, mem_stats.available, mem_stats.free
                );
                println!("  Cached: {:<12}", mem_stats.cached);
                println!("\n(Press Ctrl+C to exit)");

                // 4. 等待下一个周期
                thread::sleep(Duration::from_secs(interval));
            }
        }
    }
}

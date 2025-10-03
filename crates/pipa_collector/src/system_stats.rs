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

//! This module is responsible for collecting system-wide statistics by parsing the `/proc` filesystem.
//! It serves as a replacement for tools like `sar`.
//!
//! 本模块负责通过解析 `/proc` 文件系统来收集系统级统计信息。
//! 它的功能是替代像 `sar` 这样的工具。

use std::io;
use std::num::ParseIntError;

/// A unified error type for all potential failures within the `pipa_collector`.
/// This enum allows for precise error handling.
///
/// `pipa_collector` 中所有潜在失败的统一错误类型。
/// 这个枚举允许我们进行精确的错误处理。
#[derive(Debug)]
pub enum PipaCollectorError {
    /// Represents an I/O error that occurred while reading a file (e.g., from `/proc`).
    /// 代表在读取文件时发生的 I/O 错误（例如，从 `/proc` 读取时）。
    Io(io::Error),
    /// Represents an error that occurred while parsing a string into a number.
    /// 代表在将字符串解析为数字时发生的错误。
    Parse(ParseIntError),
    /// Represents a format mismatch in the parsed file content.
    /// 代表在解析的文件内容中出现格式不匹配。
    InvalidFormat(String),
    /// Represents missing data where it was expected.
    /// 代表在预期位置缺少数据。
    MissingData(String),
}

// Boilerplate to allow easy conversion from standard errors using the `?` operator.
// 使用 `?` 操作符简化从标准错误类型到自定义错误类型的转换的模板代码。
impl From<io::Error> for PipaCollectorError {
    fn from(err: io::Error) -> Self {
        PipaCollectorError::Io(err)
    }
}

impl From<ParseIntError> for PipaCollectorError {
    fn from(err: ParseIntError) -> Self {
        PipaCollectorError::Parse(err)
    }
}

/// Holds aggregated CPU time statistics from `/proc/stat`.
/// The values are in units of `jiffies` (typically 1/100s of a second).
///
/// 存储从 `/proc/stat` 中聚合的 CPU 时间统计信息。
/// 所有值的单位都是 `jiffies`（通常是 1/100 秒）。
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct CpuStats {
    /// Time spent in user mode. / 在用户模式下花费的时间。
    pub user: u64,
    /// Time spent in user mode with low priority (nice). / 在低优先级用户模式下花费的时间 (nice)。
    pub nice: u64,
    /// Time spent in system mode. / 在系统模式下花费的时间。
    pub system: u64,
    /// Time spent in the idle task. / 在空闲任务中花费的时间。
    pub idle: u64,
    /// Time waiting for I/O to complete. / 等待 I/O 完成的时间。
    pub iowait: u64,
    /// Time servicing interrupts. / 服务于中断的时间。
    pub irq: u64,
    /// Time servicing softirqs. / 服务于软中断的时间。
    pub softirq: u64,
    /// Stolen time, which is the time spent in other operating systems when running in a virtualized environment.
    /// 被偷走的时间，即在虚拟化环境中运行时，在其他操作系统中花费的时间。
    pub steal: u64,
    /// Time spent running a virtual CPU for guest operating systems.
    /// 为客户操作系统运行虚拟 CPU 所花费的时间。
    pub guest: u64,
    /// Time spent running a niced guest.
    /// 运行一个 niced 客户虚拟机所花费的时间。
    pub guest_nice: u64,
}

/// Parses a single line from `/proc/stat` (the aggregated "cpu" line) into a `CpuStats` struct.
/// This function is kept private and pure (no I/O) to make it easily testable.
///
/// 将 `/proc/stat` 的单行（聚合的 "cpu" 行）解析为 `CpuStats` 结构体。
/// 这个函数保持私有和纯粹（无 I/O），以便于测试。
fn parse_cpu_stats_from_line(line: &str) -> Result<CpuStats, PipaCollectorError> {
    // A small helper macro to reduce boilerplate when parsing iterator values.
    // 一个小的辅助宏，用于减少解析迭代器值时的模板代码。
    macro_rules! parse_next {
        ($iter:expr, $field_name:literal) => {
            $iter
                .next()
                .ok_or_else(|| {
                    PipaCollectorError::MissingData(format!("Missing value for {}", $field_name))
                })?
                .parse::<u64>()?
        };
    }

    // `/proc/stat` might have one or two spaces after "cpu". `strip_prefix` handles one case,
    // and `or_else` provides a fallback to try the other.
    let trimmed =
        line.strip_prefix("cpu  ").or_else(|| line.strip_prefix("cpu ")).ok_or_else(|| {
            PipaCollectorError::InvalidFormat(
                "Line does not start with 'cpu ' or 'cpu  '".to_string(),
            )
        })?;

    let mut values = trimmed.split_whitespace();

    let stats = CpuStats {
        user: parse_next!(values, "user"),
        nice: parse_next!(values, "nice"),
        system: parse_next!(values, "system"),
        idle: parse_next!(values, "idle"),
        iowait: parse_next!(values, "iowait"),
        irq: parse_next!(values, "irq"),
        softirq: parse_next!(values, "softirq"),
        steal: parse_next!(values, "steal"),
        guest: parse_next!(values, "guest"),
        guest_nice: parse_next!(values, "guest_nice"),
    };

    Ok(stats)
}

/// Reads and parses aggregated CPU statistics from the `/proc/stat` file.
/// This is the main public entry point for this functionality.
///
/// 从 `/proc/stat` 文件中读取并解析聚合的 CPU 统计信息。
/// 这是该功能的主要公共入口点。
pub fn read_cpu_stats() -> Result<CpuStats, PipaCollectorError> {
    let content = std::fs::read_to_string("/proc/stat")?;
    let first_line = content.lines().next().ok_or_else(|| {
        PipaCollectorError::InvalidFormat("Cannot read first line from /proc/stat".to_string())
    })?;

    parse_cpu_stats_from_line(first_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu_stats_happy_path_two_spaces() {
        let line = "cpu  74608 2520 24433 1117073 6176 4054 0 0 0 0";
        let stats = parse_cpu_stats_from_line(line).unwrap();

        assert_eq!(stats.user, 74608);
        assert_eq!(stats.nice, 2520);
        assert_eq!(stats.system, 24433);
        assert_eq!(stats.idle, 1117073);
        assert_eq!(stats.iowait, 6176);
        assert_eq!(stats.irq, 4054);
        assert_eq!(stats.softirq, 0);
        assert_eq!(stats.steal, 0);
        assert_eq!(stats.guest, 0);
        assert_eq!(stats.guest_nice, 0);
    }

    #[test]
    fn test_parse_cpu_stats_happy_path_one_space() {
        let line = "cpu 74608 2520 24433 1117073 6176 4054 0 0 0 0";
        let stats = parse_cpu_stats_from_line(line).unwrap();
        assert_eq!(stats.user, 74608);
    }

    #[test]
    fn test_parse_cpu_stats_invalid_prefix() {
        let line = "cqu 74608 2520 24433 1117073 6176 4054 0 0 0 0";
        let result = parse_cpu_stats_from_line(line);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PipaCollectorError::InvalidFormat(_)));
    }

    #[test]
    fn test_parse_cpu_stats_not_enough_values() {
        let line = "cpu  74608 2520 24433";
        let result = parse_cpu_stats_from_line(line);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PipaCollectorError::MissingData(_)));
    }

    #[test]
    fn test_parse_cpu_stats_non_numeric_value() {
        let line = "cpu  74608 2520 not-a-number 1117073 6176 4054 0 0 0 0";
        let result = parse_cpu_stats_from_line(line);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PipaCollectorError::Parse(_)));
    }
}

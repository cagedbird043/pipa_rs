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

//! This module provides a low-level, unsafe interface to the `perf_event_open` syscall.
//! It is designed to be the single point of contact with raw `perf_event_open` calls,
//! isolating all `unsafe` logic related to this functionality. The primary motivation
//! for using this raw interface is to gain access to features not exposed by safe
//! high-level libraries, such as creating inheritable performance counter groups
//! for monitoring child processes.
//!
//! 本模块提供了对 `perf_event_open` 系统调用的底层、非安全接口。
//! 它被设计为与原始 `perf_event_open` 调用的唯一接触点，隔离了所有与此功能相关的 `unsafe` 逻辑。
//! 使用此原始接口的主要动机是访问安全的高级库未暴露的功能，例如为监控子进程创建可继承的性能计数器组。

use crate::system_stats::PipaCollectorError;
use perf_event_open_sys as sys;
use std::io;
use std::os::unix::io::RawFd;

/// Represents a specific hardware performance event that can be monitored.
///
/// 代表一个可以被监控的特定硬件性能事件。
#[derive(Debug, Clone, Copy)]
pub enum PerfEvent {
    /// Counts the number of CPU cycles. / 统计 CPU 周期数。
    Cycles,
    /// Counts the number of instructions executed. / 统计执行的指令数。
    Instructions,
}

impl PerfEvent {
    /// Converts the enum variant into the raw `type` and `config` values required by `perf_event_attr`.
    ///
    /// 将枚举变体转换为 `perf_event_attr` 所需的原始 `type` 和 `config` 值。
    fn to_config(self) -> (u32, u64) {
        match self {
            Self::Cycles => {
                (sys::bindings::PERF_TYPE_HARDWARE, sys::bindings::PERF_COUNT_HW_CPU_CYCLES as u64)
            }
            Self::Instructions => (
                sys::bindings::PERF_TYPE_HARDWARE,
                sys::bindings::PERF_COUNT_HW_INSTRUCTIONS as u64,
            ),
        }
    }
}

/// A handle to a group of performance counters, ensuring they are closed on drop.
/// This struct manages the lifecycle of the file descriptors returned by `perf_event_open`.
///
/// 性能计数器组的句柄，确保在销毁时关闭它们。
/// 此结构体管理 `perf_event_open` 返回的文件描述符的生命周期。
#[derive(Debug)]
pub struct EventGroup {
    fds: Vec<RawFd>,
}

impl EventGroup {
    /// Returns the file descriptor of the group leader.
    /// This is the primary FD used for group-wide operations like `ioctl`.
    ///
    /// 返回组长的文件描述符。
    /// 这是用于组范围操作（如 `ioctl`）的主要文件描述符。
    pub fn leader_fd(&self) -> RawFd {
        // The first FD in our list is always the leader.
        // The constructor ensures the list is never empty.
        self.fds[0]
    }
}

impl Drop for EventGroup {
    /// Closes all file descriptors associated with this event group.
    ///
    /// 关闭与此事件组关联的所有文件描述符。
    fn drop(&mut self) {
        for &fd in &self.fds {
            // This is an unsafe call to a C function, but it's safe in this context
            // as we own the file descriptors and they are valid until `drop` is called.
            // 这是一个对 C 函数的非安全调用，但在这种上下文中是安全的，
            // 因为我们拥有文件描述符，并且它们在 `drop` 被调用前都是有效的。
            unsafe {
                libc::close(fd);
            }
        }
    }
}

/// Creates a group of performance counters that can be inherited by child processes.
/// The group is created in a disabled state.
///
/// 创建一个可由子进程继承的性能计数器组。
/// 该组在创建时处于禁用状态。
///
/// # Arguments
/// * `events`: A slice of `PerfEvent`s to monitor. The first event becomes the group leader.
///
/// # Returns
/// An `EventGroup` handle on success, which manages the file descriptors.
pub fn create_event_group(events: &[PerfEvent]) -> Result<EventGroup, PipaCollectorError> {
    if events.is_empty() {
        return Err(PipaCollectorError::InvalidFormat(
            "Cannot create an event group with no events.".to_string(),
        ));
    }

    let mut fds = Vec::with_capacity(events.len());
    let mut leader_fd: RawFd = -1;

    for (i, event) in events.iter().enumerate() {
        let (type_, config) = event.to_config();

        let mut attrs = sys::bindings::perf_event_attr {
            type_,
            config,
            size: std::mem::size_of::<sys::bindings::perf_event_attr>() as u32,
            ..Default::default()
        };

        // --- Critical Settings for Child Process Monitoring ---
        if i == 0 {
            attrs.set_disabled(1);
            attrs.set_inherit(1);
            attrs.read_format =
                (sys::bindings::PERF_FORMAT_GROUP | sys::bindings::PERF_FORMAT_ID) as u64;
        }

        // The syscall is unsafe because it's a raw FFI call.
        // We must ensure the arguments are valid.
        // `pid = 0`: Monitor the current process's children (when they exec).
        // `cpu = -1`: Monitor on all CPUs.
        // `group_fd`: The FD of the group leader, or -1 if this *is* the leader.
        // `flags = 0`: No special flags needed.
        let fd = unsafe { sys::perf_event_open(&mut attrs, 0, -1, leader_fd, 0) };

        if fd < 0 {
            // On error, `perf_event_open` returns a negative value.
            // We capture the OS error (from `errno`) and return it.
            return Err(PipaCollectorError::Io(io::Error::last_os_error()));
        }

        if i == 0 {
            leader_fd = fd;
        }
        fds.push(fd);
    }

    Ok(EventGroup { fds })
}

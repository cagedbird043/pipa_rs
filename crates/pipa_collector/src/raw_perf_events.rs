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

//! This module provides a low-level interface to the `perf_event_open` syscall,
//! precisely mimicking the behavior of the `perf stat` command.

use crate::system_stats::PipaCollectorError;
use perf_event_open_sys as sys;
use std::io;
use std::os::unix::io::RawFd;

/// Represents a specific hardware performance event that can be monitored.
#[derive(Debug, Clone, Copy)]
pub enum PerfEvent {
    Cycles,
    Instructions,
}

impl PerfEvent {
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

/// A handle to a single performance counter, ensuring it is closed on drop.
#[derive(Debug)]
pub struct Counter {
    fd: RawFd,
}

impl Counter {
    pub fn fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

/// Creates a single, inheritable performance counter for a command to be executed.
/// This function precisely replicates the parameters used by `perf stat`.
pub fn create_counter_for_command(event: PerfEvent) -> Result<Counter, PipaCollectorError> {
    let mut attrs = sys::bindings::perf_event_attr {
        size: std::mem::size_of::<sys::bindings::perf_event_attr>() as u32,
        ..Default::default()
    };
    let (type_, config) = event.to_config();
    attrs.type_ = type_;
    attrs.config = config;

    // --- Settings copied exactly from `perf stat` strace ---
    attrs.set_disabled(1); // Start disabled.
    attrs.set_inherit(1); // Inherit to child processes.
    attrs.set_enable_on_exec(1); // Kernel will auto-enable on `execve`.

    // pid = 0: Monitor the current process. With inherit=1, this targets children.
    // cpu = -1: Monitor on any CPU the process runs on.
    // group_fd = -1: This is a standalone counter, not part of a group.
    // flags = 0: No special flags needed for this basic case.
    let fd = unsafe { sys::perf_event_open(&mut attrs, 0, -1, -1, 0) };

    if fd < 0 {
        let last_error = io::Error::last_os_error();
        return Err(PipaCollectorError::Io(io::Error::new(
            last_error.kind(),
            format!("perf_event_open failed for event {:?}: {}", event, last_error),
        )));
    }

    Ok(Counter { fd })
}

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

// TODO: Implement wrapper functions around the syscall.

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

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
#[ignore]
fn test_stat_runs_successfully_on_true() {
    let mut cmd = Command::cargo_bin("pipa_rs").unwrap();
    cmd.arg("stat")
        .arg("--")
        .arg("true") // `true` is a minimal command that does nothing and exits successfully.
        .assert()
        .success() // Chained method call
        .stdout(
            predicate::str::contains("Cycles")
                .and(predicate::str::contains("Instructions"))
                .and(predicate::str::contains("CPI")),
        );
}

#[test]
#[ignore]
fn test_stat_reports_error_for_nonexistent_command() {
    let mut cmd = Command::cargo_bin("pipa_rs").unwrap();
    cmd.arg("stat")
        .arg("--")
        .arg("this_command_does_not_exist_12345")
        .assert()
        .failure() // Chained method call
        .stderr(predicate::str::contains("Failed to execute command"));
}

#[test]
#[ignore]
fn test_stat_reports_error_if_no_command_is_given() {
    let mut cmd = Command::cargo_bin("pipa_rs").unwrap();
    cmd.arg("stat")
        .assert()
        .failure()
        // Corrected the usage string to include `--` as produced by clap.
        .stderr(predicate::str::contains("Usage: pipa_rs stat -- <COMMAND>..."));
}

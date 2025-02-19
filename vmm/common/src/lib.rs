/*
Copyright 2022 The Kuasar Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

pub use containerd_sandbox::data::Io;

pub mod api;
pub mod mount;
pub mod storage;

pub const KUASAR_STATE_DIR: &str = "/run/kuasar/state";

pub const IO_FILE_PREFIX: &str = "io";
pub const STORAGE_FILE_PREFIX: &str = "storage";
pub const SHARED_DIR_SUFFIX: &str = "shared";

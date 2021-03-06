// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(non_snake_case)]

mod conversion;
mod errors;
mod exception;
mod handle;
mod jni;
pub mod jni_cache;
mod pair_iter;
mod resource_manager;
mod services;
mod time_service;

pub use self::conversion::{convert_hash, convert_to_hash, convert_to_string};
pub use self::errors::{
    check_error_on_exception, describe_java_exception, get_and_clear_java_exception,
    panic_on_exception, unwrap_jni, unwrap_jni_verbose,
};
pub use self::exception::{any_to_string, unwrap_exc_or, unwrap_exc_or_default};
pub use self::handle::{as_handle, cast_handle, drop_handle, to_handle, Handle};
pub use self::jni::{get_class_name, get_exception_message};
pub use self::pair_iter::PairIter;
pub use self::resource_manager::known_handles;
pub use self::services::{
    is_service_enabled_in_config_file, load_enabled_services, BTC_ANCHORING_SERVICE,
    CONFIGURATION_SERVICE, EJB_SERVICE, PATH_TO_SERVICES_TO_ENABLE, TIME_SERVICE,
};

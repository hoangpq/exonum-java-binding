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

/// The main goal of resource manager is early detection of possible problems with resources
/// management and accessing objects with pointers. Not required in production.

#[cfg(not(feature = "resource-manager"))]
#[path = "stub.rs"]
mod imp;

#[cfg(feature = "resource-manager")]
#[path = "imp.rs"]
mod imp;

pub use self::imp::*;

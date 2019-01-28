// Copyright 2019 The Exonum Team
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

use exonum::storage::{Fork, Snapshot};
use exonum_time::schema::TimeSchema;
use jni::{
    objects::{JClass, JObject},
    sys::{jobjectArray, jsize},
    JNIEnv,
};
use std::{panic, ptr};
use storage::db::{View, ViewRef};
use utils::{self, Handle};

const BYTE_ARRAY: &str = "[B";

enum SchemaType {
    SnapshotSchema(TimeSchema<&'static Snapshot>),
    ForkSchema(TimeSchema<&'static mut Fork>),
}

/// Returns pointer to created CoreSchemaProxy object
#[no_mangle]
pub extern "system" fn Java_com_exonum_binding_time_TimeSchemaProxy_nativeCreate(
    env: JNIEnv,
    _: JClass,
    view_handle: Handle,
) -> Handle {
    let res = panic::catch_unwind(|| {
        let schema_type = match *utils::cast_handle::<View>(view_handle).get() {
            ViewRef::Snapshot(snapshot) => SchemaType::SnapshotSchema(TimeSchema::new(snapshot)),
            ViewRef::Fork(ref mut fork) => SchemaType::ForkSchema(TimeSchema::new(fork)),
        };
        Ok(utils::to_handle(schema_type))
    });
    utils::unwrap_exc_or_default(&env, res)
}

/// Destroys the underlying `Schema` object and frees memory.
#[no_mangle]
pub extern "system" fn Java_com_exonum_binding_time_TimeSchemaProxy_nativeFree(
    env: JNIEnv,
    _: JClass,
    schema_handle: Handle,
) {
    utils::drop_handle::<SchemaType>(&env, schema_handle);
}

/// Returns the height of the latest committed block. Throws `java.lang.RuntimeException` if the
/// "genesis block" has not been created yet.
#[no_mangle]
pub extern "system" fn Java_com_exonum_binding_time_TimeSchemaProxy_nativeGetStateHashes(
    env: JNIEnv,
    _: JClass,
    schema_handle: Handle,
) -> jobjectArray {
    let result = panic::catch_unwind(|| {
        let val = match utils::cast_handle::<SchemaType>(schema_handle) {
            SchemaType::SnapshotSchema(schema) => schema.state_hash(),
            SchemaType::ForkSchema(schema) => schema.state_hash(),
        };

        let java_hashes_array =
            env.new_object_array(val.len() as jsize, BYTE_ARRAY, JObject::null())?;
        for (i, hash) in val.into_iter().enumerate() {
            let java_hash = env.byte_array_from_slice(hash.as_ref())?.into();
            env.set_object_array_element(java_hashes_array, i as jsize, java_hash)?;
            env.delete_local_ref(java_hash)?;
        }
        Ok(java_hashes_array)
    });
    utils::unwrap_exc_or(&env, result, ptr::null_mut())
}

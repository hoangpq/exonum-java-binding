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

use exonum::blockchain::Blockchain;

use jni::objects::JClass;
use jni::sys::jboolean;
use jni::JNIEnv;

use parking_lot::{Once, ONCE_INIT};

const TIME_SERVICE_ID: u16 = 4;

static INIT: Once = ONCE_INIT;
static mut BLOCKCHAIN: Option<Blockchain> = None;

#[no_mangle]
pub extern "system" fn Java_com_exonum_binding_time_TimeSchemaImpl_isTimeServiceEnabled(
    _: JNIEnv,
    _: JClass,
) -> jboolean {
    is_time_service_enabled().into()
}

/// Initializes static context with reference to blockchain
pub fn init_with_blockchain(blockchain: Blockchain) {
    unsafe {
        INIT.call_once(|| BLOCKCHAIN = Some(blockchain));
    }
}

fn is_time_service_enabled() -> bool {
    INIT.state().done() &&
    unsafe { BLOCKCHAIN.clone() }.map_or(false, |blockchain|{
        blockchain.service_map().get(&TIME_SERVICE_ID).is_some()
    })
}

#[cfg(test)]
mod tests {
    extern crate futures;
    extern crate exonum_time;

    use super::*;
    use exonum::{blockchain::Service, crypto::gen_keypair, node::ApiSender, storage::MemoryDB};
//    use self::exonum_time::TimeService;
    use self::futures::{
        sync::mpsc,
    };

    fn create_blockchain(services: Vec<Box<dyn Service>>) -> Blockchain {
        let (public_key, secret_key) = gen_keypair();
        let api_channel = mpsc::channel(128);
        let app_tx= ApiSender::new(api_channel.0);
        let storage = MemoryDB::new();
        let blockchain = Blockchain::new(
            storage,
            services,
            public_key,
            secret_key,
            app_tx.clone(),
        );
        blockchain
    }

    #[test]
    fn is_not_initialised_with_blockchain() {
        assert!(!is_time_service_enabled());
    }

    #[test]
    fn initialized_no_time_service() {
        let blockchain = create_blockchain(vec![]);
        init_with_blockchain(blockchain);
        assert!(!is_time_service_enabled());
    }
// TODO: Fix compilation
//    #[test]
//    fn initialized_time_service_presented() {
//        let blockchain = create_blockchain(vec![Box::new(TimeService::new()) as Box<dyn Service>]);
//        init_with_blockchain(blockchain);
//        assert!(is_time_service_enabled());
//    }
}
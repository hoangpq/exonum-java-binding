/*
 * Copyright 2018 The Exonum Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.exonum.binding.cryptocurrency.transactions;

enum TransactionError {
  WALLET_ALREADY_EXISTS(0),
  UNKNOWN_RECEIVER(1),
  UNKNOWN_SENDER(2),
  INSUFFICIENT_FUNDS(3),
  SAME_SENDER_AND_RECEIVER(5);

  final byte errorCode;

  TransactionError(int errorCode) {
    this.errorCode = (byte) errorCode;
  }
}

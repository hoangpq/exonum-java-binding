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

package com.exonum.binding.blockchain.serialization;

import com.exonum.binding.blockchain.TransactionResult;
import com.exonum.binding.common.serialization.Serializer;
import com.google.protobuf.InvalidProtocolBufferException;

public enum TransactionResultSerializer implements Serializer<TransactionResult> {
  INSTANCE;

  @Override
  public byte[] toBytes(TransactionResult value) {
    int status = convertToCoreStatusCode(value);
    CoreProtos.TransactionResult txLocation =
        CoreProtos.TransactionResult.newBuilder()
            .setStatus(status)
            .setDescription(value.getErrorDescription().orElse(""))
            .build();
    return txLocation.toByteArray();
  }

  @Override
  public TransactionResult fromBytes(byte[] binaryTransactionResult) {
    try {
      CoreProtos.TransactionResult copiedtxLocationProtos =
          CoreProtos.TransactionResult.parseFrom(binaryTransactionResult);
      int status = copiedtxLocationProtos.getStatus();
      String description = copiedtxLocationProtos.getDescription();
      if (status <= 255) {
        return TransactionResult.error(status, description);
      } else if (status == 256) {
        return TransactionResult.successful();
      } else if (status == 257) {
        return TransactionResult.unexpectedError(description);
      } else {
        throw new InvalidProtocolBufferException("Invalid status code");
      }
    } catch (InvalidProtocolBufferException e) {
      throw new IllegalArgumentException("Unable to instantiate "
          + "CoreProtos.TransactionResult instance from provided binary data", e);
    }
  }

  private int convertToCoreStatusCode(TransactionResult transactionResult) {
    int status;
    switch (transactionResult.getType()) {
      case ERROR:
        status = transactionResult.getErrorCode().getAsInt();
        break;
      case SUCCESS:
        status = 256;
        break;
      case UNEXPECTED_ERROR:
        status = 257;
        break;
      default:
        throw new AssertionError("Unreachable");
    }
    return status;
  }

}

/*
 * Copyright 2019 The Exonum Team
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
 *
 */

package com.exonum.binding.common.serialization;

import static com.exonum.binding.common.serialization.SerializationUtils.checkLength;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.time.Instant;
import java.time.ZoneOffset;
import java.time.ZonedDateTime;

enum ZonedDateTimeSerializer implements Serializer<ZonedDateTime> {
  INSTANCE;

  static private final int BYTE_BUFFER_CAPACITY = Long.BYTES + Integer.BYTES;

  @Override
  public byte[] toBytes(ZonedDateTime value) {
    long seconds = value.toEpochSecond();
    int nanos = value.getNano();
    ByteBuffer buffer = ByteBuffer.allocate(BYTE_BUFFER_CAPACITY)
        .order(ByteOrder.LITTLE_ENDIAN);
    buffer.putLong(seconds);
    buffer.putInt(nanos);
    return buffer.array();
  }

  @Override
  public ZonedDateTime fromBytes(byte[] serializedValue) {
    checkLength(serializedValue, BYTE_BUFFER_CAPACITY);

    ByteBuffer buffer = ByteBuffer.wrap(serializedValue)
        .order(ByteOrder.LITTLE_ENDIAN);
    return retrieveZdtFromBuffer(buffer);
  }

  private ZonedDateTime retrieveZdtFromBuffer(ByteBuffer buffer) {
    long seconds = buffer.getLong();
    int nanos = buffer.getInt();
    Instant instant = Instant.ofEpochSecond(seconds, nanos);
    return ZonedDateTime.ofInstant(instant, ZoneOffset.UTC);
  }
}

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

import static com.exonum.binding.common.serialization.StandardSerializersTest.invalidBytesValueTest;
import static com.exonum.binding.common.serialization.StandardSerializersTest.roundTripTest;

import com.exonum.binding.test.Bytes;
import java.time.ZoneOffset;
import java.time.ZonedDateTime;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;

class ZonedDateTimeSerializerTest {
  private Serializer<ZonedDateTime> serializer = ZonedDateTimeSerializer.INSTANCE;

  @ParameterizedTest
  @MethodSource("testSource")
  void roundTrip(ZonedDateTime key) {
    roundTripTest(key, serializer);
  }

  @Test
  void deserializeInvalidValue() {
    byte[] invalidValue = Bytes.bytes();
    invalidBytesValueTest(invalidValue, serializer);
  }

  private static Stream<ZonedDateTime> testSource() {
    return Stream.of(
        ZonedDateTime.now(ZoneOffset.UTC),
        ZonedDateTime.of(2000, 1, 1, 1, 1, 1, 1, ZoneOffset.UTC));
  }
}

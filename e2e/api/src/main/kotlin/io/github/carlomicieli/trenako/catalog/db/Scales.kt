/*
 *   Copyright (c) 2022-2023 (C) Carlo Micieli
 *
 *    Licensed to the Apache Software Foundation (ASF) under one
 *    or more contributor license agreements.  See the NOTICE file
 *    distributed with this work for additional information
 *    regarding copyright ownership.  The ASF licenses this file
 *    to you under the Apache License, Version 2.0 (the
 *    "License"); you may not use this file except in compliance
 *    with the License.  You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing,
 *    software distributed under the License is distributed on an
 *    "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 *    KIND, either express or implied.  See the License for the
 *    specific language governing permissions and limitations
 *    under the License.
 */
package io.github.carlomicieli.trenako.catalog.db

import io.github.carlomicieli.trenako.model.ScaleGauge
import io.github.carlomicieli.trenako.model.ScaleStandard
import io.github.carlomicieli.trenako.model.TrackGauge
import java.math.BigDecimal
import java.time.Instant

object Scales {
    val H0: ScaleRow = ScaleRow(
        scaleId = "h0",
        name = "H0",
        ratio = BigDecimal(87),
        gauge = ScaleGauge().trackGauge(TrackGauge.STANDARD).inches(0.65f).millimeters(16.5f),
        standards = listOf(ScaleStandard.NEM),
        description = mapOf("en" to "description", "it" to "descrizione"),
        version = 0,
        createdAt = Instant.parse("2023-04-21T10:15:30.00Z"),
        lastModifiedAt = Instant.parse("2023-04-21T10:15:30.00Z")
    )
}

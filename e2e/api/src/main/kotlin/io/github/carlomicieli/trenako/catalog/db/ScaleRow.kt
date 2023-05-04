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
import org.springframework.data.annotation.Id
import org.springframework.data.relational.core.mapping.Table
import java.math.BigDecimal
import java.time.Instant

@Table("scales")
data class ScaleRow(
    @Id
    val scaleId: String,
    val name: String,
    val ratio: BigDecimal,
    val gaugeMillimeters: BigDecimal,
    val gaugeInches: BigDecimal,
    val trackGauge: TrackGauge,
    val descriptionEn: String?,
    val descriptionIt: String?,
    val standards: List<ScaleStandard>,
    val version: Int,
    val createdAt: Instant,
    val lastModifiedAt: Instant? = null
) {
    constructor(
        scaleId: String,
        name: String,
        ratio: BigDecimal,
        gauge: ScaleGauge,
        description: Map<String, String>? = null,
        standards: List<ScaleStandard>,
        version: Int = 1,
        createdAt: Instant = Instant.now(),
        lastModifiedAt: Instant? = null
    ) : this(
        scaleId,
        name,
        ratio,
        gauge.millimeters.toBigDecimal(),
        gauge.inches.toBigDecimal(),
        gauge.trackGauge,
        description?.get("en"),
        description?.get("it"),
        standards,
        version,
        createdAt,
        lastModifiedAt
    )
}

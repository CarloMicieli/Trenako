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
package io.github.carlomicieli.trenako.api.catalog

import io.github.carlomicieli.trenako.AbstractApiTest
import io.github.carlomicieli.trenako.ProblemDetailException
import io.github.carlomicieli.trenako.catalog.api.ScalesApi
import io.github.carlomicieli.trenako.model.Metadata
import io.github.carlomicieli.trenako.model.ScaleGauge
import io.github.carlomicieli.trenako.model.ScaleStandard
import io.github.carlomicieli.trenako.model.TrackGauge
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.reactor.awaitSingle
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.http.HttpStatusCode

@DisplayName("scales")
class ScalesWebApiTest : AbstractApiTest() {
    @Autowired
    lateinit var scalesApi: ScalesApi

    @Nested
    @DisplayName("GET /api/scales/{id}")
    inner class GetScalesById {
        @Test
        fun `it should read scales by their id1`() = runBlocking {
            val ex = shouldThrow<ProblemDetailException> {
                scalesApi.getScaleById("not-found").awaitSingle()
            }

            with(ex.problemDetail) {
                status shouldBe 404
                title shouldBe "The resource was not found"
            }
        }

        @Test
        fun `it should read scales by their id`() = runBlocking {
            val response = scalesApi.getScaleById("h0").awaitSingle()

            response.statusCode shouldBe HttpStatusCode.valueOf(200)

            val scale = response.body!!
            with(scale) {
                name shouldBe "H0"
                description shouldBe mapOf("en" to "description", "it" to "descrizione")
                ratio shouldBe 87.0f
                gauge shouldBe ScaleGauge().trackGauge(TrackGauge.STANDARD).inches(0.65f).millimeters(16.5f)
                standards shouldBe listOf(ScaleStandard.NEM)
                metadata shouldBe Metadata().version(0)
            }
        }
    }
}

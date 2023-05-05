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
import io.github.carlomicieli.trenako.catalog.api.RailwaysApi
import io.github.carlomicieli.trenako.catalog.db.RailwaysTable
import io.github.carlomicieli.trenako.model.ContactInfo
import io.github.carlomicieli.trenako.model.Metadata
import io.github.carlomicieli.trenako.model.OrganizationEntityType
import io.github.carlomicieli.trenako.model.RailwayPeriodOfActivity
import io.github.carlomicieli.trenako.model.RailwayRequest
import io.github.carlomicieli.trenako.model.RailwayStatus
import io.github.carlomicieli.trenako.model.RailwayTotalLength
import io.github.carlomicieli.trenako.model.Socials
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.reactor.awaitSingle
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.http.HttpStatus
import org.springframework.http.HttpStatusCode
import java.net.URI
import java.time.LocalDate

@DisplayName("railways")
class RailwaysWebApiTest : AbstractApiTest() {
    @Autowired
    lateinit var railwaysApi: RailwaysApi

    @Autowired
    lateinit var railwaysTable: RailwaysTable

    @Nested
    @DisplayName("POST /api/railways")
    inner class PostRailways {
        @Test
        fun `it should return 409 when the railway already exists`() = runBlocking {
            val railway = RailwayRequest()
            with(railway) {
                name = "FS"
                country = "IT"
                description = mapOf("en" to "description", "it" to "descrizione")
                headquarters = listOf()
            }

            val ex = shouldThrow<ProblemDetailException> {
                railwaysApi.postRailway(railway).awaitSingle()
            }

            with(ex.problemDetail) {
                detail shouldBe "The railway already exists (id: fs)"
                status shouldBe 409
                title shouldBe "The resource already exists"
                type shouldBe URI.create("https://httpstatuses.com/409")
            }
        }

        @Test
        fun `it should create new railways`() = runBlocking {
            val railway = RailwayRequest()
            with(railway) {
                abbreviation = "FS"
                name = "Ferrovie dello Stato"
                organizationEntityType = OrganizationEntityType.STATE_OWNED_ENTERPRISE
                country = "IT"
                periodOfActivity = RailwayPeriodOfActivity().operatingSince(LocalDate.of(1905, 7, 1)).status(
                    RailwayStatus.ACTIVE
                )
                headquarters = listOf("Rome")
                totalLength = RailwayTotalLength().kilometers(1000.0f).miles(621.4f)
                contactInfo = ContactInfo()
                    .email("mail@mail.com")
                    .phone("+14152370800")
                    .websiteUrl(URI.create("https://www.fs.com"))
                socials = Socials()
                    .facebook(URI.create("facebook_handler"))
                    .instagram(URI.create("instagram_handler"))
                    .linkedin(URI.create("linkedin_handler"))
                    .twitter(URI.create("twitter_handler"))
                    .youtube(URI.create("youtube_handler"))
            }

            val response = railwaysApi.postRailway(railway).awaitSingle()

            response.statusCode shouldBe HttpStatus.CREATED
            response.headers.location shouldBe URI.create("/api/railways/ferrovie-dello-stato")

            railwaysTable.existsByName("Ferrovie dello Stato") shouldBe true
        }
    }

    @Nested
    @DisplayName("GET /api/railways")
    inner class GetRailways {
        @Test
        fun `it should get all railways`() = runBlocking {
            val response = railwaysApi.getRailways().awaitSingle()

            response.statusCode shouldBe HttpStatusCode.valueOf(200)

            val railways = response.body.orEmpty()
            with(railways) {
                size shouldBe 1
            }

            val railway = railways.first()
            with(railway) {
                abbreviation shouldBe "FS"
                name shouldBe "Ferrovie dello Stato"
                organizationEntityType shouldBe OrganizationEntityType.STATE_OWNED_ENTERPRISE
                country shouldBe "IT"
                periodOfActivity shouldBe RailwayPeriodOfActivity().operatingSince(LocalDate.of(1905, 7, 1)).status(
                    RailwayStatus.ACTIVE
                )
                totalLength shouldBe RailwayTotalLength().kilometers(1000.0f).miles(621.4f)
                contactInfo shouldBe ContactInfo()
                    .email("mail@mail.com")
                    .phone("+14152370800")
                    .websiteUrl(URI.create("https://www.acmetreni.com"))
                socials shouldBe Socials()
                    .facebook(URI.create("facebook_handler"))
                    .instagram(URI.create("instagram_handler"))
                    .linkedin(URI.create("linkedin_handler"))
                    .twitter(URI.create("twitter_handler"))
                    .youtube(URI.create("youtube_handler"))
                metadata shouldBe Metadata().version(0)
            }
        }
    }

    @Nested
    @DisplayName("GET /api/railways/{id}")
    inner class GetRailwayById {
        @Test
        fun `it returns 400 when the railway with the given id is not found`() = runBlocking {
            val ex = shouldThrow<ProblemDetailException> {
                railwaysApi.getRailwayById("not-found").awaitSingle()
            }

            with(ex.problemDetail) {
                status shouldBe 404
                title shouldBe "The resource was not found"
            }
        }

        @Test
        fun `it should find a railway by its id`() = runBlocking {
            val response = railwaysApi.getRailwayById("fs").awaitSingle()

            response.statusCode shouldBe HttpStatusCode.valueOf(200)

            val railway = response.body!!
            with(railway) {
                abbreviation shouldBe "FS"
                name shouldBe "Ferrovie dello Stato"
                organizationEntityType shouldBe OrganizationEntityType.STATE_OWNED_ENTERPRISE
                country shouldBe "IT"
                periodOfActivity shouldBe RailwayPeriodOfActivity().operatingSince(LocalDate.of(1905, 7, 1)).status(
                    RailwayStatus.ACTIVE
                )
                totalLength shouldBe RailwayTotalLength().kilometers(1000.0f).miles(621.4f)
                contactInfo shouldBe ContactInfo()
                    .email("mail@mail.com")
                    .phone("+14152370800")
                    .websiteUrl(URI.create("https://www.acmetreni.com"))
                socials shouldBe Socials()
                    .facebook(URI.create("facebook_handler"))
                    .instagram(URI.create("instagram_handler"))
                    .linkedin(URI.create("linkedin_handler"))
                    .twitter(URI.create("twitter_handler"))
                    .youtube(URI.create("youtube_handler"))
                metadata shouldBe Metadata().version(0)
            }
        }
    }
}

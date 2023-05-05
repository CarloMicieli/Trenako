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
import io.github.carlomicieli.trenako.catalog.api.BrandsApi
import io.github.carlomicieli.trenako.catalog.db.BrandsTable
import io.github.carlomicieli.trenako.model.Address
import io.github.carlomicieli.trenako.model.BrandKind
import io.github.carlomicieli.trenako.model.BrandRequest
import io.github.carlomicieli.trenako.model.BrandStatus
import io.github.carlomicieli.trenako.model.ContactInfo
import io.github.carlomicieli.trenako.model.OrganizationEntityType
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

@DisplayName("brands")
class BrandsWebApiTest : AbstractApiTest() {

    @Autowired
    lateinit var brandsApi: BrandsApi

    @Autowired
    lateinit var brandsTable: BrandsTable

    @Nested
    @DisplayName("POST /api/brands")
    inner class PostBrands {
        @Test
        fun `it should return 409 when the brand already exists`() = runBlocking {
            val brand = BrandRequest()
            with(brand) {
                name = "ACME"
                kind = BrandKind.INDUSTRIAL
            }

            val ex = shouldThrow<ProblemDetailException> {
                brandsApi.postBrand(brand).awaitSingle()
            }

            with(ex.problemDetail) {
                detail shouldBe "The brand already exists (id: acme)"
                status shouldBe 409
                title shouldBe "The resource already exists"
                type shouldBe URI.create("https://httpstatuses.com/409")
            }
        }

        @Test
        fun `it should return 400 when the request is not valid`() = runBlocking {
            val brand = BrandRequest()
            with(brand) {
                name = "A"
                kind = BrandKind.INDUSTRIAL
            }

            val ex = shouldThrow<ProblemDetailException> {
                brandsApi.postBrand(brand).awaitSingle()
            }

            with(ex.problemDetail) {
                status shouldBe 400
                title shouldBe "Bad request"
                type shouldBe URI.create("https://httpstatuses.com/400")
            }
        }

        @Test
        fun `it should create new brands`() = runBlocking {
            val brand = BrandRequest()
            with(brand) {
                name = "Name"
                registeredCompanyName = "Registered Company Ltd"
                organizationEntityType = OrganizationEntityType.LIMITED_COMPANY
                groupName = "Group Name"
                description = mapOf("en" to "description", "it" to "descrizione")
                address = Address()
                    .streetAddress("Viale Lombardia, 27")
                    .city("Milano")
                    .country("IT")
                    .region("MI")
                    .postalCode("20131")
                contactInfo = ContactInfo()
                    .email("mail@mail.com")
                    .phone("+14152370800")
                    .websiteUrl(URI.create("https://www.acmetreni.com"))
                socials = Socials()
                    .facebook(URI.create("facebook_handler"))
                    .instagram(URI.create("instagram_handler"))
                    .linkedin(URI.create("linkedin_handler"))
                    .twitter(URI.create("twitter_handler"))
                    .youtube(URI.create("youtube_handler"))
                kind = BrandKind.BRASS_MODELS
                status = BrandStatus.ACTIVE
            }
            val response = brandsApi.postBrand(brand).awaitSingle()

            response.statusCode shouldBe HttpStatus.CREATED
            response.headers.location shouldBe URI.create("/api/brands/name")

            val brandRow = brandsTable.selectByName("Name") ?: throw AssertionError("row is not found")
            with(brandRow) {
                brandId shouldBe "name"
                name shouldBe "Name"
                registeredCompanyName shouldBe "Registered Company Ltd"
                organizationEntityType shouldBe OrganizationEntityType.LIMITED_COMPANY
                groupName shouldBe "Group Name"
                descriptionEn shouldBe "description"
                descriptionIt shouldBe "descrizione"
                addressStreetAddress shouldBe "Viale Lombardia, 27"
                addressCity shouldBe "Milano"
                addressCountry shouldBe "IT"
                addressRegion shouldBe "MI"
                addressPostalCode shouldBe "20131"
                contactEmail shouldBe "mail@mail.com"
                contactPhone shouldBe "+14152370800"
                contactWebsiteUrl shouldBe "https://www.acmetreni.com"
                socialsFacebook shouldBe "facebook_handler"
                socialsInstagram shouldBe "instagram_handler"
                socialsLinkedin shouldBe "linkedin_handler"
                socialsTwitter shouldBe "twitter_handler"
                socialsYoutube shouldBe "youtube_handler"
                kind shouldBe BrandKind.BRASS_MODELS
                status shouldBe BrandStatus.ACTIVE
            }
        }
    }

    @Nested
    @DisplayName("GET /api/brands")
    inner class GetBrands {
        @Test
        fun `it should get all brands`() = runBlocking {
            val response = brandsApi.getBrands().awaitSingle()

            response.statusCode shouldBe HttpStatusCode.valueOf(200)

            val brands = response.body.orEmpty()
            with(brands) {
                size shouldBe 1
            }

            val first = brands.first()
            with(first) {
                name shouldBe "ACME"
                registeredCompanyName shouldBe "Associazione Costruzioni Modellistiche Esatte"
                organizationEntityType shouldBe OrganizationEntityType.LIMITED_COMPANY
                groupName shouldBe "UNKNOWN"
                description shouldBe mapOf("en" to "description", "it" to "descrizione")
                address shouldBe Address()
                    .streetAddress("Viale Lombardia, 27")
                    .city("Milano")
                    .country("IT")
                    .region("MI")
                    .postalCode("20131")
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
                kind shouldBe BrandKind.INDUSTRIAL
                status shouldBe BrandStatus.ACTIVE
            }
        }
    }

    @Nested
    @DisplayName("GET /api/brands/{id}")
    inner class GetBrandById {
        @Test
        fun `it returns 400 when the brand with the given id is not found`() = runBlocking {
            val ex = shouldThrow<ProblemDetailException> {
                brandsApi.getBrandById("not-found").awaitSingle()
            }

            with(ex.problemDetail) {
                detail shouldBe "/api/brands/not-found"
                status shouldBe 404
                title shouldBe "The resource was not found"
                type shouldBe URI.create("https://httpstatuses.com/404")
            }
        }

        @Test
        fun `it should find a brand by its id`() = runBlocking {
            val response = brandsApi.getBrandById("acme").awaitSingle()

            response.statusCode shouldBe HttpStatusCode.valueOf(200)

            val brand = response.body!!
            with(brand) {
                name shouldBe "ACME"
                registeredCompanyName shouldBe "Associazione Costruzioni Modellistiche Esatte"
                organizationEntityType shouldBe OrganizationEntityType.LIMITED_COMPANY
                groupName shouldBe "UNKNOWN"
                description shouldBe mapOf("en" to "description", "it" to "descrizione")
                address shouldBe Address()
                    .streetAddress("Viale Lombardia, 27")
                    .city("Milano")
                    .country("IT")
                    .region("MI")
                    .postalCode("20131")
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
                kind shouldBe BrandKind.INDUSTRIAL
                status shouldBe BrandStatus.ACTIVE
            }
        }
    }
}

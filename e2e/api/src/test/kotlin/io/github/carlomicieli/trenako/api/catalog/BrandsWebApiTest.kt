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
import io.github.carlomicieli.trenako.catalog.BrandsApi
import io.github.carlomicieli.trenako.database.BrandsRepository
import io.github.carlomicieli.trenako.fake.FakeData
import io.github.carlomicieli.trenako.model.BrandKind
import io.github.carlomicieli.trenako.model.BrandRequest
import io.github.carlomicieli.trenako.model.BrandStatus
import io.github.carlomicieli.trenako.model.OrganizationEntityType
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
    lateinit var brandsRepository: BrandsRepository

    @Nested
    @DisplayName("POST /brands")
    inner class PostBrands {
        @Test
        fun `it should create new brands`() = runBlocking {
            val brand = BrandRequest()
            with(brand) {
                name = "Name"
                registeredCompanyName = "Registered Company Ltd"
                organizationEntityType = OrganizationEntityType.LIMITED_COMPANY
                groupName = "Group Name"
                description = FakeData.localized("English description", "Descrizione in Italiano")
                address = FakeData.address()
                contactInfo = FakeData.contactInfo()
                socials = FakeData.socials()
                kind = BrandKind.BRASS_MODELS
                status = BrandStatus.ACTIVE
            }
            val response = brandsApi.postBrand(brand).awaitSingle()

            response.statusCode shouldBe HttpStatus.CREATED
            response.headers.location shouldBe URI.create("/api/brands/name")

            // val result = brandsRepository.findById("name").awaitSingle()
            // result?.name shouldBe "Name"
            // brandsTable.rowExistsWithName("Name") shouldBe true
        }
    }

    @Nested
    @DisplayName("GET /brands")
    inner class GetBrands {
        @Test
        fun `it should read brands`() = runBlocking {
            val response = brandsApi.getBrands().awaitSingle()

            response.statusCode shouldBe HttpStatusCode.valueOf(200)

            val brands = response.body.orEmpty()
            with(brands) {
                size shouldBe 5
            }

            val first = brands.first()
            with(first) {
                name shouldBe "ACME"
                registeredCompanyName shouldBe "Associazione Costruzioni Modellistiche Esatte"
                organizationEntityType shouldBe OrganizationEntityType.LIMITED_COMPANY
                groupName shouldBe "UNKNOWN"
                description shouldBe FakeData.localized("description", "descrizione")
                address?.country shouldBe "IT"
                contactInfo?.websiteUrl shouldBe URI.create("https://www.acmetreni.com")
                socials shouldBe FakeData.socials()
                kind shouldBe BrandKind.INDUSTRIAL
                status shouldBe BrandStatus.ACTIVE
            }
        }
    }

    @Nested
    @DisplayName("GET /brands/{id}")
    inner class GetBrandById {
        @Test
        fun `it should read brands by their id`() = runBlocking {
            val ex = shouldThrow<ProblemDetailException> {
                brandsApi.getBrandById("not-found").awaitSingle()
            }

            with(ex.problemDetail) {
                status shouldBe 404
                title shouldBe "The resource was not found"
            }
        }
    }
}

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
package io.github.carlomicieli.trenako.database.seeding

import io.github.carlomicieli.trenako.database.BrandRow
import io.github.carlomicieli.trenako.database.BrandsRepository
import io.github.carlomicieli.trenako.model.Address
import io.github.carlomicieli.trenako.model.BrandKind
import io.github.carlomicieli.trenako.model.BrandStatus
import io.github.carlomicieli.trenako.model.ContactInfo
import io.github.carlomicieli.trenako.model.OrganizationEntityType
import io.github.carlomicieli.trenako.model.Socials
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Component
import java.net.URI
import java.time.Instant

@Component
class Brands(val brandsRepository: BrandsRepository) {
    companion object {
        val LOG: Logger = LoggerFactory.getLogger(Brands::class.java)

        val ACME: BrandRow = BrandRow(
            brandId = "acme",
            name = "ACME",
            registeredCompanyName = "Associazione Costruzioni Modellistiche Esatte",
            organizationEntityType = OrganizationEntityType.LIMITED_COMPANY,
            groupName = "UNKNOWN",
            description = mapOf("it" to "descrizione", "en" to "description"),
            kind = BrandKind.INDUSTRIAL,
            status = BrandStatus.ACTIVE,
            contacts = ContactInfo()
                .email("mail@mail.com")
                .phone("+14152370800")
                .websiteUrl(URI.create("https://www.acmetreni.com")),
            address = Address()
                .streetAddress("Viale Lombardia, 27")
                .city("Milano")
                .country("IT")
                .region("MI")
                .postalCode("20131"),
            socials = Socials()
                .facebook(URI.create("facebook_handler"))
                .instagram(URI.create("instagram_handler"))
                .linkedin(URI.create("linkedin_handler"))
                .twitter(URI.create("twitter_handler"))
                .youtube(URI.create("youtube_handler")),
            version = 0,
            createdAt = Instant.parse("2023-04-21T10:15:30.00Z"),
            lastModifiedAt = Instant.parse("2023-04-21T10:15:30.00Z")
        )
    }

    suspend fun seed() {
        LOG.info("Seeding the brands")

        brandsRepository.insert(ACME)
        LOG.info("Inserted 1 brand...")
    }
}

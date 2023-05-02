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
package io.github.carlomicieli.trenako.database

import io.github.carlomicieli.trenako.model.Address
import io.github.carlomicieli.trenako.model.BrandKind
import io.github.carlomicieli.trenako.model.BrandStatus
import io.github.carlomicieli.trenako.model.ContactInfo
import io.github.carlomicieli.trenako.model.OrganizationEntityType
import io.github.carlomicieli.trenako.model.Socials
import org.springframework.data.annotation.Id
import org.springframework.data.relational.core.mapping.Table
import java.time.Instant

@Table("brands")
data class BrandRow(
    @Id
    val brandId: String,
    val name: String,
    val registeredCompanyName: String?,
    val organizationEntityType: OrganizationEntityType?,
    val groupName: String?,
    val descriptionEn: String?,
    val descriptionIt: String?,
    val kind: BrandKind,
    val contactPhone: String?,
    val contactWebsiteUrl: String?,
    val contactEmail: String?,

    val addressStreetAddress: String?,
    val addressExtendedAddress: String?,
    val addressCity: String?,
    val addressRegion: String?,
    val addressPostalCode: String?,
    val addressCountry: String?,

    val socialsFacebook: String?,
    val socialsInstagram: String?,
    val socialsLinkedin: String?,
    val socialsTwitter: String?,
    val socialsYoutube: String?,

    val status: BrandStatus?,
    val version: Int,
    val createdAt: Instant,
    val lastModifiedAt: Instant? = null
) {
    constructor(
        brandId: String,
        name: String,
        registeredCompanyName: String?,
        organizationEntityType: OrganizationEntityType?,
        groupName: String?,
        description: Map<String, String>?,
        kind: BrandKind = BrandKind.INDUSTRIAL,
        status: BrandStatus? = BrandStatus.ACTIVE,
        contacts: ContactInfo?,
        address: Address?,
        socials: Socials?,
        version: Int = 1,
        createdAt: Instant = Instant.now(),
        lastModifiedAt: Instant? = null
    ) : this(
        brandId,
        name,
        registeredCompanyName,
        organizationEntityType,
        groupName,
        description?.get("en"),
        description?.get("it"),
        kind,
        contacts?.phone,
        contacts?.websiteUrl?.toString(),
        contacts?.email,
        address?.streetAddress,
        address?.extendedAddress,
        address?.city,
        address?.region,
        address?.postalCode,
        address?.country,
        socials?.facebook?.toString(),
        socials?.instagram?.toString(),
        socials?.linkedin?.toString(),
        socials?.twitter?.toString(),
        socials?.youtube?.toString(),
        status,
        version,
        createdAt,
        lastModifiedAt
    )
}

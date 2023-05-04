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

import io.github.carlomicieli.trenako.model.ContactInfo
import io.github.carlomicieli.trenako.model.OrganizationEntityType
import io.github.carlomicieli.trenako.model.RailwayGauge
import io.github.carlomicieli.trenako.model.RailwayPeriodOfActivity
import io.github.carlomicieli.trenako.model.RailwayStatus
import io.github.carlomicieli.trenako.model.RailwayTotalLength
import io.github.carlomicieli.trenako.model.Socials
import io.github.carlomicieli.trenako.model.TrackGauge
import org.springframework.data.annotation.Id
import org.springframework.data.relational.core.mapping.Table
import java.math.BigDecimal
import java.time.Instant
import java.time.LocalDate

@Table("railways")
data class RailwayRow(
    @Id
    val railwayId: String,
    val name: String,
    val abbreviation: String?,
    val registeredCompanyName: String?,
    val organizationEntityType: OrganizationEntityType?,
    val descriptionEn: String?,
    val descriptionIt: String?,
    val country: String,
    val operatingSince: LocalDate?,
    val operatingUntil: LocalDate?,
    val status: RailwayStatus?,
    val gaugeMeters: BigDecimal?,
    val trackGauge: TrackGauge?,
    val headquarters: List<String>?,
    val totalLengthMi: BigDecimal?,
    val totalLengthKm: BigDecimal?,
    val contactPhone: String?,
    val contactWebsiteUrl: String?,
    val contactEmail: String?,
    val socialsFacebook: String?,
    val socialsInstagram: String?,
    val socialsLinkedin: String?,
    val socialsTwitter: String?,
    val socialsYoutube: String?,
    val version: Int,
    val createdAt: Instant,
    val lastModifiedAt: Instant? = null
) {
    constructor(
        railwayId: String,
        name: String,
        country: String,
        abbreviation: String? = null,
        registeredCompanyName: String? = null,
        organizationEntityType: OrganizationEntityType? = null,
        description: Map<String, String>? = null,
        railwayPeriodOfActivity: RailwayPeriodOfActivity? = null,
        railwayGauge: RailwayGauge? = null,
        railwayTotalLength: RailwayTotalLength? = null,
        headquarters: List<String>? = null,
        contacts: ContactInfo? = null,
        socials: Socials? = null,
        version: Int = 1,
        createdAt: Instant = Instant.now(),
        lastModifiedAt: Instant? = null
    ) : this(
        railwayId,
        name,
        abbreviation,
        registeredCompanyName,
        organizationEntityType,
        description?.get("en"),
        description?.get("it"),
        country,
        railwayPeriodOfActivity?.operatingSince,
        railwayPeriodOfActivity?.operatingUntil,
        railwayPeriodOfActivity?.status,
        railwayGauge?.meters?.toBigDecimal(),
        railwayGauge?.trackGauge,
        headquarters,
        railwayTotalLength?.kilometers?.toBigDecimal(),
        railwayTotalLength?.miles?.toBigDecimal(),
        contacts?.phone,
        contacts?.websiteUrl?.toString(),
        contacts?.email,
        socials?.facebook?.toString(),
        socials?.instagram?.toString(),
        socials?.linkedin?.toString(),
        socials?.twitter?.toString(),
        socials?.youtube?.toString(),
        version,
        createdAt,
        lastModifiedAt
    )
}

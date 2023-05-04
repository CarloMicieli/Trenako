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
import java.net.URI
import java.time.Instant
import java.time.LocalDate

object Railways {
    val FS: RailwayRow = RailwayRow(
        railwayId = "fs",
        name = "Ferrovie dello Stato",
        country = "IT",
        abbreviation = "FS",
        organizationEntityType = OrganizationEntityType.STATE_OWNED_ENTERPRISE,
        contacts = ContactInfo()
            .email("mail@mail.com")
            .phone("+14152370800")
            .websiteUrl(URI.create("https://www.acmetreni.com")),
        socials = Socials()
            .facebook(URI.create("facebook_handler"))
            .instagram(URI.create("instagram_handler"))
            .linkedin(URI.create("linkedin_handler"))
            .twitter(URI.create("twitter_handler"))
            .youtube(URI.create("youtube_handler")),
        description = mapOf("en" to "description", "it" to "descrizione"),
        railwayGauge = RailwayGauge().trackGauge(TrackGauge.STANDARD).meters(1.435f),
        railwayTotalLength = RailwayTotalLength().kilometers(1000f).miles(621.371f),
        railwayPeriodOfActivity = RailwayPeriodOfActivity().operatingSince(LocalDate.of(1905, 7, 1)).status(
            RailwayStatus.ACTIVE
        ),
        headquarters = listOf("Roma"),
        version = 0,
        createdAt = Instant.parse("2023-04-21T10:15:30.00Z"),
        lastModifiedAt = Instant.parse("2023-04-21T10:15:30.00Z")
    )
}

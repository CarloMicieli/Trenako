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
package io.github.carlomicieli.trenako.fake

import io.github.carlomicieli.trenako.model.Address
import io.github.carlomicieli.trenako.model.ContactInfo
import io.github.carlomicieli.trenako.model.Socials
import java.net.URI

object FakeData {
    fun localized(english: String?, italian: String?): Map<String, String> {
        val map = mutableMapOf<String, String>()

        english?.let { map.put("en", it) }
        italian?.let { map.put("it", it) }

        return map
    }

    fun address(): Address {
        val address = Address()
        with(address) {
            streetAddress = "Rue Morgue 22"
            extendedAddress = "Apartment 42"
            postalCode = "1H2 4BB"
            city = "London"
            region = "Essex"
            country = "GB"
        }
        return address
    }

    fun contactInfo(): ContactInfo {
        val contactInfo = ContactInfo()
        with(contactInfo) {
            email = "mail@mail.com"
            phone = "+14152370800"
            websiteUrl = URI.create("https://www.site.com")
        }
        return contactInfo
    }

    fun socials(): Socials {
        val socials = Socials()
        with(socials) {
            facebook = URI.create("facebook_handler")
            instagram = URI.create("instagram_handler")
            linkedin = URI.create("linkedin_handler")
            twitter = URI.create("twitter_handler")
            youtube = URI.create("youtube_handler")
        }
        return socials
    }
}

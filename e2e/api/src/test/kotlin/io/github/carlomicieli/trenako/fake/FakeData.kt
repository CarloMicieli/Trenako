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

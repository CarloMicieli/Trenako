package io.github.carlomicieli.trenako.api.catalog

import io.github.carlomicieli.trenako.AbstractApiTest
import io.github.carlomicieli.trenako.ProblemDetailException
import io.github.carlomicieli.trenako.catalog.BrandsApi
import io.github.carlomicieli.trenako.database.BrandsTable
import io.github.carlomicieli.trenako.fake.FakeData
import io.github.carlomicieli.trenako.model.BrandKind
import io.github.carlomicieli.trenako.model.BrandRequest
import io.github.carlomicieli.trenako.model.BrandStatus
import io.github.carlomicieli.trenako.model.OrganizationEntityType
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.reactor.awaitSingle
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.Disabled
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

            brandsTable.rowExistsWithName("Name") shouldBe true
        }
    }

    @Disabled
    @Nested
    @DisplayName("GET /brands")
    inner class GetBrands {
        @Test
        fun `it should read brands`() = runBlocking {
            val response = brandsApi.getBrands().awaitSingle()

            response.statusCode shouldBe HttpStatusCode.valueOf(204)
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

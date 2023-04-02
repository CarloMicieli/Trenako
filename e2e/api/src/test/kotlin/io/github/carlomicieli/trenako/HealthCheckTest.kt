package io.github.carlomicieli.trenako

import io.kotest.matchers.shouldBe
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.Test
import org.springframework.web.reactive.function.client.awaitExchange

class HealthCheckTest : AbstractApiTest() {
    @Test
    fun `it should run the api health check`() = runBlocking {
        webClient.get()
            .uri("/health-check")
            .awaitExchange {
                it.statusCode().is2xxSuccessful shouldBe true
            }
    }
}

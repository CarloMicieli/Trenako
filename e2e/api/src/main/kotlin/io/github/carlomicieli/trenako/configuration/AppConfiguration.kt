package io.github.carlomicieli.trenako.configuration

import io.github.carlomicieli.trenako.ProblemDetailException
import io.github.carlomicieli.trenako.catalog.BrandsApi
import io.github.carlomicieli.trenako.model.ProblemDetail
import org.springframework.boot.context.properties.EnableConfigurationProperties
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.http.HttpStatusCode
import org.springframework.http.client.reactive.ReactorClientHttpConnector
import org.springframework.web.reactive.function.client.ClientResponse
import org.springframework.web.reactive.function.client.WebClient
import org.springframework.web.reactive.function.client.support.WebClientAdapter
import org.springframework.web.service.invoker.HttpServiceProxyFactory
import reactor.core.publisher.Mono
import reactor.netty.http.client.HttpClient
import reactor.netty.resources.ConnectionProvider
import java.time.Duration

@Configuration
@EnableConfigurationProperties(ApiConfiguration::class)
class AppConfiguration {

    @Bean
    fun webClient(builder: WebClient.Builder, configuration: ApiConfiguration): WebClient {
        val provider = ConnectionProvider.builder("e2e")
            .maxConnections(50)
            .maxIdleTime(Duration.ofSeconds(20))
            .maxLifeTime(Duration.ofSeconds(60))
            .pendingAcquireTimeout(Duration.ofSeconds(60))
            .evictInBackground(Duration.ofSeconds(120))
            .build()

        val httpClient: HttpClient = HttpClient.create(provider)
        return builder
            .clientConnector(ReactorClientHttpConnector(httpClient))
            .defaultStatusHandler(HttpStatusCode::is4xxClientError, this::mapException)
            .defaultStatusHandler(HttpStatusCode::is5xxServerError, this::mapException)
            .baseUrl(configuration.endpointUrl)
            .build()
    }

    private fun mapException(clientResponse: ClientResponse): Mono<out Throwable> {
        return clientResponse.bodyToMono(ProblemDetail::class.java)
            .map { ProblemDetailException(it) }
    }

    @Bean
    fun httpServiceProxyFactory(webClient: WebClient): HttpServiceProxyFactory {
        return HttpServiceProxyFactory
            .builder(WebClientAdapter.forClient(webClient))
            .build()
    }

    @Bean
    fun brandsApi(httpServiceProxyFactory: HttpServiceProxyFactory): BrandsApi {
        return httpServiceProxyFactory.createClient(BrandsApi::class.java)
    }
}

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
@EnableConfigurationProperties(ApiServerConfiguration::class)
class AppConfiguration {

    @Bean
    fun webClient(builder: WebClient.Builder, configuration: ApiServerConfiguration): WebClient {
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

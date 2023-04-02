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
package io.github.carlomicieli.trenako

import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.context.SpringBootTest
import org.springframework.test.context.ActiveProfiles
import org.springframework.test.context.DynamicPropertyRegistry
import org.springframework.test.context.DynamicPropertySource
import org.springframework.web.reactive.function.client.WebClient
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.Network
import org.testcontainers.containers.PostgreSQLContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.containers.wait.strategy.Wait
import org.testcontainers.images.PullPolicy
import org.testcontainers.junit.jupiter.Testcontainers

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.NONE, properties = ["spring.main.banner-mode=off"])
@ActiveProfiles("e2e")
@Testcontainers
abstract class AbstractApiTest {
    companion object {
        private val LOGGER: Logger = LoggerFactory.getLogger("api-e2e-test")

        private val network = Network.newNetwork()

        private const val POSTGRES_IMAGE = "postgres:15.2-alpine"
        private const val TRENAKO_SERVER_IMAGE = "carlomicieli/trenako-server"
        private const val TRENAKO_MIGRATIONS_IMAGE = "carlomicieli/trenako-migrations"

        private const val USERNAME: String = "postgres"
        private const val PASSWORD: String = "mysecretpassword"
        private const val DATABASE_HOST: String = "db"
        private const val DATABASE_NAME: String = "trenakodb"
        private const val DATABASE_PORT: Int = 5432

        private val postgresContainer: PostgreSQLContainer<*> = PostgreSQLContainer(POSTGRES_IMAGE)
            .withDatabaseName(DATABASE_NAME)
            .withUsername(USERNAME)
            .withPassword(PASSWORD)
            .withExposedPorts(DATABASE_PORT)
            .withNetwork(network)
            .withNetworkAliases("db")

        private val migrationsContainer: GenericContainer<*> = GenericContainer(migrationsImageName())
            .withEnv("DATABASE_URL", "postgresql://$USERNAME:$PASSWORD@$DATABASE_HOST:$DATABASE_PORT/$DATABASE_NAME")
            .withNetwork(network)
            .withNetworkAliases("db_migrations")
            .dependsOn(postgresContainer)
            .withImagePullPolicy(PullPolicy.defaultPolicy())

        private val serverContainer: GenericContainer<*> = GenericContainer(serverImageName())
            .withEnv("DATABASE__HOST", DATABASE_HOST)
            .withEnv("DATABASE__PORT", DATABASE_PORT.toString())
            .withEnv("DATABASE__NAME", DATABASE_NAME)
            .withEnv("DATABASE__USERNAME", USERNAME)
            .withEnv("DATABASE__PASSWORD", PASSWORD)
            .withExposedPorts(5000)
            .withNetwork(network)
            .withNetworkAliases("server")
            .waitingFor(Wait.forHttp("/health-check"))
            .dependsOn(postgresContainer, migrationsContainer)
            .withImagePullPolicy(PullPolicy.defaultPolicy())

        init {
            postgresContainer.start()
            postgresContainer.followOutput(Slf4jLogConsumer(LOGGER))

            migrationsContainer.start()
            migrationsContainer.followOutput(Slf4jLogConsumer(LOGGER))

            serverContainer.start()
            serverContainer.followOutput(Slf4jLogConsumer(LOGGER))
        }

        @DynamicPropertySource
        @JvmStatic
        fun registerDynamicProperties(registry: DynamicPropertyRegistry) {
            with(registry) {
                add("api.server.endpointUrl") {
                    val port = serverContainer.getMappedPort(5000)
                    "http://localhost:$port"
                }

                add("spring.datasource.url") {
                    postgresContainer.jdbcUrl
                }
                add("spring.datasource.username") {
                    postgresContainer.username
                }
                add("spring.datasource.password") {
                    postgresContainer.password
                }

                add("spring.r2dbc.url") {
                    postgresContainer.jdbcUrl.replace("jdbc:", "r2dbc:")
                }
                add("spring.r2dbc.username") {
                    postgresContainer.username
                }
                add("spring.r2dbc.password") {
                    postgresContainer.password
                }
            }
        }

        private fun migrationsImageName(): String {
            val registry: String = System.getenv("REGISTRY_NAME") ?: "ghr.io"
            val tag: String = System.getenv("TAG") ?: "latest"
            return "$registry/$TRENAKO_MIGRATIONS_IMAGE:$tag"
        }

        private fun serverImageName(): String {
            val registry: String = System.getenv("REGISTRY_NAME") ?: "ghr.io"
            val tag: String = System.getenv("TAG") ?: "latest"
            return "$registry/$TRENAKO_SERVER_IMAGE:$tag"
        }
    }

    @Autowired
    lateinit var webClient: WebClient
}

package io.github.carlomicieli.trenako

import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.boot.test.context.SpringBootTest
import org.springframework.http.client.reactive.ReactorClientHttpConnector
import org.springframework.test.context.ActiveProfiles
import org.springframework.web.reactive.function.client.WebClient
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.Network
import org.testcontainers.containers.PostgreSQLContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.containers.wait.strategy.Wait
import org.testcontainers.junit.jupiter.Testcontainers
import reactor.netty.http.client.HttpClient
import reactor.netty.resources.ConnectionProvider
import java.time.Duration

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.NONE, properties = ["spring.main.banner-mode=off"])
@ActiveProfiles("e2e")
@Testcontainers
abstract class AbstractApiTest {
    companion object {
        private val LOGGER: Logger = LoggerFactory.getLogger("api-e2e-test")

        private val network = Network.newNetwork()

        private const val POSTGRES_IMAGE = "postgres:15.2-alpine"
        private const val TRENAKO_SERVER_IMAGE = "ghr.io/carlomicieli/trenako-server:latest"

        private const val USERNAME: String = "postgres"
        private const val PASSWORD: String = "mysecretpassword"
        private const val DATABASE_NAME: String = "trenakodb"

        private val postgresContainer: PostgreSQLContainer<*> = PostgreSQLContainer(POSTGRES_IMAGE)
            .withDatabaseName(DATABASE_NAME)
            .withUsername(USERNAME)
            .withPassword(PASSWORD)
            .withExposedPorts(5432)
            .withNetwork(network)
            .withNetworkAliases("db")
            .withInitScript("init-db/init_schema.sql")

        private val serverContainer: GenericContainer<*> = GenericContainer(TRENAKO_SERVER_IMAGE)
            .withEnv("DATABASE__HOST", "db")
            .withEnv("DATABASE__PORT", "5432")
            .withEnv("DATABASE__NAME", DATABASE_NAME)
            .withEnv("DATABASE__USERNAME", USERNAME)
            .withEnv("DATABASE__PASSWORD", PASSWORD)
            .withExposedPorts(5000)
            .withNetwork(network)
            .withNetworkAliases("server")
            .waitingFor(Wait.forHttp("/health-check"))
            .dependsOn(postgresContainer)

        val webClient: WebClient

        init {
            postgresContainer.start()
            postgresContainer.followOutput(Slf4jLogConsumer(LOGGER))

            serverContainer.start()
            serverContainer.followOutput(Slf4jLogConsumer(LOGGER))

            val provider = ConnectionProvider.builder("e2e")
                .maxConnections(50)
                .maxIdleTime(Duration.ofSeconds(20))
                .maxLifeTime(Duration.ofSeconds(60))
                .pendingAcquireTimeout(Duration.ofSeconds(60))
                .evictInBackground(Duration.ofSeconds(120))
                .build()

            val httpClient: HttpClient = HttpClient.create(provider)
            val port = serverContainer.getMappedPort(5000)
            webClient = WebClient.builder()
                .clientConnector(ReactorClientHttpConnector(httpClient))
                .baseUrl("http://localhost:$port")
                .build()
        }
    }
}

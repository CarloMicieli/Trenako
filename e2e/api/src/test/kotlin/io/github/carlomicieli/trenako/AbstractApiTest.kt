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

        private val serverContainer: GenericContainer<*> = GenericContainer(serverImageName())
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
            .withImagePullPolicy(PullPolicy.defaultPolicy())

        init {
            postgresContainer.start()
            postgresContainer.followOutput(Slf4jLogConsumer(LOGGER))

            serverContainer.start()
            serverContainer.followOutput(Slf4jLogConsumer(LOGGER))
        }

        @DynamicPropertySource
        @JvmStatic
        fun registerDynamicProperties(registry: DynamicPropertyRegistry) {
            with(registry) {
                add("app.endpointUrl") {
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
            }
        }

        fun serverImageName(): String {
            val registry: String = System.getenv("REGISTRY_NAME") ?: "ghr.io"
            val tag: String = System.getenv("TAG") ?: "latest"
            return "$registry/$TRENAKO_SERVER_IMAGE:$tag"
        }
    }

    @Autowired
    lateinit var webClient: WebClient
}

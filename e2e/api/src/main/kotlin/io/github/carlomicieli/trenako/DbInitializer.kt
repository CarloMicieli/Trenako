package io.github.carlomicieli.trenako

import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.boot.CommandLineRunner
import org.springframework.core.io.DefaultResourceLoader
import org.springframework.core.io.Resource
import org.springframework.core.io.ResourceLoader
import org.springframework.jdbc.core.JdbcTemplate
import org.springframework.jdbc.datasource.init.ResourceDatabasePopulator
import org.springframework.stereotype.Component
import javax.sql.DataSource

@Suppress("SqlResolve", "SqlNoDataSourceInspection")
@Component
class DbInitializer(private val dataSource: DataSource, private val jdbcTemplate: JdbcTemplate) : CommandLineRunner {

    companion object {
        val LOG: Logger = LoggerFactory.getLogger(DbInitializer::class.java)
    }

    override fun run(vararg args: String?) {
        LOG.info("Running the database initializer...")

        val count: Int = jdbcTemplate.queryForObject("SELECT COUNT(*) FROM brands", Int::class.java) ?: 0
        if (count > 0) {
            LOG.info("Database contains already data. Nothing to do here")
            return
        }

        val resourceLoader: ResourceLoader = DefaultResourceLoader()
        val scripts: Array<Resource> = arrayOf(
            resourceLoader.getResource("classpath:init-db/seed/brands.sql")
        )

        ResourceDatabasePopulator(*scripts).execute(dataSource)
    }
}

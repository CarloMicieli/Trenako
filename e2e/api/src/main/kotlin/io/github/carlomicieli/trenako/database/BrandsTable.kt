package io.github.carlomicieli.trenako.database

import org.springframework.jdbc.core.JdbcTemplate
import org.springframework.stereotype.Component

@Suppress("SqlResolve", "SqlNoDataSourceInspection")
@Component
class BrandsTable(private val jdbcTemplate: JdbcTemplate) {

    fun rowExistsWithName(name: String): Boolean {
        val count = jdbcTemplate.queryForObject("SELECT COUNT(*) FROM brands WHERE name = ?", Int::class.java, name)
        return count == 1
    }
}

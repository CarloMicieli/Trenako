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
package io.github.carlomicieli.trenako.database

import io.netty.util.internal.StringUtil
import io.r2dbc.pool.ConnectionPool
import io.r2dbc.pool.ConnectionPoolConfiguration
import io.r2dbc.postgresql.PostgresqlConnectionConfiguration
import io.r2dbc.postgresql.PostgresqlConnectionFactory
import io.r2dbc.spi.ConnectionFactoryOptions
import io.r2dbc.spi.Option
import org.springframework.boot.autoconfigure.r2dbc.R2dbcProperties
import org.springframework.boot.context.properties.PropertyMapper
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.data.r2dbc.config.AbstractR2dbcConfiguration
import org.springframework.data.r2dbc.repository.config.EnableR2dbcRepositories

@Configuration
@EnableR2dbcRepositories
class R2dbcConfiguration(val r2dbcProperties: R2dbcProperties) : AbstractR2dbcConfiguration() {
    @Bean(destroyMethod = "dispose")
    override fun connectionFactory(): ConnectionPool {
        val connectionFactory = createConnectionFactory(r2dbcProperties)
        val builder = ConnectionPoolConfiguration.builder(connectionFactory)
        val pool: R2dbcProperties.Pool = r2dbcProperties.pool
        val map = PropertyMapper.get().alwaysApplyingWhenNonNull()
        map.from(pool.maxIdleTime) to builder::maxIdleTime
        map.from(pool.maxLifeTime) to builder::maxLifeTime
        map.from(pool.maxAcquireTime) to builder::maxAcquireTime
        map.from(pool.maxCreateConnectionTime) to builder::maxCreateConnectionTime
        map.from(pool.initialSize) to builder::initialSize
        map.from(pool.maxSize) to builder::maxSize
        map.from(pool.validationQuery) to builder::validationQuery
        map.from(pool.validationDepth) to builder::validationDepth
        return ConnectionPool(builder.build())
    }

    private fun createConnectionFactory(r2dbcProperties: R2dbcProperties): PostgresqlConnectionFactory {
        val builder = ConnectionFactoryOptions.parse(r2dbcProperties.url).mutate()
        if (!StringUtil.isNullOrEmpty(r2dbcProperties.name)) {
            builder.option(ConnectionFactoryOptions.DATABASE, r2dbcProperties.name)
        }
        if (!StringUtil.isNullOrEmpty(r2dbcProperties.username)) {
            builder.option(ConnectionFactoryOptions.USER, r2dbcProperties.username)
        }
        if (!StringUtil.isNullOrEmpty(r2dbcProperties.password)) {
            builder.option(ConnectionFactoryOptions.PASSWORD, r2dbcProperties.password)
        }
        val connectionFactoryOptions = builder.build()

        val connectionConfiguration = PostgresqlConnectionConfiguration.builder()
            .host(connectionFactoryOptions.getOptionAsString(ConnectionFactoryOptions.HOST))
            .port(connectionFactoryOptions.getOptionAsInt(ConnectionFactoryOptions.PORT))
            .database(connectionFactoryOptions.getOptionAsString(ConnectionFactoryOptions.DATABASE))
            .username(connectionFactoryOptions.getOptionAsString(ConnectionFactoryOptions.USER))
            .password(connectionFactoryOptions.getOptionAsString(ConnectionFactoryOptions.PASSWORD))
            .build()
        return PostgresqlConnectionFactory(connectionConfiguration)
    }

    private fun ConnectionFactoryOptions.getOptionAsString(option: Option<*>): String =
        getRequiredValue(option) as String

    private fun ConnectionFactoryOptions.getOptionAsInt(option: Option<*>): Int =
        getRequiredValue(option) as Int
}

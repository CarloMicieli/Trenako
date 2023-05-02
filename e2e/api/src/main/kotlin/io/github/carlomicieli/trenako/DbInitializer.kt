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

import io.github.carlomicieli.trenako.database.BrandsRepository
import io.github.carlomicieli.trenako.database.seeding.Brands
import io.r2dbc.pool.ConnectionPool
import kotlinx.coroutines.reactor.awaitSingleOrNull
import kotlinx.coroutines.runBlocking
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.boot.CommandLineRunner
import org.springframework.core.io.DefaultResourceLoader
import org.springframework.core.io.Resource
import org.springframework.core.io.ResourceLoader
import org.springframework.r2dbc.connection.init.ResourceDatabasePopulator
import org.springframework.stereotype.Component

@Suppress("SqlResolve", "SqlNoDataSourceInspection")
@Component
class DbInitializer(
    private val connectionPool: ConnectionPool,
    private val brandsRepository: BrandsRepository,
    private val brands: Brands
) : CommandLineRunner {

    companion object {
        val LOG: Logger = LoggerFactory.getLogger(DbInitializer::class.java)
    }

    override fun run(vararg args: String?) {
        runBlocking {
            val count = brandsRepository.count()
            if (count == 0L) {
                LOG.info("Running the database initializer...")
                seedDatabase()

                brands.seed()
            } else {
                LOG.info("Database contains already data. Nothing to do here")
            }
        }
    }

    private suspend fun seedDatabase() {
        val filenames = listOf("brands", "railways", "scales")
        val resourceLoader: ResourceLoader = DefaultResourceLoader()
        val scripts: Array<Resource> = filenames.map {
            resourceLoader.getResource("classpath:init-db/seed/$it.sql")
        }.toTypedArray()
        ResourceDatabasePopulator(*scripts)
            .populate(connectionPool)
            .awaitSingleOrNull()
    }
}

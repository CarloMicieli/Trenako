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

import kotlinx.coroutines.reactor.awaitSingle
import org.springframework.data.r2dbc.core.R2dbcEntityTemplate
import org.springframework.data.relational.core.query.Criteria.where
import org.springframework.data.relational.core.query.Query.empty
import org.springframework.data.relational.core.query.Query.query
import org.springframework.stereotype.Component

@Component
class BrandsRepository(private val r2dbcEntityTemplate: R2dbcEntityTemplate) {
    suspend fun count(): Long {
        return r2dbcEntityTemplate
            .count(empty(), ENTITY)
            .awaitSingle()
    }

    suspend fun existsByName(name: String): Boolean {
        return r2dbcEntityTemplate
            .exists(query(where("name").`is`(name)), ENTITY)
            .awaitSingle()
    }

    suspend fun insert(newBrand: BrandRow) {
        r2dbcEntityTemplate.insert(newBrand).awaitSingle()
    }

    companion object {
        private val ENTITY: Class<BrandRow> = BrandRow::class.java
    }
}


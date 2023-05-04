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
package io.github.carlomicieli.trenako.catalog.api

import io.github.carlomicieli.trenako.model.CatalogItem
import io.github.carlomicieli.trenako.model.CatalogItemRequest
import io.github.carlomicieli.trenako.model.RollingStock
import io.github.carlomicieli.trenako.model.RollingStockRequest
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.service.annotation.DeleteExchange
import org.springframework.web.service.annotation.GetExchange
import org.springframework.web.service.annotation.HttpExchange
import org.springframework.web.service.annotation.PostExchange
import org.springframework.web.service.annotation.PutExchange
import reactor.core.publisher.Mono

@HttpExchange(
    url = "/api/catalog-items",
    accept = [MediaType.APPLICATION_JSON_VALUE],
    contentType = MediaType.APPLICATION_JSON_VALUE
)
interface CatalogItemsApi {
    @PostExchange
    fun postCatalogItem(@RequestBody catalogItemRequest: CatalogItemRequest): Mono<ResponseEntity<Unit>>

    @GetExchange("/{id}")
    fun getCatalogItemById(@PathVariable id: String): Mono<ResponseEntity<CatalogItem?>>

    @PutExchange("/{id}")
    fun putCatalogItem(
        @PathVariable id: String,
        @RequestBody catalogItemRequest: CatalogItemRequest
    ): Mono<ResponseEntity<Unit>>

    @DeleteExchange("/{id}")
    fun deleteCatalogItemById(@PathVariable id: String): Mono<ResponseEntity<Unit>>

    @PostExchange("/{id}/rolling-stocks")
    fun postRollingStock(
        @PathVariable id: String,
        @RequestBody rollingStockRequest: RollingStockRequest
    ): Mono<ResponseEntity<Unit>>

    @GetExchange("/{id}/rolling-stocks/{rollingStockId}")
    fun getRollingStockById(
        @PathVariable id: String,
        @PathVariable rollingStockId: String
    ): Mono<ResponseEntity<List<RollingStock>>>

    @PutExchange("/{id}/rolling-stocks/{rollingStockId}")
    fun putRollingStockById(
        @PathVariable id: String,
        @PathVariable rollingStockId: String,
        @RequestBody rollingStockRequest: RollingStockRequest
    ): Mono<ResponseEntity<Unit>>

    @DeleteExchange("/{id}/rolling-stocks/{rollingStockId}")
    fun deleteRollingStockBy(
        @PathVariable id: String,
        @PathVariable rollingStockId: String
    ): Mono<ResponseEntity<Unit>>
}

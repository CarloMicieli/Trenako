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

import io.github.carlomicieli.trenako.model.Brand
import io.github.carlomicieli.trenako.model.BrandRequest
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
    url = "/api/brands",
    accept = [MediaType.APPLICATION_JSON_VALUE],
    contentType = MediaType.APPLICATION_JSON_VALUE
)
interface BrandsApi {
    @GetExchange
    fun getBrands(): Mono<ResponseEntity<List<Brand>>>

    @GetExchange("{id}")
    fun getBrandById(@PathVariable id: String): Mono<ResponseEntity<Brand?>>

    @PostExchange
    fun postBrand(@RequestBody brand: BrandRequest): Mono<ResponseEntity<Unit>>

    @PutExchange("/{id}")
    fun putBrand(@PathVariable id: String, @RequestBody brand: BrandRequest): Mono<ResponseEntity<Unit>>

    @DeleteExchange("/{id}")
    fun deleteBrand(@PathVariable id: String): Mono<ResponseEntity<Unit>>
}

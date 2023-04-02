package io.github.carlomicieli.trenako.catalog

import io.github.carlomicieli.trenako.model.Brand
import io.github.carlomicieli.trenako.model.BrandRequest
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestBody
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
}

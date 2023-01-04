package com.trenako.seeding

import com.fasterxml.jackson.databind.ObjectMapper
import io.micronaut.core.annotation.Introspected

@Introspected
class Parser(val objectMapper: ObjectMapper) {
}

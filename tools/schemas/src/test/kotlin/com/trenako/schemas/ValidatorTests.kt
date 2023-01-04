package com.trenako.schemas

import com.networknt.schema.JsonSchemaFactory
import com.networknt.schema.SpecVersion
import org.junit.jupiter.api.Test

class ValidatorTests {
    @Test
    fun `the schema definitions are valid`() {
        val factory = JsonSchemaFactory.getInstance(SpecVersion.VersionFlag.V7)
        factory.getSchema(Validator::class.java.getResourceAsStream("/schemas/brands.schema.json"))
        factory.getSchema(Validator::class.java.getResourceAsStream("/schemas/catalog-items.schema.json"))
        factory.getSchema(Validator::class.java.getResourceAsStream("/schemas/railways.schema.json"))
        factory.getSchema(Validator::class.java.getResourceAsStream("/schemas/scales.schema.json"))
    }
}

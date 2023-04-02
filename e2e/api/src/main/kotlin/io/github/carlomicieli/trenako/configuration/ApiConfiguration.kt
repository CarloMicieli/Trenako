package io.github.carlomicieli.trenako.configuration

import org.springframework.boot.context.properties.ConfigurationProperties

@ConfigurationProperties(prefix = "app")
data class ApiConfiguration(var endpointUrl: String = "http://localhost:5000")

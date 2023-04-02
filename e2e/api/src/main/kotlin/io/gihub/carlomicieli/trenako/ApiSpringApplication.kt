package io.github.carlomicieli.trenako

import org.springframework.boot.Banner
import org.springframework.boot.WebApplicationType
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class ApiSpringApplication

fun main(args: Array<String>) {
    runApplication<ApiSpringApplication>(*args) {
        webApplicationType = WebApplicationType.NONE
        setBannerMode(Banner.Mode.OFF)
    }
}

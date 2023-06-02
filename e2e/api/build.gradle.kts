import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    id("kotlin-common-conventions")
    id("org.springframework.boot")
    id("io.spring.dependency-management")
    id("org.openapi.generator") version "6.6.0"
}

dependencies {
    annotationProcessor("org.springframework.boot:spring-boot-configuration-processor")
    implementation("org.springframework.boot:spring-boot-starter-data-r2dbc")
    implementation("org.postgresql:r2dbc-postgresql:1.0.0.RELEASE")
}

openApiGenerate {
    generatorName.set("java")
    inputSpec.set("${project.projectDir}/src/main/resources/openapi/api-schema.yaml")
    outputDir.set("${project.buildDir}/generated")
    apiPackage.set("io.github.carlomicieli.trenako.api")
    modelPackage.set("io.github.carlomicieli.trenako.model")
    globalProperties.set(
        mapOf(
            "apis" to "false",
            "invokers" to "false",
            "models" to ""
        )
    )
    configOptions.set(
        mapOf(
            "hideGenerationTimestamp" to "true",
            "useJakartaEe" to "true",
            "dateLibrary" to "java8",
            "library" to "webclient",
            "enumPropertyNaming" to "UPPERCASE"
        )
    )
    logToStderr.set(false)
    generateApiDocumentation.set(false)
    generateApiTests.set(false)
    generateModelDocumentation.set(false)
    generateModelTests.set(false)
    enablePostProcessFile.set(false)
}

tasks {
    withType<KotlinCompile> {
        dependsOn(openApiGenerate)
    }
}

sourceSets {
    main {
        java {
            srcDir("${project.buildDir}/generated/src/main/java")
        }
    }
}

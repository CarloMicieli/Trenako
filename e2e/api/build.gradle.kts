import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    id("kotlin-common-conventions")
    id("org.springframework.boot")
    id("io.spring.dependency-management")
    id("org.openapi.generator") version "6.3.0"
}

dependencies {
    implementation("org.springdoc:springdoc-openapi-ui:1.6.8")
    implementation("org.openapitools:jackson-databind-nullable:0.2.6")
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
/*
configure<SourceSetContainer> {
    named("main") {
        kotlin.srcDir("${project.buildDir}/generated/src/main/kotlin")
    }
}
*/

sourceSets {
    main {
        java {
            srcDir("${project.buildDir}/generated/src/main/java")
        }
    }
}

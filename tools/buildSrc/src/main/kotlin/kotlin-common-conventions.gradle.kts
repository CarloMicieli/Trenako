import org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL
import org.gradle.api.tasks.testing.logging.TestLogEvent.FAILED
import org.gradle.api.tasks.testing.logging.TestLogEvent.PASSED
import org.gradle.api.tasks.testing.logging.TestLogEvent.SKIPPED
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    id("com.diffplug.spotless")
    kotlin("jvm")
}

group = "com.trenako"
version = "0.0.0-SNAPSHOT"

repositories {
    mavenCentral()
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

tasks {
    withType<JavaCompile> {
        options.isIncremental = true
        options.isFork = true
        options.isFailOnError = false

        options.compilerArgs.addAll(
                arrayOf("-Xlint:all",
                        "-Xlint:-processing",
                        "-Werror"))
    }

    withType<KotlinCompile> {
        kotlinOptions {
            allWarningsAsErrors = true
            freeCompilerArgs = listOf("-Xjsr305=strict")
            jvmTarget = JavaVersion.VERSION_17.toString()
            apiVersion = "1.7"
            languageVersion = "1.7"
        }
    }
}

configurations {
    compileClasspath {
        resolutionStrategy.activateDependencyLocking()
    }
}

tasks {
    test {
        useJUnitPlatform()

        minHeapSize = "512m"
        maxHeapSize = "1G"
        failFast = false
        ignoreFailures = true

        testLogging {
            showStandardStreams = false
            events(PASSED, FAILED, SKIPPED)
            showExceptions = true
            showCauses = true
            showStackTraces = true
            exceptionFormat = FULL
        }
    }
}

spotless {
    kotlin {
        endWithNewline()
        ktlint()
        toggleOffOn("fmt:off", "fmt:on")
        indentWithSpaces()
        trimTrailingWhitespace()
        licenseHeaderFile("${project.rootDir}/.spotless/header.txt")
    }

    kotlinGradle {
        endWithNewline()
        ktlint()
        indentWithSpaces()
        trimTrailingWhitespace()
    }
}

dependencies {
	testImplementation(platform("org.junit:junit-bom:5.9.1"))
	testImplementation("org.junit.jupiter:junit-jupiter")
}

tasks.check {
    dependsOn(tasks.spotlessCheck)
}

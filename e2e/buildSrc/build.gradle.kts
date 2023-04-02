plugins {
    `kotlin-dsl`
}

repositories {
    gradlePluginPortal()
}

dependencies {
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.18.0")
    implementation("io.spring.gradle:dependency-management-plugin:1.1.0")
    implementation("org.springframework.boot:spring-boot-gradle-plugin:3.0.6")
    implementation("org.jetbrains.kotlin:kotlin-gradle-plugin:1.8.21")
    implementation("org.jetbrains.kotlin:kotlin-allopen:1.8.21")
}

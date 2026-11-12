/*-------------------------------------------------------------------------------------------------
  Advent of Code 2015 - Java
-------------------------------------------------------------------------------------------------*/

plugins {
    application
}

group = "aoc15"
version = "0.1.0"

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(27)
    }
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.11.3"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

application {
    mainClass = "aoc15.Main"
}

/*--------------------------------------------------------------------------------------
  Tests
--------------------------------------------------------------------------------------*/

// Tests tagged @Tag("slow") are excluded by default; run them with `gradle testSlow`.
tasks.test {
    useJUnitPlatform { excludeTags("slow") }
    testLogging { events("passed", "skipped", "failed") }
}

tasks.register<Test>("testSlow") {
    description = "Runs all tests, including those tagged slow."
    group = "verification"
    testClassesDirs = sourceSets.test.get().output.classesDirs
    classpath = sourceSets.test.get().runtimeClasspath
    useJUnitPlatform()
    testLogging { events("passed", "skipped", "failed") }
}

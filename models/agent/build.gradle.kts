import software.amazon.smithy.gradle.tasks.SmithyBuild

plugins {
    id("software.amazon.smithy").version("0.7.0")
}

configure<software.amazon.smithy.gradle.SmithyExtension> {}

repositories {
    mavenLocal()
    mavenCentral()
}

buildscript {
    val smithyVersion = "1.32.0"
    dependencies {
        classpath("software.amazon.smithy:smithy-openapi:$smithyVersion")
        classpath("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
        classpath("software.amazon.smithy:smithy-cli:$smithyVersion")
    }
}

dependencies {
    val smithyVersion = "1.32.0"
    implementation("software.amazon.smithy:smithy-model:$smithyVersion")
    implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
    implementation("software.amazon.smithy:smithy-validation-model:$smithyVersion")
    implementation("software.amazon.smithy:smithy-linters:$smithyVersion")
    implementation("software.amazon.smithy:smithy-openapi:$smithyVersion")
    implementation("software.amazon.smithy:smithy-waiters:$smithyVersion")
    implementation("software.amazon.smithy.rust.codegen:codegen-client:0.1.0")
    implementation("software.amazon.smithy.rust.codegen.server.smithy:codegen-server:0.1.0")
    implementation("software.amazon.smithy.rust.codegen:codegen-core:0.1.0")
    implementation("software.amazon.smithy.go:smithy-go-codegen:0.1.0")
    implementation("software.amazon.smithy:smithy-aws-traits:$smithyVersion")
}
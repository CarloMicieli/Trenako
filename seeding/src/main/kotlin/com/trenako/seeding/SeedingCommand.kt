/*
 *   Copyright (c) 2021-2023 (C) Carlo Micieli
 *
 *    Licensed to the Apache Software Foundation (ASF) under one
 *    or more contributor license agreements.  See the NOTICE file
 *    distributed with this work for additional information
 *    regarding copyright ownership.  The ASF licenses this file
 *    to you under the Apache License, Version 2.0 (the
 *    "License"); you may not use this file except in compliance
 *    with the License.  You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing,
 *    software distributed under the License is distributed on an
 *    "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 *    KIND, either express or implied.  See the License for the
 *    specific language governing permissions and limitations
 *    under the License.
 */
package com.trenako.seeding

import io.micronaut.configuration.picocli.PicocliRunner
import jakarta.inject.Inject
import picocli.CommandLine.Command
import picocli.CommandLine.Option

@Command(
    name = "trenako-seeding",
    description = ["..."],
    mixinStandardHelpOptions = true,
)
class SeedingCommand : Runnable {

    @Option(names = ["-v", "--verbose"], description = ["..."])
    private var verbose: Boolean = false

    @Inject private lateinit var parser: Parser

    override fun run() {
        // business logic here
        if (verbose) {
            println("Hi!")
        }
    }

    companion object {
        @JvmStatic fun main(args: Array<String>) {
            PicocliRunner.run(SeedingCommand::class.java, *args)
        }
    }
}

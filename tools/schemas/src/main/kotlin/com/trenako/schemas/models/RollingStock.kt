/*
 *   Copyright (c) 2022-2023 (C) Carlo Micieli
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

package com.trenako.schemas.models

import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "category", visible = true)
@JsonSubTypes(
    JsonSubTypes.Type(value = ElectricMultipleUnit::class, name = "ELECTRIC_MULTIPLE_UNIT"),
    JsonSubTypes.Type(value = FreightCar::class, name = "FREIGHT_CAR"),
    JsonSubTypes.Type(value = Locomotive::class, name = "LOCOMOTIVE"),
    JsonSubTypes.Type(value = PassengerCar::class, name = "PASSENGER_CAR"),
    JsonSubTypes.Type(value = Railcar::class, name = "RAILCAR")
)
interface RollingStock {
    val railway: String
    val epoch: String
    val livery: String?
    val lengthOverBuffer: LengthOverBuffer?
    val technicalSpecifications: TechnicalSpecifications?
}

data class LengthOverBuffer(
    val inches: Float? = null,
    val millimeters: Float? = null
)

data class TechnicalSpecifications(
    val minimumRadius: Float,
    val coupling: Coupling,
    val flywheelFitted: FeatureFlag,
    val metalBody: FeatureFlag,
    val interiorLights: FeatureFlag,
    val lights: FeatureFlag,
    val springBuffers: FeatureFlag
)

data class Coupling(
    val socket: Socket?,
    val closeCouplers: FeatureFlag,
    val digitalShunting: FeatureFlag
)

enum class Socket(val value: String) {
    @JsonProperty("NONE")
    NONE("NONE"),

    @JsonProperty("NEM_355")
    NEM_355("NEM_355"),

    @JsonProperty("NEM_356")
    NEM_356("NEM_356"),

    @JsonProperty("NEM_357")
    NEM_357("NEM_357"),

    @JsonProperty("NEM_359")
    NEM_359("NEM_359"),

    @JsonProperty("NEM_360")
    NEM_360("NEM_360"),

    @JsonProperty("NEM_362")
    NEM_362("NEM_362"),

    @JsonProperty("NEM_365")
    NEM_365("NEM_365")
}

enum class FeatureFlag(val value: String) {
    @JsonProperty("YES")
    YES("YES"),

    @JsonProperty("NO")
    NO("NO"),

    @JsonProperty("NOT_AVAILABLE")
    NOT_AVAILABLE("NOT_AVAILABLE")
}

data class ElectricMultipleUnit(
    val typeName: String,
    val roadNumber: String? = null,
    val series: String? = null,
    val electricMultipleUnitType: ElectricMultipleUnitType,
    override val railway: String,
    override val epoch: String,
    val depot: String? = null,
    val dccInterface: DccInterface? = null,
    val control: Control? = null,
    val isDummy: Boolean,
    override val livery: String? = null,
    override val lengthOverBuffer: LengthOverBuffer? = null,
    override val technicalSpecifications: TechnicalSpecifications? = null
) : RollingStock

enum class ElectricMultipleUnitType(val value: String) {

    @JsonProperty("DRIVING_CAR")
    DRIVING_CAR("DRIVING_CAR"),

    @JsonProperty("MOTOR_CAR")
    MOTOR_CAR("MOTOR_CAR"),

    @JsonProperty("POWER_CAR")
    POWER_CAR("POWER_CAR"),

    @JsonProperty("TRAILER_CAR")
    TRAILER_CAR("TRAILER_CAR")
}

data class FreightCar(
    val typeName: String,
    val roadNumber: String? = null,
    val freightCarType: FreightCarType? = null,
    override val railway: String,
    override val epoch: String,
    override val livery: String? = null,
    override val lengthOverBuffer: LengthOverBuffer? = null,
    override val technicalSpecifications: TechnicalSpecifications? = null
) : RollingStock

enum class FreightCarType(val value: String) {
    @JsonProperty("AUTO_TRANSPORT_CARS")
    AUTO_TRANSPORT_CARS("AUTO_TRANSPORT_CARS"),

    @JsonProperty("BRAKE_WAGON")
    BRAKE_WAGON("BRAKE_WAGON"),

    @JsonProperty("CONTAINER_CARS")
    CONTAINER_CARS("CONTAINER_CARS"),

    @JsonProperty("COVERED_FREIGHT_CARS")
    COVERED_FREIGHT_CARS("COVERED_FREIGHT_CARS"),

    @JsonProperty("DEEP_WELL_FLAT_CARS")
    DEEP_WELL_FLAT_CARS("DEEP_WELL_FLAT_CARS"),

    @JsonProperty("DUMP_CARS")
    DUMP_CARS("DUMP_CARS"),

    @JsonProperty("GONDOLA")
    GONDOLA("GONDOLA"),

    @JsonProperty("HEAVY_GOODS_WAGONS")
    HEAVY_GOODS_WAGONS("HEAVY_GOODS_WAGONS"),

    @JsonProperty("HINGED_COVER_WAGONS")
    HINGED_COVER_WAGONS("HINGED_COVER_WAGONS"),

    @JsonProperty("HOPPER_WAGON")
    HOPPER_WAGON("HOPPER_WAGON"),

    @JsonProperty("REFRIGERATOR_CARS")
    REFRIGERATOR_CARS("REFRIGERATOR_CARS"),

    @JsonProperty("SILO_CONTAINER_CARS")
    SILO_CONTAINER_CARS("SILO_CONTAINER_CARS"),

    @JsonProperty("SLIDE_TARPAULIN_WAGON")
    SLIDE_TARPAULIN_WAGON("SLIDE_TARPAULIN_WAGON"),

    @JsonProperty("SLIDING_WALL_BOXCARS")
    SLIDING_WALL_BOXCARS("SLIDING_WALL_BOXCARS"),

    @JsonProperty("SPECIAL_TRANSPORT")
    SPECIAL_TRANSPORT("SPECIAL_TRANSPORT"),

    @JsonProperty("STAKE_WAGONS")
    STAKE_WAGONS("STAKE_WAGONS"),

    @JsonProperty("SWING_ROOF_WAGON")
    SWING_ROOF_WAGON("SWING_ROOF_WAGON"),

    @JsonProperty("TANK_CARS")
    TANK_CARS("TANK_CARS"),

    @JsonProperty("TELESCOPE_HOOD_WAGONS")
    TELESCOPE_HOOD_WAGONS("TELESCOPE_HOOD_WAGONS")
}

data class Locomotive(
    val className: String,
    val roadNumber: String,
    val series: String? = null,
    val locomotiveType: LocomotiveType,
    override val railway: String,
    override val epoch: String,
    val depot: String? = null,
    val dccInterface: DccInterface? = null,
    val control: Control? = null,
    val isDummy: Boolean,
    override val livery: String? = null,
    override val lengthOverBuffer: LengthOverBuffer? = null,
    override val technicalSpecifications: TechnicalSpecifications? = null
) : RollingStock

enum class LocomotiveType(val value: String) {
    @JsonProperty("DIESEL_LOCOMOTIVE")
    DIESEL_LOCOMOTIVE("DIESEL_LOCOMOTIVE"),

    @JsonProperty("ELECTRIC_LOCOMOTIVE")
    ELECTRIC_LOCOMOTIVE("ELECTRIC_LOCOMOTIVE"),

    @JsonProperty("STEAM_LOCOMOTIVE")
    STEAM_LOCOMOTIVE("STEAM_LOCOMOTIVE")
}

data class PassengerCar(
    val typeName: String,
    val roadNumber: String? = null,
    val passengerCarType: PassengerCarType? = null,
    val serviceLevel: String? = null,
    override val railway: String,
    override val epoch: String,
    override val livery: String? = null,
    override val lengthOverBuffer: LengthOverBuffer? = null,
    override val technicalSpecifications: TechnicalSpecifications? = null
) : RollingStock

enum class PassengerCarType(val value: String) {
    @JsonProperty("BAGGAGE_CAR")
    BAGGAGE_CAR("BAGGAGE_CAR"),

    @JsonProperty("COMBINE_CAR")
    COMBINE_CAR("COMBINE_CAR"),

    @JsonProperty("COMPARTMENT_COACH")
    COMPARTMENT_COACH("COMPARTMENT_COACH"),

    @JsonProperty("DINING_CAR")
    DINING_CAR("DINING_CAR"),

    @JsonProperty("DOUBLE_DECKER")
    DOUBLE_DECKER("DOUBLE_DECKER"),

    @JsonProperty("DRIVING_TRAILER")
    DRIVING_TRAILER("DRIVING_TRAILER"),

    @JsonProperty("LOUNGE")
    LOUNGE("LOUNGE"),

    @JsonProperty("OBSERVATION")
    OBSERVATION("OBSERVATION"),

    @JsonProperty("OPEN_COACH")
    OPEN_COACH("OPEN_COACH"),

    @JsonProperty("RAILWAY_POST_OFFICE")
    RAILWAY_POST_OFFICE("RAILWAY_POST_OFFICE"),

    @JsonProperty("SLEEPING_CAR")
    SLEEPING_CAR("SLEEPING_CAR")
}

data class Railcar(
    val typeName: String,
    val roadNumber: String? = null,
    val series: String? = null,
    val railcarType: RailcarType,
    override val railway: String,
    override val epoch: String,
    val depot: String? = null,
    val dccInterface: DccInterface? = null,
    val control: Control? = null,
    val isDummy: Boolean,
    override val livery: String? = null,
    override val lengthOverBuffer: LengthOverBuffer? = null,
    override val technicalSpecifications: TechnicalSpecifications? = null
) : RollingStock

enum class RailcarType(val value: String) {
    @JsonProperty("POWER_CAR")
    POWER_CAR("POWER_CAR"),

    @JsonProperty("TRAILER_CAR")
    TRAILER_CAR("TRAILER_CAR")
}

enum class Control(val value: String) {
    @JsonProperty("DCC")
    DCC("DCC"),

    @JsonProperty("DCC_READY")
    DCC_READY("DCC_READY"),

    @JsonProperty("DCC_SOUND")
    DCC_SOUND("DCC_SOUND"),

    @JsonProperty("NO_DCC")
    NO_DCC("NO_DCC")
}

enum class DccInterface(val value: String) {
    @JsonProperty("NONE")
    NONE("NONE"),

    @JsonProperty("MTC_21")
    MTC_21("MTC_21"),

    @JsonProperty("NEM_651")
    NEM_651("NEM_651"),

    @JsonProperty("NEM_652")
    NEM_652("NEM_652"),

    @JsonProperty("NEM_654")
    NEM_654("NEM_654"),

    @JsonProperty("NEXT_18")
    NEXT_18("NEXT_18"),

    @JsonProperty("NEXT_18_S")
    NEXT_18_S("NEXT_18_S"),

    @JsonProperty("PLUX_12")
    PLUX_12("PLUX_12"),

    @JsonProperty("PLUX_16")
    PLUX_16("PLUX_16"),

    @JsonProperty("PLUX_22")
    PLUX_22("PLUX_22"),

    @JsonProperty("PLUX_8")
    PLUX_8("PLUX_8")
}

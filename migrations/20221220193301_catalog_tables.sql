-- noinspection SqlNoDataSourceInspectionForFile
CREATE TYPE brand_kind AS ENUM ('INDUSTRIAL', 'BRASS_MODELS');
CREATE TYPE brand_status AS ENUM ('ACTIVE', 'OUT_OF_BUSINESS');
CREATE TYPE gauge AS ENUM ('BROAD', 'MEDIUM', 'MINIMUM', 'NARROW', 'STANDARD');
CREATE TYPE organization_entity_type AS ENUM (
    'CIVIL_LAW_PARTNERSHIP',
    'ENTREPRENEURIAL_COMPANY',
    'GLOBAL_PARTNERSHIP',
    'LIMITED_COMPANY',
    'LIMITED_PARTNERSHIP',
    'LIMITED_PARTNERSHIP_LIMITED_COMPANY',
    'OTHER',
    'PUBLIC_INSTITUTION',
    'PUBLIC_LIMITED_COMPANY',
    'REGISTERED_SOLE_TRADER',
    'SOLE_TRADER');
CREATE TYPE railway_status AS ENUM ('ACTIVE', 'INACTIVE');
CREATE TYPE catalog_item_type AS ENUM (
    'LOCOMOTIVES',
    'TRAIN_SETS',
    'STARTER_SETS',
    'FREIGHT_CARS',
    'PASSENGER_CARS',
    'ELECTRIC_MULTIPLE_UNITS',
    'RAILCARS'
);
CREATE TYPE feature_flag AS ENUM ('YES', 'NO', 'N/A');

CREATE TABLE public.brands
(
    brand_id                 varchar(50) NOT NULL,
    name                     varchar(50) NOT NULL,
    registered_company_name  varchar(100),
    group_name               varchar(50),
    description              varchar(1000),
    contact_email            varchar(255),
    contact_website_url      varchar(100),
    contact_phone            varchar(20),
    kind                     brand_kind  NOT NULL,
    status                   brand_status,
    address_street_address   varchar(255),
    address_extended_address varchar(255),
    address_city             varchar(50),
    address_region           varchar(50),
    address_postal_code      varchar(10),
    address_country          varchar(2),
    socials_facebook         varchar(255),
    socials_instagram        varchar(255),
    socials_linkedin         varchar(255),
    socials_twitter          varchar(255),
    socials_youtube          varchar(255),
    created                  timestamp without time zone NOT NULL,
    last_modified            timestamp without time zone,
    version                  integer     NOT NULL DEFAULT 1,
    CONSTRAINT "PK_brands" PRIMARY KEY (brand_id)
);

CREATE UNIQUE INDEX "Idx_brands_name"
    ON brands USING btree
    (name ASC NULLS LAST);

CREATE TABLE public.railways
(
    railway_id               varchar(25) NOT NULL,
    name                     varchar(25) NOT NULL,
    registered_company_name  varchar(250),
    organization_entity_type organization_entity_type,
    description              varchar(1000),
    country                  varchar(2),
    operating_since          timestamp without time zone,
    operating_until          timestamp without time zone,
    status                   railway_status,
    gauge_mm                 numeric(19, 5),
    gauge_in                 numeric(19, 5),
    track_gauge              gauge,
    headquarters             varchar(250),
    total_length_mi          numeric(19, 5),
    total_length_km          numeric(19, 5),
    contact_email            varchar(255),
    contact_website_url      varchar(100),
    contact_phone            varchar(20),
    socials_facebook         varchar(255),
    socials_instagram        varchar(255),
    socials_linkedin         varchar(255),
    socials_twitter          varchar(255),
    socials_youtube          varchar(255),
    created                  timestamp without time zone NOT NULL,
    last_modified            timestamp without time zone,
    version                  integer     NOT NULL DEFAULT 1,
    CONSTRAINT "PK_railways" PRIMARY KEY (railway_id)
);

CREATE UNIQUE INDEX "Idx_railways_name"
    ON public.railways USING btree
    (name ASC NULLS LAST);

CREATE TABLE public.scales
(
    scale_id          varchar(25)    NOT NULL,
    name              varchar(25)    NOT NULL,
    ratio             numeric(19, 5) NOT NULL,
    gauge_millimeters numeric(19, 5),
    gauge_inches      numeric(19, 5),
    track_gauge       gauge          NOT NULL,
    description       varchar(2500),
    standards         varchar(100),
    created           timestamp without time zone NOT NULL,
    last_modified     timestamp without time zone,
    version           integer        NOT NULL DEFAULT 1,
    CONSTRAINT "PK_scales" PRIMARY KEY (scale_id)
);

CREATE UNIQUE INDEX "Idx_scales_name"
    ON scales USING btree
    (name ASC NULLS LAST);

CREATE TABLE public.catalog_items
(
    catalog_item_id varchar(65)       NOT NULL,
    brand_id        varchar(50)       NOT NULL,
    item_number     varchar(10)       NOT NULL,
    scale_id        varchar(25)       NOT NULL,
    category        catalog_item_type NOT NULL,
    description     varchar(2500),
    details         varchar(2500),
    power_method    varchar(2)        NOT NULL,
    delivery_date   varchar(10),
    available       boolean,
    count           integer,
    created         timestamp without time zone NOT NULL,
    last_modified   timestamp without time zone,
    version         integer           NOT NULL DEFAULT 1,
    CONSTRAINT "PK_catalog_items" PRIMARY KEY (catalog_item_id),
    CONSTRAINT "FK_catalog_items_brands" FOREIGN KEY (brand_id)
        REFERENCES public.brands (brand_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_catalog_items_scales" FOREIGN KEY (scale_id)
        REFERENCES public.scales (scale_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE UNIQUE INDEX "Idx_catalog_items_brand_id_item_number"
    ON public.catalog_items USING btree
    (item_number ASC NULLS LAST, brand_id ASC NULLS LAST);

CREATE TABLE public.rolling_stocks
(
    rolling_stock_id          uuid        NOT NULL,
    catalog_item_id           varchar(65) NOT NULL,
    railway_id                varchar(25) NOT NULL,
    category                  varchar(25) NOT NULL,
    epoch                     varchar(10) NOT NULL,
    livery                    varchar(50),
    length_over_buffer_mm     numeric(19, 5),
    length_over_buffer_in     numeric(19, 5),
    type_name                 varchar(25),
    class_name                varchar(15),
    road_number               varchar(15),
    series                    varchar(50),
    depot                     varchar(100),
    dcc_interface             varchar(10),
    control                   varchar(10),
    passenger_car_type        varchar(25),
    service_level             varchar(15),
    is_dummy                  boolean,
    minimum_radius            numeric(19, 5),
    coupling                  varchar(10),
    flywheel_fitted           feature_flag,
    close_couplers            feature_flag,
    metal_body                feature_flag,
    interior_lights           feature_flag,
    lights                    feature_flag,
    spring_buffers            feature_flag,
    digital_shunting_coupling feature_flag,
    CONSTRAINT "PK_rolling_stocks" PRIMARY KEY (rolling_stock_id),
    CONSTRAINT "FK_rolling_stocks_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_rolling_stocks_railways" FOREIGN KEY (railway_id)
        REFERENCES public.railways (railway_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

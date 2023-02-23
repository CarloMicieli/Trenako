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
    'SOLE_TRADER',
    'STATE_OWNED_ENTERPRISE'
    );
CREATE TYPE railway_status AS ENUM ('ACTIVE', 'INACTIVE');

CREATE TABLE public.brands
(
    brand_id                 varchar(50) NOT NULL,
    name                     varchar(50) NOT NULL,
    registered_company_name  varchar(100),
    organization_entity_type organization_entity_type,
    group_name               varchar(100),
    description_en           varchar(2500),
    description_it           varchar(2500),
    kind                     brand_kind  NOT NULL,
    status                   brand_status,
    contact_email            varchar(250),
    contact_website_url      varchar(100),
    contact_phone            varchar(20),
    address_street_address   varchar(255),
    address_extended_address varchar(255),
    address_city             varchar(50),
    address_region           varchar(50),
    address_postal_code      varchar(10),
    address_country          varchar(3),
    socials_facebook         varchar(100),
    socials_instagram        varchar(100),
    socials_linkedin         varchar(100),
    socials_twitter          varchar(100),
    socials_youtube          varchar(100),
    created_at               timestamptz NOT NULL,
    last_modified_at         timestamptz,
    version                  integer     NOT NULL DEFAULT 1,
    CONSTRAINT "PK_brands" PRIMARY KEY (brand_id)
);

CREATE TABLE public.railways
(
    railway_id               varchar(50) NOT NULL,
    name                     varchar(50) NOT NULL,
    abbreviation             varchar(10),
    registered_company_name  varchar(250),
    organization_entity_type organization_entity_type,
    description_en           varchar(2500),
    description_it           varchar(2500),
    country                  varchar(3)  NOT NULL,
    operating_since          date,
    operating_until          date,
    status                   railway_status,
    gauge_m                  numeric(19, 5),
    track_gauge              gauge,
    headquarters             varchar(250),
    total_length_mi          numeric(19, 5),
    total_length_km          numeric(19, 5),
    contact_email            varchar(255),
    contact_website_url      varchar(100),
    contact_phone            varchar(20),
    socials_facebook         varchar(100),
    socials_instagram        varchar(100),
    socials_linkedin         varchar(100),
    socials_twitter          varchar(100),
    socials_youtube          varchar(100),
    created_at               timestamptz NOT NULL,
    last_modified_at         timestamptz,
    version                  integer     NOT NULL DEFAULT 1,
    CONSTRAINT "PK_railways" PRIMARY KEY (railway_id)
);

CREATE TABLE public.scales
(
    scale_id          varchar(50)    NOT NULL,
    name              varchar(50)    NOT NULL,
    ratio             numeric(19, 5) NOT NULL,
    gauge_millimeters numeric(19, 5),
    gauge_inches      numeric(19, 5),
    track_gauge       gauge          NOT NULL,
    description_en    varchar(2500),
    description_it    varchar(2500),
    standards         varchar(100),
    created_at        timestamptz    NOT NULL,
    last_modified_at  timestamptz,
    version           integer        NOT NULL DEFAULT 1,
    CONSTRAINT "PK_scales" PRIMARY KEY (scale_id)
);

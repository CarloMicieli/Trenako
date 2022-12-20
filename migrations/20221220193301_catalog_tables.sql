-- noinspection SqlNoDataSourceInspectionForFile
CREATE TYPE brand_kind AS ENUM ('INDUSTRIAL', 'BRASS_MODELS');
CREATE TYPE brand_status AS ENUM ('ACTIVE', 'OUT_OF_BUSINESS');

CREATE TABLE public.brands
(
    brand_id                 varchar(50)                 NOT NULL,
    name                     varchar(50)                 NOT NULL,
    registered_company_name  varchar(100),
    group_name               varchar(50),
    description              varchar(1000),
    email                    varchar(100),
    website_url              varchar(100),
    phone_number             varchar(20),
    kind                     brand_kind                  NOT NULL,
    status                   brand_status,
    address_street_address   varchar(255),
    address_extended_address varchar(255),
    address_city             varchar(50),
    address_region           varchar(50),
    address_postal_code      varchar(10),
    address_country          varchar(2),
    created                  timestamp without time zone NOT NULL,
    last_modified            timestamp without time zone,
    version                  integer                     NOT NULL DEFAULT 1,
    CONSTRAINT "PK_brands" PRIMARY KEY (brand_id)
);

CREATE UNIQUE INDEX "Idx_brands_name"
    ON brands USING btree
    (name ASC NULLS LAST);

CREATE TYPE scale_gauge AS ENUM ('BROAD', 'MEDIUM', 'MINIMUM', 'NARROW', 'STANDARD');

CREATE TABLE public.scales
(
    scale_id          varchar(25)    NOT NULL,
    name              varchar(25)    NOT NULL,
    ratio             numeric(19, 5) NOT NULL,
    gauge_millimeters numeric(19, 5),
    gauge_inches      numeric(19, 5),
    track_gauge       scale_gauge    NOT NULL,
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

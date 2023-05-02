-- noinspection SqlNoDataSourceInspectionForFile

INSERT INTO public.railways (railway_id, name, abbreviation, registered_company_name, organization_entity_type,
                             description_en, description_it, country, operating_since, operating_until, status,
                             gauge_meters, track_gauge, headquarters, total_length_mi, total_length_km, contact_email,
                             contact_website_url, contact_phone, socials_facebook, socials_instagram, socials_linkedin,
                             socials_twitter, socials_youtube, created_at, last_modified_at, version)
VALUES ('fs', 'FS', 'FS', 'Ferrovie dello Stato Italiane S.p.A.', 'STATE_OWNED_ENTERPRISE', null, null, 'IT',
        '1905-07-01', null, 'ACTIVE', 1.435, 'STANDARD', '{Roma}', 15263.4, 24564.0, null, 'https://www.fsitaliane.it',
        null, null, 'fsitaliane', 'ferrovie-dello-stato-s-p-a-', 'FSitaliane', 'fsitaliane',
        '2023-04-27 18:08:17.719759 +00:00', null, 1);

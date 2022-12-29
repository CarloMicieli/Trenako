use crate::railways::period_of_activity::PeriodOfActivity;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_id::RailwayId;
use crate::railways::railway_length::RailwayLength;
use common::contact::ContactInfo;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use isocountry::CountryCode;
use std::fmt::Formatter;
use std::{cmp, fmt};

/// A railway company is a company within the rail industry.
///
/// It can be a manufacturing firm or an operator. Some railway companies operate both the trains
/// and the track, while, particularly in the European Union, operation of the track is undertaken
/// by infrastructure operators and trains are run by different companies.
///
/// Railway companies can be private or public.
#[derive(Debug, Clone, Serialize)]
pub struct Railway {
    railway_id: RailwayId,
    name: String,
    abbreviation: Option<String>,
    registered_company_name: String,
    organization_entity_type: Option<OrganizationEntityType>,
    description: Option<String>,
    period_of_activity: Option<PeriodOfActivity>,
    length: Option<RailwayLength>,
    gauge: Option<RailwayGauge>,
    country: CountryCode,
    headquarters: Option<String>,
    contact_info: Option<ContactInfo>,
    socials: Option<Socials>,
    metadata: Metadata,
}

impl Railway {
    /// Create new railway
    pub fn new(
        railway_id: RailwayId,
        name: &str,
        abbreviation: Option<&str>,
        registered_company_name: &str,
        organization_entity_type: Option<OrganizationEntityType>,
        description: Option<&str>,
        period_of_activity: Option<PeriodOfActivity>,
        length: Option<RailwayLength>,
        gauge: Option<RailwayGauge>,
        country: CountryCode,
        headquarters: Option<&str>,
        contact_info: Option<ContactInfo>,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        Railway {
            railway_id,
            name: String::from(name),
            abbreviation: abbreviation.map(str::to_string),
            registered_company_name: String::from(registered_company_name),
            organization_entity_type,
            description: description.map(str::to_string),
            period_of_activity,
            length,
            gauge,
            country,
            headquarters: headquarters.map(str::to_string),
            contact_info,
            socials,
            metadata,
        }
    }

    /// The unique identifier for this railway company
    pub fn railway_id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// The name for this railway company
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The abbreviated name for this railway company
    pub fn abbreviation(&self) -> Option<&str> {
        self.abbreviation.as_deref()
    }

    /// The registered company name (the more formal denomination)
    /// for this Railway company
    pub fn registered_company_name(&self) -> &str {
        &self.registered_company_name
    }

    /// The organization entity type for this railway company
    pub fn organization_entity_type(&self) -> Option<OrganizationEntityType> {
        self.organization_entity_type
    }

    /// The description for this railway company
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// The period of activity (active/inactive) for this railway company
    pub fn period_of_activity(&self) -> Option<&PeriodOfActivity> {
        self.period_of_activity.as_ref()
    }

    /// The total railway network length controlled by this railway company
    pub fn length(&self) -> Option<&RailwayLength> {
        self.length.as_ref()
    }

    /// The track gauge for this railway
    pub fn gauge(&self) -> Option<&RailwayGauge> {
        self.gauge.as_ref()
    }

    /// The registration country
    pub fn country(&self) -> CountryCode {
        self.country
    }

    /// The railway headquarter
    pub fn headquarters(&self) -> Option<&str> {
        self.headquarters.as_deref()
    }

    /// The contact railway info
    pub fn contact_info(&self) -> Option<&ContactInfo> {
        self.contact_info.as_ref()
    }

    /// The social profiles for this railway
    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    /// The metadata for this railway company
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Railway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", &self.name, self.registered_company_name)
    }
}

impl cmp::PartialEq for Railway {
    fn eq(&self, other: &Self) -> bool {
        self.railway_id == other.railway_id
    }
}

impl cmp::Eq for Railway {}

#[cfg(test)]
mod test {
    use super::*;

    mod railways {
        use super::*;
        use crate::railways::test_data::{die_bahn, fs};
        use chrono::Utc;
        use common::contact::WebsiteUrl;
        use common::socials::SocialsBuilder;
        use pretty_assertions::{assert_eq, assert_ne};
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_railways() {
            let metadata = Metadata::created_at(Utc::now());
            let socials = SocialsBuilder::default()
                .instagram("fsitaliane")
                .linkedin("ferrovie-dello-stato-s-p-a-")
                .twitter("FSitaliane")
                .youtube("fsitaliane")
                .build();
            let length = RailwayLength::of_kilometers(dec!(24564.0));
            let gauge = RailwayGauge::standard();
            let contact_info = ContactInfo::new(None, Some(WebsiteUrl::new("https://www.fsitaliane.it")), None);

            let railway = Railway::new(
                RailwayId::new("FS"),
                "FS",
                Some("FS"),
                "Ferrovie dello stato italiane",
                Some(OrganizationEntityType::StateOwnedEnterprise),
                None,
                None,
                Some(length),
                Some(gauge.clone()),
                CountryCode::ITA,
                Some("Rome"),
                Some(contact_info.clone()),
                Some(socials.clone()),
                metadata.clone(),
            );

            assert_eq!(&RailwayId::new("FS"), railway.railway_id());
            assert_eq!("FS", railway.name());
            assert_eq!(Some("FS"), railway.abbreviation());
            assert_eq!("Ferrovie dello stato italiane", railway.registered_company_name());
            assert_eq!(
                Some(OrganizationEntityType::StateOwnedEnterprise),
                railway.organization_entity_type()
            );
            assert_eq!(Some("Rome"), railway.headquarters());
            assert_eq!(Some(&length), railway.length());
            assert_eq!(Some(&gauge), railway.gauge());
            assert_eq!(Some(&contact_info), railway.contact_info());
            assert_eq!(Some(&socials), railway.socials());
            assert_eq!(&metadata, railway.metadata());
        }

        #[test]
        fn it_should_display_railways() {
            let metadata = Metadata::created_at(Utc::now());
            let railway = Railway::new(
                RailwayId::new("FS"),
                "FS",
                Some("FS"),
                "Ferrovie dello stato italiane",
                Some(OrganizationEntityType::StateOwnedEnterprise),
                None,
                None,
                None,
                None,
                CountryCode::ITA,
                Some("Rome"),
                None,
                None,
                metadata,
            );

            assert_eq!("FS - Ferrovie dello stato italiane", railway.to_string());
        }

        #[test]
        fn it_should_compare_railways() {
            let db = die_bahn();
            let fs = fs();

            assert_eq!(fs, fs);
            assert_ne!(fs, db);
        }
    }
}

use serde_derive::{Deserialize, Serialize};
use strum_macros;
use strum_macros::{Display, EnumString};

/// The many types of business entities defined in the legal systems of various countries
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum OrganizationEntityType {
    CivilLawPartnership,
    EntrepreneurialCompany,
    GlobalPartnership,
    LimitedCompany,
    LimitedPartnership,
    LimitedPartnershipLimitedCompany,
    Other,
    PublicInstitution,
    PublicLimitedCompany,
    RegisteredSoleTrader,
    SoleTrader,
    StateOwnedEnterprise,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod organization_entity_types {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_parse_organization_entity_types() {
            let org_type = "Entrepreneurial_Company".parse::<OrganizationEntityType>();
            assert!(org_type.is_ok());
            assert_eq!(org_type.unwrap(), OrganizationEntityType::EntrepreneurialCompany);
        }

        #[test]
        fn it_should_display_organization_entity_types() {
            assert_eq!(
                "entrepreneurial_company".to_string(),
                OrganizationEntityType::EntrepreneurialCompany.to_string()
            );
        }
    }
}

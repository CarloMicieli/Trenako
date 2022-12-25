use serde_derive::{Deserialize, Serialize};
use strum_macros;
use strum_macros::{Display, EnumString};

/// The many types of business entities defined in the legal systems of various countries
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
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
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("CIVIL_LAW_PARTNERSHIP", Ok(OrganizationEntityType::CivilLawPartnership))]
        #[case("ENTREPRENEURIAL_COMPANY", Ok(OrganizationEntityType::EntrepreneurialCompany))]
        #[case("GLOBAL_PARTNERSHIP", Ok(OrganizationEntityType::GlobalPartnership))]
        #[case("LIMITED_COMPANY", Ok(OrganizationEntityType::LimitedCompany))]
        #[case("LIMITED_PARTNERSHIP", Ok(OrganizationEntityType::LimitedPartnership))]
        #[case(
            "LIMITED_PARTNERSHIP_LIMITED_COMPANY",
            Ok(OrganizationEntityType::LimitedPartnershipLimitedCompany)
        )]
        #[case("OTHER", Ok(OrganizationEntityType::Other))]
        #[case("PUBLIC_INSTITUTION", Ok(OrganizationEntityType::PublicInstitution))]
        #[case("PUBLIC_LIMITED_COMPANY", Ok(OrganizationEntityType::PublicLimitedCompany))]
        #[case("REGISTERED_SOLE_TRADER", Ok(OrganizationEntityType::RegisteredSoleTrader))]
        #[case("SOLE_TRADER", Ok(OrganizationEntityType::SoleTrader))]
        #[case("STATE_OWNED_ENTERPRISE", Ok(OrganizationEntityType::StateOwnedEnterprise))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_organization_entity_types(
            #[case] input: &str,
            #[case] expected: Result<OrganizationEntityType, ParseError>,
        ) {
            let org_type = input.parse::<OrganizationEntityType>();
            assert_eq!(expected, org_type);
        }

        #[rstest]
        #[case(OrganizationEntityType::CivilLawPartnership, "CIVIL_LAW_PARTNERSHIP")]
        #[case(OrganizationEntityType::EntrepreneurialCompany, "ENTREPRENEURIAL_COMPANY")]
        #[case(OrganizationEntityType::GlobalPartnership, "GLOBAL_PARTNERSHIP")]
        #[case(OrganizationEntityType::LimitedCompany, "LIMITED_COMPANY")]
        #[case(OrganizationEntityType::LimitedPartnership, "LIMITED_PARTNERSHIP")]
        #[case(
            OrganizationEntityType::LimitedPartnershipLimitedCompany,
            "LIMITED_PARTNERSHIP_LIMITED_COMPANY"
        )]
        #[case(OrganizationEntityType::Other, "OTHER")]
        #[case(OrganizationEntityType::PublicInstitution, "PUBLIC_INSTITUTION")]
        #[case(OrganizationEntityType::PublicLimitedCompany, "PUBLIC_LIMITED_COMPANY")]
        #[case(OrganizationEntityType::RegisteredSoleTrader, "REGISTERED_SOLE_TRADER")]
        #[case(OrganizationEntityType::SoleTrader, "SOLE_TRADER")]
        #[case(OrganizationEntityType::StateOwnedEnterprise, "STATE_OWNED_ENTERPRISE")]
        fn it_should_display_organization_entity_types(#[case] input: OrganizationEntityType, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}

use isocountry::CountryCode;
use thiserror::Error;

/// It represents a physical street address
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    /// the street address
    pub street_address: String,
    /// the (optional) extended information for the address
    pub extended_address: Option<String>,
    /// the city/town
    pub city: String,
    /// the region code; for example, the state or province.
    pub region: Option<String>,
    /// the postal code (ZIP code)
    pub postal_code: String,
    /// the ISO country code (ISO 3166-1 alpha-3)
    pub country: CountryCode,
}

impl Address {
    /// the street address
    pub fn street_address(&self) -> &str {
        &self.street_address
    }

    /// the (optional) extended information for the address
    pub fn extended_address(&self) -> Option<&str> {
        self.extended_address.as_deref()
    }

    /// the city/town
    pub fn city(&self) -> &str {
        &self.city
    }

    /// the region code; for example, the state or province.
    pub fn region(&self) -> Option<&str> {
        self.region.as_deref()
    }

    /// the postal code (ZIP code)
    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    /// the ISO country code (ISO 3166-1 alpha-3)
    pub fn country_code(&self) -> CountryCode {
        self.country
    }

    /// Creates a new address builder
    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}

/// A physical street address builder
#[derive(Default)]
pub struct AddressBuilder {
    street_address: Option<String>,
    extended_address: Option<String>,
    city: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
    country_code: Option<CountryCode>,
}

impl AddressBuilder {
    /// the street address
    pub fn street_address(mut self, street_address: &str) -> AddressBuilder {
        self.street_address = Some(street_address.to_owned());
        self
    }

    /// the (optional) extended information for the address
    pub fn extended_address(mut self, extended_address: &str) -> AddressBuilder {
        self.extended_address = Some(extended_address.to_owned());
        self
    }

    /// the city/town
    pub fn city(mut self, city: &str) -> AddressBuilder {
        self.city = Some(city.to_owned());
        self
    }

    /// the region code
    pub fn region(mut self, region: &str) -> AddressBuilder {
        self.region = Some(region.to_owned());
        self
    }

    /// the postal code (ZIP code)
    pub fn postal_code(mut self, postal_code: &str) -> AddressBuilder {
        self.postal_code = Some(postal_code.to_owned());
        self
    }

    /// the ISO country code (ISO 3166-1 alpha-3)
    pub fn country(mut self, country_code: CountryCode) -> AddressBuilder {
        self.country_code = Some(country_code);
        self
    }

    pub fn build(self) -> Result<Address, AddressBuilderError> {
        let street_address = self.street_address.ok_or(AddressBuilderError::MissingStreetAddress)?;
        let extended_address = self.extended_address;
        let city = self.city.ok_or(AddressBuilderError::MissingCity)?;
        let region = self.region;
        let postal_code = self.postal_code.ok_or(AddressBuilderError::MissingPostalCode)?;
        let country_code = self.country_code.ok_or(AddressBuilderError::MissingCountry)?;

        Ok(Address {
            street_address,
            extended_address,
            city,
            region,
            postal_code,
            country: country_code,
        })
    }
}

#[derive(Debug, Error, PartialEq, Eq, Copy, Clone)]
pub enum AddressBuilderError {
    #[error("street address is required")]
    MissingStreetAddress,
    #[error("city is required")]
    MissingCity,
    #[error("postal code is required")]
    MissingPostalCode,
    #[error("country is required")]
    MissingCountry,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod addresses {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_create_new_addresses() {
            let address = Address::builder()
                .street_address("22 acacia avenue")
                .extended_address("Apt. 999")
                .region("Essex")
                .city("London")
                .country(CountryCode::GBR)
                .postal_code("123456")
                .build()
                .unwrap();

            assert_eq!("22 acacia avenue", address.street_address());
            assert_eq!(Some("Apt. 999"), address.extended_address());
            assert_eq!(Some("Essex"), address.region());
            assert_eq!("London", address.city());
            assert_eq!(CountryCode::GBR, address.country_code());
            assert_eq!("123456", address.postal_code());
        }

        #[rstest]
        #[case(
            None,
            Some("postal_code"),
            Some("city"),
            Some(CountryCode::ITA),
            Err(AddressBuilderError::MissingStreetAddress)
        )]
        #[case(
            Some("street_address"),
            None,
            Some("city"),
            Some(CountryCode::ITA),
            Err(AddressBuilderError::MissingPostalCode)
        )]
        #[case(
            Some("street_address"),
            Some("postal_code"),
            None,
            Some(CountryCode::ITA),
            Err(AddressBuilderError::MissingCity)
        )]
        #[case(
            Some("street_address"),
            Some("postal_code"),
            Some("city"),
            None,
            Err(AddressBuilderError::MissingCountry)
        )]
        fn it_should_validate_the_required_value(
            #[case] street_address: Option<&str>,
            #[case] postal_code: Option<&str>,
            #[case] city: Option<&str>,
            #[case] country: Option<CountryCode>,
            #[case] expected: Result<Address, AddressBuilderError>,
        ) {
            let mut address_builder = Address::builder();

            if let Some(street_address) = street_address {
                address_builder = address_builder.street_address(street_address);
            }
            if let Some(postal_code) = postal_code {
                address_builder = address_builder.postal_code(postal_code);
            }
            if let Some(city) = city {
                address_builder = address_builder.city(city);
            }
            if let Some(country) = country {
                address_builder = address_builder.country(country);
            }

            let result = address_builder.build();
            assert_eq!(expected, result);
        }
    }
}

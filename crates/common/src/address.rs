use isocountry::CountryCode;

/// A physical street address
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    street_address: String,
    extended_address: Option<String>,
    city: String,
    region: Option<String>,
    postal_code: String,
    country_code: CountryCode,
}

impl Address {
    /// The street address
    pub fn street_address(&self) -> &str {
        &self.street_address
    }

    /// The extended address of the address; for example, the apartment number.
    pub fn extended_address(&self) -> Option<&str> {
        self.extended_address.as_deref()
    }

    /// The city of the address.
    pub fn city(&self) -> &str {
        &self.city
    }

    /// The region of the address; for example, the state or province.
    pub fn region(&self) -> Option<&str> {
        self.region.as_deref()
    }

    /// The postal code of the address.
    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    /// The ISO 3166-1 alpha-3 country code of the address.
    pub fn country_code(&self) -> CountryCode {
        self.country_code
    }

    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}

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
    pub fn street_address(mut self, street_address: &str) -> AddressBuilder {
        self.street_address = Some(street_address.to_owned());
        self
    }

    pub fn extended_address(mut self, extended_address: &str) -> AddressBuilder {
        self.extended_address = Some(extended_address.to_owned());
        self
    }

    pub fn city(mut self, city: &str) -> AddressBuilder {
        self.city = Some(city.to_owned());
        self
    }

    pub fn region(mut self, region: &str) -> AddressBuilder {
        self.region = Some(region.to_owned());
        self
    }

    pub fn postal_code(mut self, postal_code: &str) -> AddressBuilder {
        self.postal_code = Some(postal_code.to_owned());
        self
    }

    pub fn country_code(mut self, country_code: CountryCode) -> AddressBuilder {
        self.country_code = Some(country_code);
        self
    }

    pub fn build(self) -> Result<Address, &'static str> {
        let street_address = self.street_address.ok_or("street address is required")?;
        let extended_address = self.extended_address;
        let city = self.city.ok_or("city is required")?;
        let region = self.region;
        let postal_code = self.postal_code.ok_or("postal code is required")?;
        let country_code = self.country_code.ok_or("country code is required")?;

        Ok(Address {
            street_address,
            extended_address,
            city,
            region,
            postal_code,
            country_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod addresses {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_addresses() {
            let address = Address::builder()
                .street_address("22 acacia avenue")
                .extended_address("Apt. 999")
                .region("Essex")
                .city("London")
                .country_code(CountryCode::GBR)
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
    }
}

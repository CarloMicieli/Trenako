use crate::common::TrackGauge;
use common::length::Length;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RailwayGauge {
    /// the distance between the rails in meters
    pub meters: Length,
    /// the track gauge
    pub track_gauge: TrackGauge,
}

impl RailwayGauge {
    /// Creates a new railway gauge
    pub fn new(meters: Decimal, track_gauge: TrackGauge) -> Self {
        RailwayGauge {
            meters: Length::Meters(meters),
            track_gauge,
        }
    }

    /// Creates a new standard railway gauge
    ///
    /// # Details
    /// A standard-gauge railway is a railway with a track gauge of 1,435 mm (4 ft 8+1⁄2 in).
    /// The standard gauge is also called Stephenson gauge (after George Stephenson),
    /// International gauge, UIC gauge, uniform gauge, normal gauge and European gauge in Europe,
    /// and SGR in East Africa. It is the most widely used track gauge around the world, with
    /// approximately 55% of the lines in the world using it. All high-speed rail lines use standard
    /// gauge except those in Russia, Finland, and Uzbekistan. The distance between the inside edges
    /// of the rails is defined to be 1435 mm except in the United States and on some heritage
    /// British lines, where it is defined in U.S. customary/Imperial units as exactly "four feet
    /// eight and one half inches" which is equivalent to 1435.1 mm.
    pub fn standard() -> Self {
        RailwayGauge::new(dec!(1.435), TrackGauge::Standard)
    }

    /// Creates a new narrow meter railway gauge
    ///
    /// # Details
    /// Metre-gauge railways are narrow-gauge railways with track gauge of 1,000 mm (3 ft 3+3⁄8 in) or 1 metre.
    ///
    /// The metre gauge is used in around 95,000 kilometres (59,000 mi) of tracks around the world.
    /// It was used by European colonial powers, such as the French, British and German Empires.
    /// In Europe, large metre-gauge networks remain in use in Switzerland, Spain and many European
    /// towns with urban trams, but most metre-gauge local railways in France, Germany and Belgium
    /// closed down in the mid-20th century, although many still remain. With the revival of urban
    /// rail transport, metre-gauge light metros were established in some cities, and in other cities,
    /// metre gauge was replaced by standard gauge. The slightly-wider 1,009 mm (3 ft 3+23⁄32 in)
    /// gauge is used in Sofia.
    pub fn metre() -> Self {
        RailwayGauge::new(dec!(1.0), TrackGauge::Narrow)
    }

    /// Returns the distance between the two rails of a railway track in meters
    pub fn meters(&self) -> Length {
        self.meters
    }

    /// Returns the track gauge for this railway company
    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }
}

impl Serialize for RailwayGauge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RailwayGauge", 2)?;
        state.serialize_field("meters", &self.meters.quantity())?;
        state.serialize_field("track_gauge", &self.track_gauge)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RailwayGauge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Meters,
            TrackGauge,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`meters` or `track_gauge`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "meters" => Ok(Field::Meters),
                            "track_gauge" => Ok(Field::TrackGauge),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RailwayGaugeVisitor;

        impl<'de> Visitor<'de> for RailwayGaugeVisitor {
            type Value = RailwayGauge;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct RailwayGauge")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<RailwayGauge, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let meters = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let track_gauge = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(RailwayGauge::new(meters, track_gauge))
            }

            fn visit_map<V>(self, mut map: V) -> Result<RailwayGauge, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut meters = None;
                let mut track_gauge = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Meters => {
                            if meters.is_some() {
                                return Err(de::Error::duplicate_field("meters"));
                            }
                            meters = Some(map.next_value()?);
                        }
                        Field::TrackGauge => {
                            if track_gauge.is_some() {
                                return Err(de::Error::duplicate_field("track_gauge"));
                            }
                            track_gauge = Some(map.next_value()?);
                        }
                    }
                }
                let meters = meters.ok_or_else(|| de::Error::missing_field("meters"))?;
                let track_gauge = track_gauge.ok_or_else(|| de::Error::missing_field("track_gauge"))?;
                Ok(RailwayGauge::new(meters, track_gauge))
            }
        }

        const FIELDS: &[&str] = &["meters", "track_gauge"];
        deserializer.deserialize_struct("RailwayGauge", FIELDS, RailwayGaugeVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_gauges {
        use super::*;
        use crate::common::TrackGauge;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_railway_gauges() {
            let gauge = RailwayGauge::new(dec!(1.435), TrackGauge::Standard);
            assert_eq!(dec!(1.435), gauge.meters().quantity());
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
        }

        #[test]
        fn it_should_create_a_standard_railway_gauges() {
            let gauge = RailwayGauge::standard();
            assert_eq!(dec!(1.435), gauge.meters().quantity());
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
        }

        #[test]
        fn it_should_create_a_metre_railway_gauges() {
            let gauge = RailwayGauge::metre();
            assert_eq!(dec!(1.0), gauge.meters().quantity());
            assert_eq!(TrackGauge::Narrow, gauge.track_gauge());
        }

        #[test]
        fn it_should_serialize_railway_gauges_as_json() {
            let length = Length::Meters(dec!(1.435));
            let value = TestStruct {
                railway_gauge: RailwayGauge {
                    meters: length,
                    track_gauge: TrackGauge::Standard,
                },
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            assert_eq!(r#"{"railway_gauge":{"meters":"1.435","track_gauge":"STANDARD"}}"#, json);
        }

        #[test]
        fn it_should_deserialize_railway_gauges_from_json() {
            let json = r#"{"railway_gauge":{"meters":"1.435","track_gauge":"STANDARD"}}"#;
            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            let length = Length::Meters(dec!(1.435));

            assert_eq!(length, test_struct.railway_gauge.meters);
            assert_eq!(TrackGauge::Standard, test_struct.railway_gauge.track_gauge);
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            railway_gauge: RailwayGauge,
        }
    }
}

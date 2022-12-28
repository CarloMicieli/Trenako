use crate::common::TrackGauge;
use common::length::Length;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct RailwayGauge {
    meters: Length,
    track_gauge: TrackGauge,
}

impl RailwayGauge {
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

    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
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
    }
}

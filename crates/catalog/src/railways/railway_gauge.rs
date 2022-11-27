use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RailwayGauge {
    meters: Decimal,
    track_gauge: TrackGauge,
}

impl RailwayGauge {
    pub fn new(meters: Decimal, track_gauge: TrackGauge) -> Self {
        RailwayGauge { meters, track_gauge }
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
        RailwayGauge {
            meters: dec!(1.435),
            track_gauge: TrackGauge::Standard,
        }
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
        RailwayGauge {
            meters: dec!(1.0),
            track_gauge: TrackGauge::Narrow,
        }
    }

    /// Returns the distance between the two rails of a railway track in meters
    pub fn meters(&self) -> Decimal {
        self.meters
    }

    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }
}

/// In rail transport, track gauge is the distance between the two rails of a railway track.
/// All vehicles on a rail network must have wheel sets that are compatible with the track gauge.
///
/// Since many different track gauges exist worldwide, gauge differences often present a barrier to wider operation on
/// railway networks.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TrackGauge {
    /// In modern usage, the term "broad gauge" generally refers to track spaced significantly wider than
    /// 1,435 mm (4 ft 8+1⁄2 inches).
    ///
    /// Broad gauge is the dominant gauge in countries in Indian subcontinent, the former Soviet Union (CIS states,
    /// Baltic states, Georgia and Ukraine), Mongolia and Finland, Spain, Portugal, Argentina, Chile and Ireland.
    /// It is also use for the suburban railway systems in South Australia, and Victoria, Australia.
    Broad,

    /// The term "medium gauge" had different meanings throughout history, depending on the local dominant gauge in use.
    Medium,

    /// Very narrow gauges of under 2 feet (610 mm) were used for some industrial railways in space-restricted
    /// environments such as mines or farms. The French company Decauville developed 500 mm (19+3⁄4 in) and
    /// 400 mm (15+3⁄4 in) tracks, mainly for mines; Heywood developed 15 in (381 mm) gauge for estate railways.
    /// The most common minimum-gauges were 15 in (381 mm), 400 mm (15+3⁄4 in), 16 in (406 mm), 18 in (457 mm),
    /// 500 mm (19+3⁄4 in) or 20 in (508 mm).
    Minimum,

    /// In modern usage, the term "narrow gauge" generally refers to track spaced significantly narrower than 1,435 mm
    /// (4 ft 8+1⁄2 in).
    Narrow,

    /// In modern usage the term "standard gauge" refers to 1,435 mm (4 ft 8+1⁄2 inches).
    /// Standard gauge is dominant in a majority of countries, including those in North America, most of western Europe,
    /// North Africa and the Middle east, and in China.
    Standard,
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_gauges {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_railway_gauges() {
            let gauge = RailwayGauge::new(dec!(1.435), TrackGauge::Standard);
            assert_eq!(dec!(1.435), gauge.meters());
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
        }

        #[test]
        fn it_should_create_a_standard_railway_gauges() {
            let gauge = RailwayGauge::standard();
            assert_eq!(dec!(1.435), gauge.meters());
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
        }

        #[test]
        fn it_should_create_a_metre_railway_gauges() {
            let gauge = RailwayGauge::metre();
            assert_eq!(dec!(1.0), gauge.meters());
            assert_eq!(TrackGauge::Narrow, gauge.track_gauge());
        }
    }
}

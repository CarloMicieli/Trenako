use strum_macros;
use strum_macros::{Display, EnumString};

/// In rail transport, track gauge is the distance between the two rails of a railway track.
/// All vehicles on a rail network must have wheel sets that are compatible with the track gauge.
///
/// Since many different track gauges exist worldwide, gauge differences often present a barrier to wider operation on
/// railway networks.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
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

    mod track_gauges {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("BROAD", Ok(TrackGauge::Broad))]
        #[case("MEDIUM", Ok(TrackGauge::Medium))]
        #[case("MINIMUM", Ok(TrackGauge::Minimum))]
        #[case("NARROW", Ok(TrackGauge::Narrow))]
        #[case("STANDARD", Ok(TrackGauge::Standard))]
        fn it_should_parse_strings_as_track_gauges(
            #[case] input: &str,
            #[case] expected: Result<TrackGauge, ParseError>,
        ) {
            let gauge = input.parse::<TrackGauge>();
            assert_eq!(expected, gauge);
        }

        #[rstest]
        #[case(TrackGauge::Broad, "BROAD")]
        #[case(TrackGauge::Medium, "MEDIUM")]
        #[case(TrackGauge::Minimum, "MINIMUM")]
        #[case(TrackGauge::Narrow, "NARROW")]
        #[case(TrackGauge::Standard, "STANDARD")]
        fn it_should_display_track_gauges(#[case] input: TrackGauge, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}

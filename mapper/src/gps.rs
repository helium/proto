use super::*;

use chrono::{prelude::*, DateTime, NaiveDateTime};

use rust_decimal::Decimal;

pub const ZERO_DECIMAL: Decimal = Decimal::from_parts(0, 0, 0, false, 0);

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct GpsData {
    /// UTC of position fix
    pub timestamp: DateTime<Utc>,
    /// Latitude in degrees
    pub lat: Decimal,
    /// Longitude in degrees
    pub lon: Decimal,
    /// Horizontal dilution of position
    pub hdop: Decimal,
    /// Height of geoid (mean sea level) above WGS84 ellipsoid
    pub altitude: Decimal,
    /// Number of satellites in use
    pub num_sats: u8,
    /// Speed over ground (SoG), km/h
    pub speed: Decimal,
}

use nmea_parser::gnss::{GgaData, VtgData};

impl GpsData {
    pub fn is_parseable(gga: &GgaData, vtg: &VtgData) -> bool {
        gga.timestamp.is_some()
            && gga.latitude.is_some()
            && gga.longitude.is_some()
            && gga.hdop.is_some()
            && gga.geoid_separation.is_some()
            && gga.satellite_count.is_some()
            && vtg.sog_kph.is_some()
    }

    pub fn is_locked(&self) -> bool {
        self.num_sats >= 3 && self.hdop > ZERO_DECIMAL
    }

    pub fn to_h3_cell(&self) -> Result<h3o::CellIndex> {
        use h3o::{LatLng, Resolution};

        use rust_decimal::prelude::ToPrimitive;
        let coord = LatLng::new(self.lat.to_f64().unwrap(), self.lat.to_f64().unwrap())?;
        Ok(coord.to_cell(Resolution::Fifteen))
    }

    pub fn new(gga: &GgaData, vtg: &VtgData, date: &str) -> Result<Option<GpsData>> {
        let day = date[..2].parse::<u32>()?;
        let month = date[2..4].parse::<u32>()?;
        let year = date[4..].parse::<i32>()? + 2000;
        Ok(
            if let (
                Some(timestamp),
                Some(latitude),
                Some(longitude),
                Some(hdop),
                Some(altitude),
                Some(num_sats),
                Some(speed),
            ) = (
                gga.timestamp,
                gga.latitude,
                gga.longitude,
                gga.hdop,
                gga.altitude,
                gga.satellite_count,
                vtg.sog_kph,
            ) {
                Some(GpsData {
                    timestamp: Utc
                        .with_ymd_and_hms(
                            year,
                            month,
                            day,
                            timestamp.hour(),
                            timestamp.minute(),
                            timestamp.second(),
                        )
                        .unwrap(),
                    lat: Decimal::new((latitude * 1000000.0) as i64, 6),
                    lon: Decimal::new((longitude * 1000000.0) as i64, 6),
                    hdop: Decimal::new((hdop * 100.0) as i64, 2),
                    altitude: Decimal::new((altitude * 100.0) as i64, 2),
                    num_sats,
                    speed: Decimal::new((speed * 100.0) as i64, 2),
                })
            } else {
                None
            },
        )
    }
}

impl From<GpsData> for helium_proto::MapperGpsV1 {
    fn from(gps_data: GpsData) -> helium_proto::MapperGpsV1 {
        use latlon::{Degrees};

        helium_proto::MapperGpsV1 {
            timestamp: time::to_units(gps_data.timestamp),
            lat: latlon::to_units(Degrees::Lat(gps_data.lat)),
            lon: latlon::to_units(Degrees::Lon(gps_data.lon)),
            hdop: hdop::to_units(gps_data.hdop),
            altitude: altitude::to_units(gps_data.altitude),
            num_sats: gps_data.num_sats as u32,
            speed: speed::to_units(gps_data.speed),
        }
    }
}

impl From<helium_proto::MapperGpsV1> for GpsData {
    fn from(gps_proto: helium_proto::MapperGpsV1) -> GpsData {
        use latlon::Unit;

        GpsData {
            timestamp: time::from_units(gps_proto.timestamp),
            lat: latlon::from_units(Unit::Lat(gps_proto.lat)),
            lon: latlon::from_units(Unit::Lon(gps_proto.lon)),
            hdop: hdop::from_units(gps_proto.hdop),
            altitude: altitude::from_units(gps_proto.altitude),
            num_sats: gps_proto.num_sats as u8,
            speed: speed::from_units(gps_proto.speed),
        }
    }
}

pub(crate) mod hdop {
    use super::*;

    pub fn to_units(hdop: Decimal) -> u32 {
        let multiplier = Decimal::new(100, 0);
        let scaled = hdop.checked_mul(multiplier).unwrap().round();
        scaled.to_string().parse::<u32>().unwrap()
    }

    pub(crate) fn from_units(hdop: u32) -> Decimal {
        Decimal::new(hdop.into(), 2)
    }
}

pub(crate) mod time {
    use super::*;
    // time for 2023-01-01 00:00:00 UTC
    const REFERENCE: i64 = 1672531200;

    pub fn to_units(datetime: DateTime<Utc>) -> u32 {
        (datetime.timestamp() - REFERENCE) as u32
    }

    pub fn from_units(timestamp: u32) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(timestamp as i64 + REFERENCE, 0).unwrap(),
            Utc,
        )
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use chrono::TimeZone;

        #[test]
        fn time_to_units() {
            let datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 5).unwrap();
            assert_eq!(to_units(datetime), 5);
        }

        #[test]
        fn time_from_units() {
            let datetime = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 5).unwrap();
            assert_eq!(from_units(5), datetime);
        }
    }
}

pub(crate) mod latlon {
    use super::*;

    const LAT_OFFSET: Decimal = Decimal::from_parts(9000000, 0, 0, false, 5);
    const LON_OFFSET: Decimal = Decimal::from_parts(18000000, 0, 0, false, 5);

    pub(crate) enum Degrees {
        Lat(Decimal),
        Lon(Decimal),
    }

    pub(crate) enum Unit {
        Lat(u32),
        Lon(u32),
    }

    pub(crate) fn to_units(coordinate: Degrees) -> u32 {
        let offset_degrees = match coordinate {
            Degrees::Lat(lat) => lat + LAT_OFFSET,
            Degrees::Lon(lon) => lon + LON_OFFSET,
        };
        let multiplier = Decimal::new(100000, 0);
        let scaled = offset_degrees.checked_mul(multiplier).unwrap().round();
        scaled.to_string().parse::<u32>().unwrap()
    }

    pub(crate) fn from_units(unit: Unit) -> Decimal {
        match unit {
            // (-90, 90)
            Unit::Lat(lat) => Decimal::new(lat.into(), 5) - LAT_OFFSET,
            // (-180, 180)
            Unit::Lon(lon) => Decimal::new(lon.into(), 5) - LON_OFFSET,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand::Rng;

        #[test]
        fn lat_constant() {
            assert_eq!(LAT_OFFSET.to_string(), "90.00000");
        }

        #[test]
        fn lon_constant() {
            assert_eq!(LON_OFFSET.to_string(), "180.00000");
        }

        #[test]
        fn roundtrip_lat() {
            let mut rng = rand::thread_rng();
            let random_lat = rng.gen_range(-90_00000..90_00000);
            let lat = Decimal::new(random_lat, 5);
            let units = to_units(Degrees::Lat(lat));
            let degrees = from_units(Unit::Lat(units));
            assert_eq!(lat, degrees);
        }

        #[test]
        fn roundtrip_lon() {
            let mut rng = rand::thread_rng();
            let random_lon = rng.gen_range(-180_00000..180_00000);
            let lon = Decimal::new(random_lon, 5);
            let units = to_units(Degrees::Lon(lon));
            let degrees = from_units(Unit::Lon(units));
            assert_eq!(lon, degrees);
        }
    }
}

pub(crate) mod altitude {
    use super::*;
    #[allow(clippy::inconsistent_digit_grouping)]
    const ALTITUDE_OFFSET: Decimal = Decimal::from_parts(110_00, 0, 0, false, 2);
    #[allow(clippy::zero_prefixed_literal, clippy::inconsistent_digit_grouping)]
    const ALTITUDE_SCALAR: Decimal = Decimal::from_parts(0_25, 0, 0, false, 2);

    pub fn to_units(altitude: Decimal) -> u32 {
        let offset_altitude = altitude + ALTITUDE_OFFSET;
        let scaled = offset_altitude
            .checked_div(ALTITUDE_SCALAR)
            .unwrap()
            .round();
        scaled.to_string().parse::<u32>().unwrap()
    }

    pub fn from_units(altitude: u32) -> Decimal {
        let altitude_unscaled = Decimal::new(altitude.into(), 0);
        let altitude_unoffset = altitude_unscaled.checked_mul(ALTITUDE_SCALAR).unwrap();
        altitude_unoffset - ALTITUDE_OFFSET
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn altitude_offset() {
            assert_eq!(ALTITUDE_OFFSET.to_string(), "110.00");
        }

        #[test]
        fn altitude_lower_limit_roundtrip() {
            let altitude = Decimal::new(-110_00, 2);
            assert_eq!(altitude.to_string(), "-110.00");
            let units = to_units(altitude);
            assert_eq!(0, units);
            let altitude = from_units(units);
            assert_eq!(altitude.to_string(), "-110.00");
        }

        #[test]
        fn altitude_zero_roundtrip() {
            let altitude = Decimal::new(0, 2);
            assert_eq!(altitude.to_string(), "0.00");
            let units = to_units(altitude);
            assert_eq!(110_00 / 25, units);
            let altitude = from_units(units);
            assert_eq!(altitude.to_string(), "0.00");
        }

        #[test]
        fn altitude_round_down() {
            let altitude = Decimal::new(10_12, 2);
            assert_eq!(altitude.to_string(), "10.12");
            let altitude = from_units(to_units(altitude));
            assert_eq!(altitude.to_string(), "10.00");
        }

        #[test]
        fn altitude_round_up() {
            let altitude = Decimal::new(10_21, 2);
            assert_eq!(altitude.to_string(), "10.21");
            let altitude = from_units(to_units(altitude));
            assert_eq!(altitude.to_string(), "10.25");
        }
    }
}

pub(crate) mod speed {
    use super::*;
    #[allow(clippy::zero_prefixed_literal, clippy::inconsistent_digit_grouping)]
    const SPEED_SCALAR: Decimal = Decimal::from_parts(0_25, 0, 0, false, 2);

    pub fn to_units(speed: Decimal) -> u32 {
        let scaled = speed.checked_div(SPEED_SCALAR).unwrap().round();
        scaled.to_string().parse::<u32>().unwrap()
    }

    pub fn from_units(speed: u32) -> Decimal {
        let speed_unscaled = Decimal::new(speed.into(), 0);
        speed_unscaled.checked_mul(SPEED_SCALAR).unwrap()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn speed_upper_limit_roundtrip() {
            let speed = Decimal::new(80_00, 2);
            assert_eq!(speed.to_string(), "80.00");
            let units = to_units(speed);
            assert_eq!(80_00 / 25, units);
            let speed = from_units(units);
            assert_eq!(speed.to_string(), "80.00");
        }

        #[test]
        fn speed_round_down() {
            let altitude = Decimal::new(20_12, 2);
            assert_eq!(altitude.to_string(), "20.12");
            let altitude = from_units(to_units(altitude));
            assert_eq!(altitude.to_string(), "20.00");
        }

        #[test]
        fn speed_round_up() {
            let altitude = Decimal::new(20_13, 2);
            assert_eq!(altitude.to_string(), "20.13");
            let altitude = from_units(to_units(altitude));
            assert_eq!(altitude.to_string(), "20.25");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latlon_from_degrees() {
        // -122.281358
        let degrees = Decimal::new(-122281358, 6);
        let latlon_units = latlon_units_from_degrees(degrees);
        // -12228136
        assert_eq!(latlon_units, -12228136);
    }

    #[test]
    fn hdop_roundtrip() {
        let hdop = Decimal::new(123, 2);
        assert_eq!(hdop.to_string(), "1.23"); // 1.23 m
        let hdop_units = units_from_hdop(hdop);
        assert_eq!(hdop_units, 123);
        let hdop_roundtrip = hdop_from_units(hdop_units);
        assert_eq!(hdop_roundtrip, hdop);
    }

    #[test]
    /// The roundtrip is expected to round to the closes 0.25m
    fn altitude_roundtrip() {
        let altitude = Decimal::new(-4979, 2);
        assert_eq!(altitude.to_string(), "-49.79"); // -49.79 m
        let altitude_units = units_from_altitude(altitude);
        assert_eq!(altitude_units, -4979 / 25);
        let altitude_roundtrip = altitude_from_units(altitude_units);
        assert_eq!(altitude_roundtrip.to_string(), "-49.75");
    }

    #[test]
    /// The roundtrip is expected to round to the closes 0.25 km
    fn speed_roundtrip() {
        let speed = Decimal::new(5524, 2);
        assert_eq!(speed.to_string(), "55.24"); // 55.24 km/h
        let speed_units = units_from_speed(speed);
        assert_eq!(speed_units, 5524 / 25 + 1);
        let speed_roundtrip = speed_from_units(speed_units);
        assert_eq!(speed_roundtrip.to_string(), "55.25");
    }
}

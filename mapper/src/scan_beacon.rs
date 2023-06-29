use super::gps::{altitude, hdop, latlon, speed, time};
use crate::Gps;
use modular_bitfield_msb::{bitfield, specifiers::*};

#[derive(PartialEq, Debug, Clone)]
pub struct ScanBeacon {
    gps: Gps,
    hash: Vec<u8>,
}

impl ScanBeacon {
    pub fn new(gps: Gps, signature: Vec<u8>) -> Self {
        Self {
            gps,
            hash: signature,
        }
    }

    pub fn into_bytes(self) -> [u8; 17] {
        let lora_payload: LoraPayload = self.into();
        lora_payload.into_bytes()
    }

    pub fn from_bytes(bytes: [u8; 17]) -> Self {
        let lora_payload = LoraPayload::from_bytes(bytes);
        lora_payload.into()
    }
}

impl From<LoraPayload> for ScanBeacon {
    fn from(lora_payload: LoraPayload) -> Self {
        use latlon::Unit;
        Self {
            gps: Gps {
                timestamp: time::from_lora_units(lora_payload.time()),
                lat: latlon::from_lora_units(Unit::Lat(lora_payload.lat())),
                lon: latlon::from_lora_units(Unit::Lon(lora_payload.lon())),
                hdop: hdop::from_units(lora_payload.hdop().into()),
                altitude: altitude::from_lora_units(lora_payload.alt().into()),
                num_sats: lora_payload.num_sats(),
                speed: speed::from_lora_units(lora_payload.speed().into()),
            },
            hash: lora_payload.hash().to_be_bytes().to_vec(),
        }
    }
}

impl From<ScanBeacon> for LoraPayload {
    fn from(p: ScanBeacon) -> Self {
        use latlon::Degrees;
        LoraPayload::new()
            .with_time(time::to_lora_units(p.gps.timestamp))
            .with_lat(latlon::to_lora_units(Degrees::Lat(p.gps.lat)))
            .with_lon(latlon::to_lora_units(Degrees::Lon(p.gps.lon)))
            .with_hdop(hdop::to_units(p.gps.hdop) as u16)
            .with_alt(altitude::to_lora_units(p.gps.altitude) as u16)
            .with_speed(speed::to_lora_units(p.gps.speed) as u16)
            .with_num_sats(p.gps.num_sats)
            .with_hash(u16::from_be_bytes([p.hash[0], p.hash[1]]))
    }
}

#[bitfield]
struct LoraPayload {
    // we take seconds from 2023-01-01 00:00:00 UTC
    // 30 bits gives us over 20 years
    time: B30,
    // lat ranges from -90 to 90 w/ 5 decimal places (1.11 m accuracy)
    // shifted to a uint, ranges up to 18000000 => 25 bits
    lat: B25,
    // lon ranges from -180 to 180 w/ 5 decimal places (1.11 m accuracy)
    // shifted to a uint, ranges up to 36000000 => 26 bits
    lon: B26,
    // we will not send HDOP values greater than 10m
    // 0.01m increments => 1000 possible values => 10 bits
    hdop: B10,
    // WGS-84 on the surface of earth ranges from +85m (Iceland) to -106m (India)
    // (https://en.wikipedia.org/wiki/Geoid)
    // We will represent this in 0.25m steps shifted to a uint by 110m => 0-780 values => 10 bits
    alt: B10,
    // Will never exceed 80 km/h. We will represent in 0.25m/h steps => 0-320 values => 9 bits
    speed: B9,
    // 0-12 sats => 4 bits
    num_sats: B4,
    // truncated signature of the scan payload
    hash: B16,
    // padding for the struct is necessary to make it byte aligned
    #[allow(unused)]
    padding: B6,
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;

    #[test]
    fn payload_roundtrip_lora() {
        use chrono::TimeZone;
        let timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 5).unwrap();

        let payload = ScanBeacon {
            gps: Gps {
                timestamp,
                lat: Decimal::new(-50_12345, 5),
                lon: Decimal::new(120_12345, 5),
                hdop: Decimal::new(10_05, 2),
                altitude: Decimal::new(10_25, 2),
                num_sats: 5,
                speed: Decimal::new(50_50, 2),
            },
            hash: vec![0xAB, 0xCD],
        };
        let lora_payload = LoraPayload::from(payload.clone());
        let bytes = lora_payload.into_bytes();
        let payload_returned = ScanBeacon::from_bytes(bytes);
        assert_eq!(payload, payload_returned);
    }
}

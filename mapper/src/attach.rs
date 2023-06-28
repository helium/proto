use super::*;
use super::gps::{hdop, latlon, time, altitude, speed};

#[derive(Serialize, Debug, Clone)]
pub struct MapperAttach {
    // This allows us to detect censorship efforts. It can roll over.
    pub attach_counter: u32,
    pub gps: GpsData,
    pub candidate: AttachCandidate,
    // did the attach succeed?
    pub result: AttachResult,
}

impl From<MapperAttach> for helium_proto::MapperAttachV1 {
    fn from(attach_candidate_result: MapperAttach) -> helium_proto::MapperAttachV1 {
        use helium_proto::mapper_attach_v1::MapperAttachResult as Result;

        helium_proto::MapperAttachV1 {
            attach_counter: attach_candidate_result.attach_counter,
            gps: Some(attach_candidate_result.gps.into()),
            candidate: Some(attach_candidate_result.candidate.into()),
            result: match attach_candidate_result.result {
                AttachResult::NoAttach => Result::None,
                AttachResult::Connected => Result::Connect,
                AttachResult::LimitedService => Result::LimitedService,
                AttachResult::NoConnection => Result::NoConnection,
                AttachResult::Search => Result::Search,
                AttachResult::NoNetworkService => Result::NoNetworkService,
            }
                .into(),
        }
    }
}

impl From<AttachCandidate> for helium_proto::mapper_attach_v1::MapperAttachCandidate {
    fn from(attach_candidate: AttachCandidate) -> Self {
        Self {
            r#type: 0,
            from_scan_response: attach_candidate.from_scan,
            delay: attach_candidate.delay,
            plmn: ((attach_candidate.mcc as u32) << 16) | attach_candidate.mnc as u32,
            fcn: attach_candidate.earfcn,
            cid: attach_candidate.cell_id,
            rsrp: (attach_candidate.rsrp * 100) as i32,
            rsrq: (attach_candidate.rsrq * 100) as i32,
        }
    }
}

#[derive(BitfieldSpecifier)]
pub enum CellTech {
    LTE,
    NR,
}

use modular_bitfield_msb::{bitfield, specifiers::*, BitfieldSpecifier};

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
    // TODO check size
    attach_counter: B32,
    // TODO check size
    scan_response: B32,
    // 1024 seconds should be enough for anyone?
    delay: B10,
    // UMTS cell id (28 bits)
    cid: B36,
    rsrp: B8,
    rsrq: B8,
    cell_type: CellTech,
    #[bits = 3]
    #[allow(dead_code)]
    result: AttachResult,
    // padding for the struct is necessary to make it byte aligned
    #[allow(unused)]
    padding: B6,
}

pub const RSRP_OFFSET: isize = 150;
pub const RSRQ_OFFSET: isize = 30;

impl From<MapperAttach> for LoraPayload {
    fn from(mapper_attach: MapperAttach) -> Self {
        use latlon::Degrees;

        LoraPayload::new()
            .with_time(time::to_units(mapper_attach.gps.timestamp))
            .with_lat(latlon::to_units(Degrees::Lat(mapper_attach.gps.lat)))
            .with_lon(latlon::to_units(Degrees::Lon(mapper_attach.gps.lon)))
            .with_hdop(hdop::to_units(mapper_attach.gps.hdop) as u16)
            .with_alt(altitude::to_units(mapper_attach.gps.altitude) as u16)
            .with_speed(speed::to_units(mapper_attach.gps.speed) as u16)
            .with_num_sats(mapper_attach.gps.num_sats)
            .with_delay(mapper_attach.candidate.delay as u16)
            .with_attach_counter(mapper_attach.attach_counter)
            .with_scan_response(mapper_attach.candidate.from_scan)
            .with_mcc(mapper_attach.candidate.mcc)
            .with_mnc(mapper_attach.candidate.mnc)
            .with_cid(mapper_attach.candidate.cell_id)
            .with_rsrp((mapper_attach.candidate.rsrp + RSRP_OFFSET) as u8)
            .with_rsrq((mapper_attach.candidate.rsrq + RSRQ_OFFSET) as u8)
            .with_fcn(mapper_attach.candidate.earfcn)
        // candidate type default to 0
    }
}

impl From<LoraPayload> for MapperAttach {
    fn from(p: LoraPayload) -> Self {
        use latlon::Unit;
        MapperAttach {
            gps: GpsData {
                timestamp: time::from_units(p.time()),
                lat: latlon::from_units(Unit::Lat(p.lat())),
                lon: latlon::from_units(Unit::Lon(p.lon())),
                hdop: hdop::from_units(p.hdop().into()),
                altitude: altitude::from_units(p.alt().into()),
                num_sats: p.num_sats(),
                speed: speed::from_units(p.speed().into()),
            },
            attach_counter: p.attach_counter(),
            candidate: AttachCandidate {
                delay: p.delay() as u32,
                from_scan: p.scan_response(),
                rsrp: (p.rsrp() as isize) - RSRP_OFFSET,
                rsrq: (p.rsrq() as isize) - RSRQ_OFFSET,
                cell_type: p.cell_type.into(),
            },

            result: p.result.into(),
        }

    }
}

impl MapperAttach {
    pub fn to_proto_serialization(&self) -> std::result::Result<Vec<u8>, helium_proto::EncodeError> {
        use helium_proto::Message;
        let proto = helium_proto::MapperAttachV1::from(self.clone());
        let mut buffer = vec![];
        proto.encode(&mut buffer)?;
        Ok(buffer)
    }
}

#[derive(Serialize, Debug, Clone, BitfieldSpecifier)]
#[bits = 3]
pub enum AttachResult {
    NoAttach,
    Connected,
    LimitedService,
    NoConnection,
    Search,
    NoNetworkService,
}

impl AttachResult {
    pub fn is_successful(&self) -> bool {
        !matches!(self, AttachResult::NoAttach)
    }
}

impl std::str::FromStr for AttachResult {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(AttachResult::NoAttach),
            "CONNECT" => Ok(AttachResult::Connected),
            "LIMSERV" => Ok(AttachResult::LimitedService),
            "NOCONN" => Ok(AttachResult::NoConnection),
            "SEARCH" => Ok(AttachResult::Search),
            _ => Err(Error::UnexpectedAttachResultStr(s.into())),
        }
    }
}

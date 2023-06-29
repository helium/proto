use super::gps::{altitude, hdop, latlon, speed, time};
use super::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CellAttach {
    // This allows us to detect censorship efforts. It can roll over.
    pub attach_counter: u32,
    pub gps: GpsData,
    pub candidate: AttachCandidate,
    // did the attach succeed?
    pub result: AttachResult,
}

impl CellAttach {
    pub fn into_bytes(self) -> [u8; 32] {
        let lora_payload: LoraPayload = self.into();
        lora_payload.into_bytes()
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        let lora_payload = LoraPayload::from_bytes(bytes);
        lora_payload.into()
    }
}

impl From<CellAttach> for LoraPayload {
    fn from(mapper_attach: CellAttach) -> Self {
        use latlon::Degrees;
        LoraPayload::new()
            .with_time(time::to_lora_units(mapper_attach.gps.timestamp))
            .with_lat(latlon::to_lora_units(Degrees::Lat(mapper_attach.gps.lat)))
            .with_lon(latlon::to_lora_units(Degrees::Lon(mapper_attach.gps.lon)))
            .with_hdop(hdop::to_units(mapper_attach.gps.hdop) as u16)
            .with_alt(altitude::to_lora_units(mapper_attach.gps.altitude) as u16)
            .with_speed(speed::to_lora_units(mapper_attach.gps.speed) as u16)
            .with_num_sats(mapper_attach.gps.num_sats)
            .with_delay(mapper_attach.candidate.delay as u16)
            .with_attach_counter(mapper_attach.attach_counter)
            .with_scan_response(mapper_attach.candidate.from_scan)
            .with_cid(mapper_attach.candidate.cell_id)
            .with_rsrp((mapper_attach.candidate.rsrp + RSRP_OFFSET) as u8)
            .with_rsrq((mapper_attach.candidate.rsrq + RSRQ_OFFSET) as u8)
            .with_fcn(mapper_attach.candidate.fcn)
            .with_result(mapper_attach.result.into())
    }
}

impl From<LoraPayload> for CellAttach {
    fn from(p: LoraPayload) -> Self {
        use latlon::Unit;
        CellAttach {
            gps: GpsData {
                timestamp: time::from_lora_units(p.time()),
                lat: latlon::from_lora_units(Unit::Lat(p.lat())),
                lon: latlon::from_lora_units(Unit::Lon(p.lon())),
                hdop: hdop::from_units(p.hdop().into()),
                altitude: altitude::from_lora_units(p.alt().into()),
                num_sats: p.num_sats(),
                speed: speed::from_lora_units(p.speed().into()),
            },
            attach_counter: p.attach_counter(),
            candidate: AttachCandidate {
                delay: p.delay() as u32,
                from_scan: p.scan_response(),
                rsrp: (p.rsrp() as i32) - RSRP_OFFSET,
                rsrq: (p.rsrq() as i32) - RSRQ_OFFSET,
                fcn: p.fcn() as u16,
                cell_id: p.cid() as u32,
            },

            result: p.result().into(),
        }
    }
}

impl From<CellAttach> for helium_proto::MapperCbrsAttachV1 {
    fn from(attach_candidate_result: CellAttach) -> helium_proto::MapperCbrsAttachV1 {
        use helium_proto::mapper_cbrs_attach_v1::MapperAttachResult as Result;

        helium_proto::MapperCbrsAttachV1 {
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

impl TryFrom<helium_proto::MapperCbrsAttachV1> for CellAttach {
    type Error = Error;
    fn try_from(attach: helium_proto::MapperCbrsAttachV1) -> Result<Self> {
        let result = match attach.result {
            0 => Ok(AttachResult::NoAttach),
            1 => Ok(AttachResult::Connected),
            2 => Ok(AttachResult::LimitedService),
            3 => Ok(AttachResult::NoConnection),
            4 => Ok(AttachResult::Search),
            5 => Ok(AttachResult::NoNetworkService),
            _ => Err(Error::InvalidAttachResultInt {
                value: attach.result,
            }),
        }?;
        Ok(Self {
            attach_counter: attach.attach_counter,
            gps: attach.gps.unwrap().into(),
            candidate: attach.candidate.unwrap().into(),
            result,
        })
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct AttachCandidate {
    pub from_scan: u32,
    pub delay: u32,
    pub cell_id: u32,
    pub fcn: u16,
    pub rsrp: i32,
    pub rsrq: i32,
}

impl From<CellScanResult> for AttachCandidate {
    fn from(scan_result: CellScanResult) -> Self {
        Self {
            from_scan: 0,
            delay: 0,
            cell_id: scan_result.cell_id as u32,
            fcn: scan_result.earfcn as u16,
            rsrp: scan_result.rsrp,
            rsrq: scan_result.rsrq,
        }
    }
}

impl From<AttachCandidate> for helium_proto::mapper_cbrs_attach_v1::MapperCbrsAttachCandidate {
    fn from(attach_candidate: AttachCandidate) -> Self {
        Self {
            from_scan: attach_candidate.from_scan,
            delay: attach_candidate.delay,
            fcn: attach_candidate.fcn as u32,
            cid: attach_candidate.cell_id,
            rsrp: attach_candidate.rsrp,
            rsrq: attach_candidate.rsrq,
        }
    }
}

impl From<helium_proto::mapper_cbrs_attach_v1::MapperCbrsAttachCandidate> for AttachCandidate {
    fn from(attach_candidate: helium_proto::mapper_cbrs_attach_v1::MapperCbrsAttachCandidate) -> Self {
        Self {
            from_scan: attach_candidate.from_scan,
            delay: attach_candidate.delay,
            fcn: attach_candidate.fcn as u16,
            cell_id: attach_candidate.cid,
            rsrp: attach_candidate.rsrp,
            rsrq: attach_candidate.rsrq,
        }
    }
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
    cid: B32,
    // E-UTRA absolute radio frequency channel number of the cell
    fcn: B16,
    // rsrp ranges from -140 to -44 dBm
    rsrp: B8,
    // rsrq ranges from -20 to -3 dBm
    rsrq: B8,
    #[bits = 3]
    #[allow(dead_code)]
    result: AttachResult,
    // padding for the struct is necessary to make it byte aligned
    #[allow(unused)]
    padding: B1,
}

pub const RSRP_OFFSET: i32 = 150;
pub const RSRQ_OFFSET: i32 = 30;

impl CellAttach {
    pub fn to_proto_serialization(
        &self,
    ) -> std::result::Result<Vec<u8>, helium_proto::EncodeError> {
        let proto = helium_proto::MapperCbrsAttachV1::from(self.clone());
        let mut buffer = vec![];
        proto.encode(&mut buffer)?;
        Ok(buffer)
    }
}

#[derive(Serialize, Debug, Clone, BitfieldSpecifier, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn payload_roundtrip_lora() {
        let payload = CellAttach {
            attach_counter: 5,
            gps: GpsData::rounded(),
            candidate: AttachCandidate::from(CellScanResult::random()),
            result: AttachResult::Connected,
        };

        let lora_payload = LoraPayload::from(payload.clone());
        let bytes = lora_payload.into_bytes();
        let payload_returned = CellAttach::from_bytes(bytes);
        assert_eq!(payload, payload_returned);
    }

    #[test]
    fn payload_roundtrip_proto() {
        let attach = CellAttach {
            attach_counter: 5,
            gps: GpsData::rounded(),
            candidate: AttachCandidate::from(CellScanResult::random()),
            result: AttachResult::Connected,
        };
        let proto: helium_proto::MapperCbrsAttachV1 = attach.clone().into();

        let mut proto_bytes = Vec::new();
        proto.encode(&mut proto_bytes).unwrap();
        let attach_returned = helium_proto::MapperCbrsAttachV1::decode(proto_bytes.as_slice())
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(attach, attach_returned);
    }
}

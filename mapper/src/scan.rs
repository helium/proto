use super::Result;

use crate::GpsData;
use serde::Serialize;

pub const CBRS_MCC: u16 = 315;
pub const CBRS_MNC: u16 = 10;

#[derive(Debug, Serialize, Clone)]
pub struct ScanResult {
    pub mcc: u16,
    pub mnc: u16,
    pub earfcn: u32,
    pub physical_cell_id: u64,
    pub rsrp: isize,
    pub rsrq: isize,
    pub rx_level: isize,
    pub quality: usize,
    pub cell_id: u64,
    pub bandwidth: usize,
    pub lte: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct AttachCandidate {
    pub from_scan: u32,
    pub delay: u32,
    pub cell_id: u32,
    pub fcn: u16,
    pub rsrp: isize,
    pub rsrq: isize,
}

impl From<ScanResult> for AttachCandidate {
    fn from(scan_result: ScanResult) -> Self {
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

impl ScanResult {
    pub fn is_our_network(&self) -> Result<bool> {
        if self.mcc == CBRS_MCC && self.mnc == CBRS_MNC {
            let top_20_bits = self.cell_id >> 8;
            // Helium cells all have a prefix in this range
            Ok((0x0099D..=0x00A00).contains(&top_20_bits))
        } else {
            Ok(false)
        }
    }
}

impl From<ScanResult> for helium_proto::MapperScanResult {
    fn from(scan_result: ScanResult) -> Self {
        Self {
            cid: scan_result.cell_id,
            plmn: ((scan_result.mcc as u32) << 16) | scan_result.mnc as u32,
            fcn: scan_result.earfcn,
            pci: scan_result.physical_cell_id as u32,
            rsrp: (scan_result.rsrp * 100) as i32,
            rsrq: (scan_result.rsrq * 100) as i32,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ScanResults {
    pub scan_counter: u32,
    pub gps: GpsData,
    pub results: Vec<ScanResult>,
}

impl From<ScanResults> for helium_proto::MapperScanV1 {
    fn from(scan_response: ScanResults) -> Self {
        Self {
            scan_counter: scan_response.scan_counter,
            gps: Some(scan_response.gps.into()),
            results: scan_response
                .results
                .into_iter()
                .map(|r| r.into())
                .collect(),
        }
    }
}



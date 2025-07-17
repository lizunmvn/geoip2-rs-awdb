use crate::decoder::{
    read_array, read_control, read_map, read_pointer, read_str, read_usize, Map, DATA_TYPE_MAP,
    DATA_TYPE_POINTER,
};
use crate::errors::Error;
use geoip2_awdb_codegen::Decoder;

// Metadata structure for GeoIP2 databases
const METADATA_START_MARKER: [u8; 14] = [
    0xAB, 0xCD, 0xEF, 0x4d, 0x61, 0x78, 0x4d, 0x69, 0x6e, 0x64, 0x2e, 0x63, 0x6f, 0x6d,
];
// Metadata structure for awdb databases
const METADATA_START_MARKER_AWDB: [u8; 16] = [
    0xAB, 0xCD, 0xEF, 0x69, 0x70, 0x70, 0x6C, 0x75, 0x73, 0x33, 0x36, 0x30, 0x2E, 0x63, 0x6F, 0x6D,
];

#[derive(Default, Debug, Decoder)]
pub struct Metadata<'a> {
    pub binary_format_major_version: u16,
    pub binary_format_minor_version: u16,
    pub node_count: u32,
    pub record_size: u16,
    pub ip_version: u16,
    pub database_type: &'a str,
    pub languages: Vec<&'a str>,
    pub build_epoch: u64,
    pub description: Map<'a>,
}

impl<'a> Metadata<'a> {
    // Metadata location for GeoIP2 databases
    pub(crate) fn find_start(buffer: &[u8]) -> Option<usize> {
        if buffer.len() < 14 {
            return None;
        }
        let mut i = buffer.len() - 14;
        while i != 0 {
            i -= 1;
            if buffer[i] == METADATA_START_MARKER[0]
                && buffer[i + 13] == METADATA_START_MARKER[13]
                && buffer[i..i + 14] == METADATA_START_MARKER
            {
                return Some(i + 14);
            }
        }
        None
    }

    // Metadata location for awdb databases
    pub(crate) fn find_start_awdb(buffer: &[u8]) -> Option<usize> {
        if buffer.len() < 16 {
            return None;
        }
        let mut i = buffer.len() - 16;
        while i != 0 {
            i -= 1;
            if buffer[i] == METADATA_START_MARKER_AWDB[0]
                && buffer[i + 15] == METADATA_START_MARKER_AWDB[15]
                && buffer[i..i + 16] == METADATA_START_MARKER_AWDB
            {
                return Some(i + 16);
            }
        }
        None
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

type Blockhash = Vec<u8>;

pub fn time_now () -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
    ;

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

pub fn u128_bytes (u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,

        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,

        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,

        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

pub fn difficulty_bytes_as_u128 (hash: &Vec<u8>) -> u128 {
    ((hash[31] as u128) << 0xf * 8) |
    ((hash[30] as u128) << 0xe * 8) |
    ((hash[29] as u128) << 0xd * 8) |
    ((hash[28] as u128) << 0xc * 8) |
    ((hash[27] as u128) << 0xb * 8) |
    ((hash[26] as u128) << 0xa * 8) |
    ((hash[25] as u128) << 0x9 * 8) |
    ((hash[24] as u128) << 0x8 * 8) |
    ((hash[23] as u128) << 0x7 * 8) |
    ((hash[22] as u128) << 0x6 * 8) |
    ((hash[21] as u128) << 0x5 * 8) |
    ((hash[20] as u128) << 0x4 * 8) |
    ((hash[19] as u128) << 0x3 * 8) |
    ((hash[18] as u128) << 0x2 * 8) |
    ((hash[17] as u128) << 0x1 * 8) |
    ((hash[16] as u128) << 0x0 * 8)
}


pub mod block;
pub mod hash_function;
pub mod blockchain;
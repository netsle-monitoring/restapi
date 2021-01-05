use std::num::NonZeroU32;

pub const DB_SALT_COMPONENT: [u8; 16] = [
    // This value was generated from a secure PRNG.
    0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x51,
    0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
];

pub const PBKDF2_ITERATIONS: NonZeroU32 = unsafe {NonZeroU32::new_unchecked(100_000)};


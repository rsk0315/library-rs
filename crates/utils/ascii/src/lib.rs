pub const ASCII              /* _ */: u128 = 0xffffffffffffffffffffffffffffffff;
pub const ASCII_ALPHABETIC   /* _ */: u128 = 0x07fffffe07fffffe0000000000000000;
pub const ASCII_ALPHANUMERIC /* _ */: u128 = 0x07fffffe07fffffe03ff000000000000;
pub const ASCII_CONTROL      /* _ */: u128 = 0x800000000000000000000000ffffffff;
pub const ASCII_DIGIT        /* _ */: u128 = 0x000000000000000003ff000000000000;
pub const ASCII_GRAPHIC      /* _ */: u128 = 0x7ffffffffffffffffffffffe00000000;
pub const ASCII_HEXDIGIT     /* _ */: u128 = 0x0000007e0000007e03ff000000000000;
pub const ASCII_LOWERCASE    /* _ */: u128 = 0x07fffffe000000000000000000000000;
pub const ASCII_PUNCTUATION  /* _ */: u128 = 0x78000001f8000001fc00fffe00000000;
pub const ASCII_UPPERCASE    /* _ */: u128 = 0x0000000007fffffe0000000000000000;
pub const ASCII_WHITESPACE   /* _ */: u128 = 0x00000000000000000000000100003600;

pub fn charset(b: &[u8]) -> u128 {
    let mut res = 0_u128;
    for &bi in b {
        res |= 1 << bi;
    }
    res
}

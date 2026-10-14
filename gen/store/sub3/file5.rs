// module store::sub3::file5
pub fn store_fn_3_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    // NEEDLE marker for search
    y ^ 319
}
pub fn store_fn_3_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(12);
    y ^ 958
}
pub fn store_fn_3_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(62);
    y ^ 17
}

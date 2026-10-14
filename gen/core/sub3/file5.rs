// module core::sub3::file5
pub fn core_fn_3_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(84);
    // NEEDLE marker for search
    y ^ 591
}
pub fn core_fn_3_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(89);
    y ^ 841
}

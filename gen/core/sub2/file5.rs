// module core::sub2::file5
pub fn core_fn_2_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(38);
    // NEEDLE marker for search
    y ^ 623
}
pub fn core_fn_2_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(11);
    y ^ 120
}

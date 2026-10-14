// module core::sub1::file5
pub fn core_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(83);
    // NEEDLE marker for search
    y ^ 192
}
pub fn core_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(49);
    y ^ 99
}
pub fn core_fn_1_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(72);
    y ^ 729
}

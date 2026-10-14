// module auth::sub1::file5
pub fn auth_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(13);
    // NEEDLE marker for search
    y ^ 668
}
pub fn auth_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(22);
    y ^ 407
}
pub fn auth_fn_1_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(91);
    y ^ 277
}

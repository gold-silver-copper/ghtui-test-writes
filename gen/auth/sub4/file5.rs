// module auth::sub4::file5
pub fn auth_fn_4_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    // NEEDLE marker for search
    y ^ 849
}
pub fn auth_fn_4_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(62);
    y ^ 187
}
pub fn auth_fn_4_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(74);
    y ^ 223
}

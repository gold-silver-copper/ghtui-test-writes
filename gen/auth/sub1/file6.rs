// module auth::sub1::file6
pub fn auth_fn_1_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(38);
    y ^ 683
}
pub fn auth_fn_1_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(41);
    y ^ 427
}

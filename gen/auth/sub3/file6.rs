// module auth::sub3::file6
pub fn auth_fn_3_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(98);
    y ^ 824
}
pub fn auth_fn_3_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    y ^ 308
}

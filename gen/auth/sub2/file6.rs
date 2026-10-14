// module auth::sub2::file6
pub fn auth_fn_2_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(60);
    y ^ 791
}
pub fn auth_fn_2_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(22);
    y ^ 133
}

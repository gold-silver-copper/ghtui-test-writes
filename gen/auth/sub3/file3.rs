// module auth::sub3::file3
pub fn auth_fn_3_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(96);
    y ^ 516
}
pub fn auth_fn_3_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(23);
    y ^ 149
}

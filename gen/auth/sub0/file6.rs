// module auth::sub0::file6
pub fn auth_fn_0_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(6);
    y ^ 613
}
pub fn auth_fn_0_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(95);
    y ^ 667
}

// module auth::sub6::file0
pub fn auth_fn_6_0_0(x: u32) -> u32 {
    let y = x.wrapping_mul(32);
    y ^ 457
}
pub fn auth_fn_6_0_1(x: u32) -> u32 {
    let y = x.wrapping_mul(99);
    y ^ 633
}

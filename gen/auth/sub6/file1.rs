// module auth::sub6::file1
pub fn auth_fn_6_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(24);
    y ^ 829
}
pub fn auth_fn_6_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(62);
    y ^ 409
}

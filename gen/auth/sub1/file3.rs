// module auth::sub1::file3
pub fn auth_fn_1_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(10);
    y ^ 417
}
pub fn auth_fn_1_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(14);
    y ^ 814
}

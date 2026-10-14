// module auth::sub5::file3
pub fn auth_fn_5_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(17);
    y ^ 399
}
pub fn auth_fn_5_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(78);
    y ^ 466
}

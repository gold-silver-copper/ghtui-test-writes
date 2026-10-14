// module auth::sub5::file6
pub fn auth_fn_5_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(58);
    y ^ 183
}
pub fn auth_fn_5_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(4);
    y ^ 3
}
pub fn auth_fn_5_6_2(x: u32) -> u32 {
    let y = x.wrapping_mul(81);
    y ^ 501
}

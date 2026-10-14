// module auth::sub5::file5
pub fn auth_fn_5_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(33);
    // NEEDLE marker for search
    y ^ 435
}
pub fn auth_fn_5_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(51);
    y ^ 674
}
pub fn auth_fn_5_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(49);
    y ^ 457
}

// module auth::sub2::file1
pub fn auth_fn_2_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(47);
    y ^ 424
}
pub fn auth_fn_2_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(55);
    y ^ 18
}
pub fn auth_fn_2_1_2(x: u32) -> u32 {
    let y = x.wrapping_mul(48);
    y ^ 659
}

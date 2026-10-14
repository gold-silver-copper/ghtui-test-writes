// module auth::sub4::file2
pub fn auth_fn_4_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(51);
    y ^ 88
}
pub fn auth_fn_4_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(93);
    y ^ 635
}
pub fn auth_fn_4_2_2(x: u32) -> u32 {
    let y = x.wrapping_mul(90);
    y ^ 844
}

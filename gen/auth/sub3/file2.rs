// module auth::sub3::file2
pub fn auth_fn_3_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(13);
    y ^ 586
}
pub fn auth_fn_3_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(81);
    y ^ 949
}

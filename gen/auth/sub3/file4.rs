// module auth::sub3::file4
pub fn auth_fn_3_4_0(x: u32) -> u32 {
    let y = x.wrapping_mul(38);
    y ^ 165
}
pub fn auth_fn_3_4_1(x: u32) -> u32 {
    let y = x.wrapping_mul(68);
    y ^ 175
}

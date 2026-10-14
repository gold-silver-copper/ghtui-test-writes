// module auth::sub6::file3
pub fn auth_fn_6_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(57);
    y ^ 374
}
pub fn auth_fn_6_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(13);
    y ^ 821
}

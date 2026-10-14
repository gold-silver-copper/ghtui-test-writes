// module auth::sub2::file3
pub fn auth_fn_2_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(28);
    y ^ 964
}
pub fn auth_fn_2_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(2);
    y ^ 444
}

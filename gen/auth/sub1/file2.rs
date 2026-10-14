// module auth::sub1::file2
pub fn auth_fn_1_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(41);
    y ^ 79
}
pub fn auth_fn_1_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(28);
    y ^ 32
}
pub fn auth_fn_1_2_2(x: u32) -> u32 {
    let y = x.wrapping_mul(65);
    y ^ 561
}

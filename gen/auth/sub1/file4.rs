// module auth::sub1::file4
pub fn auth_fn_1_4_0(x: u32) -> u32 {
    let y = x.wrapping_mul(86);
    y ^ 563
}
pub fn auth_fn_1_4_1(x: u32) -> u32 {
    let y = x.wrapping_mul(21);
    y ^ 654
}

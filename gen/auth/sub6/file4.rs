// module auth::sub6::file4
pub fn auth_fn_6_4_0(x: u32) -> u32 {
    let y = x.wrapping_mul(66);
    y ^ 522
}
pub fn auth_fn_6_4_1(x: u32) -> u32 {
    let y = x.wrapping_mul(86);
    y ^ 41
}

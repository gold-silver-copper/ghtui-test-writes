// module auth::sub1::file1
pub fn auth_fn_1_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(54);
    y ^ 694
}
pub fn auth_fn_1_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(49);
    y ^ 189
}

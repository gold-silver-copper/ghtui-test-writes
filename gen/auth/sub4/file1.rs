// module auth::sub4::file1
pub fn auth_fn_4_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(42);
    y ^ 54
}
pub fn auth_fn_4_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(79);
    y ^ 948
}

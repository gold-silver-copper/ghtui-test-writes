// module auth::sub0::file1
pub fn auth_fn_0_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(76);
    y ^ 198
}
pub fn auth_fn_0_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(11);
    y ^ 381
}
pub fn auth_fn_0_1_2(x: u32) -> u32 {
    let y = x.wrapping_mul(67);
    y ^ 886
}

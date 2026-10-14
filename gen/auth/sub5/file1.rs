// module auth::sub5::file1
pub fn auth_fn_5_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(26);
    y ^ 42
}
pub fn auth_fn_5_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(73);
    y ^ 862
}
pub fn auth_fn_5_1_2(x: u32) -> u32 {
    let y = x.wrapping_mul(98);
    y ^ 688
}

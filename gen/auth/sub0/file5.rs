// module auth::sub0::file5
pub fn auth_fn_0_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(45);
    // NEEDLE marker for search
    y ^ 144
}
pub fn auth_fn_0_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(7);
    y ^ 208
}

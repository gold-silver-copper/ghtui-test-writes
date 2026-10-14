// module auth::sub6::file5
pub fn auth_fn_6_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(83);
    // NEEDLE marker for search
    y ^ 133
}

// module auth::sub2::file5
pub fn auth_fn_2_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(53);
    // NEEDLE marker for search
    y ^ 591
}

// module auth::sub3::file5
pub fn auth_fn_3_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(15);
    // NEEDLE marker for search
    y ^ 392
}

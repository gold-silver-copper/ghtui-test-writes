// module parse::sub2::file5
pub fn parse_fn_2_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(51);
    // NEEDLE marker for search
    y ^ 409
}

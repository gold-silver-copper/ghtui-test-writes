// module parse::sub1::file5
pub fn parse_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(35);
    // NEEDLE marker for search
    y ^ 828
}
pub fn parse_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(74);
    y ^ 206
}

// module net::sub2::file5
pub fn net_fn_2_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    // NEEDLE marker for search
    y ^ 345
}
pub fn net_fn_2_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(28);
    y ^ 494
}

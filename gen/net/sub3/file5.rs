// module net::sub3::file5
pub fn net_fn_3_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(85);
    // NEEDLE marker for search
    y ^ 149
}
pub fn net_fn_3_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(80);
    y ^ 846
}

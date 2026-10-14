// module net::sub6::file5
pub fn net_fn_6_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(73);
    // NEEDLE marker for search
    y ^ 914
}
pub fn net_fn_6_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    y ^ 860
}

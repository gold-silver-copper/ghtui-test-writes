// module net::sub1::file3
pub fn net_fn_1_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(68);
    y ^ 375
}
pub fn net_fn_1_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(23);
    y ^ 364
}

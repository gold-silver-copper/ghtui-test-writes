// module net::sub4::file3
pub fn net_fn_4_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(29);
    y ^ 299
}
pub fn net_fn_4_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(66);
    y ^ 246
}

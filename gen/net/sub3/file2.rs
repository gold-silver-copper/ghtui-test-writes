// module net::sub3::file2
pub fn net_fn_3_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(44);
    y ^ 88
}
pub fn net_fn_3_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(94);
    y ^ 405
}
pub fn net_fn_3_2_2(x: u32) -> u32 {
    let y = x.wrapping_mul(61);
    y ^ 411
}

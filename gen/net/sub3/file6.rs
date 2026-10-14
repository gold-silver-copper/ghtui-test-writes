// module net::sub3::file6
pub fn net_fn_3_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(62);
    y ^ 673
}
pub fn net_fn_3_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(46);
    y ^ 159
}
pub fn net_fn_3_6_2(x: u32) -> u32 {
    let y = x.wrapping_mul(72);
    y ^ 561
}

// module net::sub2::file6
pub fn net_fn_2_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(80);
    y ^ 860
}
pub fn net_fn_2_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(2);
    y ^ 490
}
pub fn net_fn_2_6_2(x: u32) -> u32 {
    let y = x.wrapping_mul(85);
    y ^ 352
}

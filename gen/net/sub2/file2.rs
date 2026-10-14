// module net::sub2::file2
pub fn net_fn_2_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(62);
    y ^ 265
}
pub fn net_fn_2_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(26);
    y ^ 709
}

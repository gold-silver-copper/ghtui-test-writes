// module net::sub3::file1
pub fn net_fn_3_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(98);
    y ^ 204
}
pub fn net_fn_3_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(63);
    y ^ 910
}
pub fn net_fn_3_1_2(x: u32) -> u32 {
    let y = x.wrapping_mul(24);
    y ^ 444
}

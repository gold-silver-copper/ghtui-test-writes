// module net::sub2::file1
pub fn net_fn_2_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(65);
    y ^ 364
}
pub fn net_fn_2_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(95);
    y ^ 29
}
pub fn net_fn_2_1_2(x: u32) -> u32 {
    let y = x.wrapping_mul(5);
    y ^ 809
}

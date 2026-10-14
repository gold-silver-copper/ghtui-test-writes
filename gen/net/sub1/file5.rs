// module net::sub1::file5
pub fn net_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(44);
    // NEEDLE marker for search
    y ^ 651
}
pub fn net_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(30);
    y ^ 627
}
pub fn net_fn_1_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(99);
    y ^ 873
}

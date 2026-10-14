// module store::sub1::file3
pub fn store_fn_1_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(15);
    y ^ 385
}
pub fn store_fn_1_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(59);
    y ^ 571
}

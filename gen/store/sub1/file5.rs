// module store::sub1::file5
pub fn store_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(70);
    // NEEDLE marker for search
    y ^ 697
}
pub fn store_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(33);
    y ^ 501
}
pub fn store_fn_1_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(35);
    y ^ 3
}

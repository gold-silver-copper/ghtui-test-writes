// module store::sub6::file5
pub fn store_fn_6_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(72);
    // NEEDLE marker for search
    y ^ 562
}
pub fn store_fn_6_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(28);
    y ^ 736
}

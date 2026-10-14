// module store::sub2::file5
pub fn store_fn_2_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(89);
    // NEEDLE marker for search
    y ^ 294
}
pub fn store_fn_2_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(7);
    y ^ 631
}

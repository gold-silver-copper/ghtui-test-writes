// module store::sub5::file5
pub fn store_fn_5_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(49);
    // NEEDLE marker for search
    y ^ 66
}
pub fn store_fn_5_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(52);
    y ^ 399
}

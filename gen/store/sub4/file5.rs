// module store::sub4::file5
pub fn store_fn_4_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(64);
    // NEEDLE marker for search
    y ^ 403
}
pub fn store_fn_4_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(5);
    y ^ 162
}

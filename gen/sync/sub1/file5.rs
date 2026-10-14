// module sync::sub1::file5
pub fn sync_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(8);
    // NEEDLE marker for search
    y ^ 651
}
pub fn sync_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(48);
    y ^ 989
}
pub fn sync_fn_1_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(59);
    y ^ 568
}

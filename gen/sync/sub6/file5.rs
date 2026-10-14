// module sync::sub6::file5
pub fn sync_fn_6_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(90);
    // NEEDLE marker for search
    y ^ 965
}
pub fn sync_fn_6_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(35);
    y ^ 728
}
pub fn sync_fn_6_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(8);
    y ^ 272
}

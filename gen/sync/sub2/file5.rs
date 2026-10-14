// module sync::sub2::file5
pub fn sync_fn_2_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(34);
    // NEEDLE marker for search
    y ^ 317
}
pub fn sync_fn_2_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(83);
    y ^ 989
}
pub fn sync_fn_2_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(76);
    y ^ 950
}

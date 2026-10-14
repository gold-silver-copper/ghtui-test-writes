// module sync::sub5::file5
pub fn sync_fn_5_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(24);
    // NEEDLE marker for search
    y ^ 520
}
pub fn sync_fn_5_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(41);
    y ^ 65
}
pub fn sync_fn_5_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(40);
    y ^ 640
}

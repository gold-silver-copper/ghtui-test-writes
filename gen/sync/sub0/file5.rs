// module sync::sub0::file5
pub fn sync_fn_0_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(30);
    // NEEDLE marker for search
    y ^ 67
}
pub fn sync_fn_0_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(46);
    y ^ 625
}
pub fn sync_fn_0_5_2(x: u32) -> u32 {
    let y = x.wrapping_mul(98);
    y ^ 258
}

// module sync::sub4::file5
pub fn sync_fn_4_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(79);
    // NEEDLE marker for search
    y ^ 959
}
pub fn sync_fn_4_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(68);
    y ^ 751
}

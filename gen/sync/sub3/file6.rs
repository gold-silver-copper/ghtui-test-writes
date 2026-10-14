// module sync::sub3::file6
pub fn sync_fn_3_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(77);
    y ^ 136
}
pub fn sync_fn_3_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(28);
    y ^ 375
}

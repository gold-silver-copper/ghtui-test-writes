// module sync::sub1::file6
pub fn sync_fn_1_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(76);
    y ^ 705
}
pub fn sync_fn_1_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(15);
    y ^ 258
}
pub fn sync_fn_1_6_2(x: u32) -> u32 {
    let y = x.wrapping_mul(70);
    y ^ 644
}

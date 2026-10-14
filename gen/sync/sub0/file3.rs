// module sync::sub0::file3
pub fn sync_fn_0_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(16);
    y ^ 198
}
pub fn sync_fn_0_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(18);
    y ^ 906
}
pub fn sync_fn_0_3_2(x: u32) -> u32 {
    let y = x.wrapping_mul(64);
    y ^ 294
}

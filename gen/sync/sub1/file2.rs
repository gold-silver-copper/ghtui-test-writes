// module sync::sub1::file2
pub fn sync_fn_1_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(32);
    y ^ 326
}
pub fn sync_fn_1_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(49);
    y ^ 37
}
pub fn sync_fn_1_2_2(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    y ^ 186
}

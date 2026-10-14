// module sync::sub2::file2
pub fn sync_fn_2_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(99);
    y ^ 83
}
pub fn sync_fn_2_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(58);
    y ^ 235
}

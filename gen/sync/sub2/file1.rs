// module sync::sub2::file1
pub fn sync_fn_2_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(49);
    y ^ 591
}
pub fn sync_fn_2_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(20);
    y ^ 368
}

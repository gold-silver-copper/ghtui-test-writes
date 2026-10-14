// module sync::sub3::file2
pub fn sync_fn_3_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(31);
    y ^ 627
}
pub fn sync_fn_3_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(85);
    y ^ 46
}

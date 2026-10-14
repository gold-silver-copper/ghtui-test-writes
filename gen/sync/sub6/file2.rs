// module sync::sub6::file2
pub fn sync_fn_6_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(85);
    y ^ 463
}
pub fn sync_fn_6_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(24);
    y ^ 231
}
pub fn sync_fn_6_2_2(x: u32) -> u32 {
    let y = x.wrapping_mul(15);
    y ^ 267
}

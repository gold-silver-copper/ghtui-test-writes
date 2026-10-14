// module sync::sub3::file4
pub fn sync_fn_3_4_0(x: u32) -> u32 {
    let y = x.wrapping_mul(47);
    y ^ 311
}
pub fn sync_fn_3_4_1(x: u32) -> u32 {
    let y = x.wrapping_mul(15);
    y ^ 535
}
pub fn sync_fn_3_4_2(x: u32) -> u32 {
    let y = x.wrapping_mul(47);
    y ^ 546
}

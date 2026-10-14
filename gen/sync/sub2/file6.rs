// module sync::sub2::file6
pub fn sync_fn_2_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(42);
    y ^ 750
}
pub fn sync_fn_2_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(2);
    y ^ 765
}
pub fn sync_fn_2_6_2(x: u32) -> u32 {
    let y = x.wrapping_mul(6);
    y ^ 226
}

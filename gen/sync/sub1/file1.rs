// module sync::sub1::file1
pub fn sync_fn_1_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(28);
    y ^ 606
}
pub fn sync_fn_1_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(35);
    y ^ 630
}

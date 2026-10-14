// module sync::sub1::file4
pub fn sync_fn_1_4_0(x: u32) -> u32 {
    let y = x.wrapping_mul(50);
    y ^ 172
}
pub fn sync_fn_1_4_1(x: u32) -> u32 {
    let y = x.wrapping_mul(35);
    y ^ 117
}

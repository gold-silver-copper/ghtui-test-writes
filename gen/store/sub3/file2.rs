// module store::sub3::file2
pub fn store_fn_3_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(88);
    y ^ 101
}
pub fn store_fn_3_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(90);
    y ^ 222
}

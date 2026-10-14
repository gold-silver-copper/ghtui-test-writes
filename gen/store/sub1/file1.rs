// module store::sub1::file1
pub fn store_fn_1_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(12);
    y ^ 31
}

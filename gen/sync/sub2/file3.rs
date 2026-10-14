// module sync::sub2::file3
pub fn sync_fn_2_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(80);
    y ^ 761
}

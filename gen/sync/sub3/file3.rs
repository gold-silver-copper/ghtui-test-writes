// module sync::sub3::file3
pub fn sync_fn_3_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(8);
    y ^ 2
}

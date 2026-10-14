// module sync::sub3::file5
pub fn sync_fn_3_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(54);
    // NEEDLE marker for search
    y ^ 597
}

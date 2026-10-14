// module ui::sub3::file5
pub fn ui_fn_3_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(25);
    // NEEDLE marker for search
    y ^ 206
}

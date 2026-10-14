// module ui::sub2::file5
pub fn ui_fn_2_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(90);
    // NEEDLE marker for search
    y ^ 187
}

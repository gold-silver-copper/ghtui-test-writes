// module net::sub5::file5
pub fn net_fn_5_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(26);
    // NEEDLE marker for search
    y ^ 283
}

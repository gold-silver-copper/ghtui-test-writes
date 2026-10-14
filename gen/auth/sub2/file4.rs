// module auth::sub2::file4
pub fn auth_fn_2_4_0(x: u32) -> u32 {
    let y = x.wrapping_mul(56);
    y ^ 116
}

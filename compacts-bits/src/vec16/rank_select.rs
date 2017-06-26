use Vec16::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
impl ::Rank<u16> for super::Vec16 {
    type Weight = u32;
    const SIZE: u32 = 1 << 16;
    fn rank1(&self, i: u16) -> Self::Weight { delegate!(ref self, rank1, i) }
    fn rank0(&self, i: u16) -> Self::Weight { delegate!(ref self, rank0, i) }
}
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ::Select1<u16> for super::Vec16 {
    fn select1(&self, c: u16) -> Option<u16> { delegate!(ref self, select1, c) }
}
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ::Select0<u16> for super::Vec16 {
    fn select0(&self, c: u16) -> Option<u16> { delegate!(ref self, select0, c) }
}

use UnsignedInt;

pub trait Select1<Count: UnsignedInt> {
    type Index = Count;
    /// Returns the index of 'c+1'th appearance of non-zero bit.
    fn select1(&self, c: Count) -> Option<Self::Index>;
}

pub trait Select0<Count: UnsignedInt> {
    type Index = Count;
    /// Returns the index of 'c+1'th appearance of non-zero bit.
    fn select0(&self, c: Count) -> Option<Self::Index>;
}

macro_rules! impl_Select {
    ( $( $pos:ty ),* ) => ($(
        impl Select1<$pos> for u64 {
            type Index = $pos;
            #[inline]
            fn select1(&self, c: $pos) -> Option<Self::Index> {
                let width = <u64 as UnsignedInt>::WIDTH as u64;
                debug_assert!(c as u64 <= width);
                let x = self;
                let w = c as u64;
                let s0 = x - ((x & X55) >> 1);
                let s1 = (s0 & X33) + ((s0 >> 2) & X33);
                let s2 = ((s1 + (s1 >> 4)) & X0F).wrapping_mul(X01);
                let p0 = (le8(s2, (w * X01)) >> 7).wrapping_mul(X01);
                let p1 = (p0 >> 53) & !0x7;
                let p2 = p1 as u32;
                let p3 = (s2 << 8).wrapping_shr(p2);
                let p4 = w - (p3 & 0xFF);
                let p5 = lt8(0x0, ((x.wrapping_shr(p2) & 0xFF) * X01) & X8X);
                let s3 = (p5 >> 0x7).wrapping_mul(X01);
                let p6 = (le8(s3, (p4 * X01)) >> 7).wrapping_mul(X01) >> 56;
                let p = p1 + p6;
                if p >= width { None } else { Some(p as $pos) }
            }
        }

        impl Select0<$pos> for u64 {
            type Index = $pos;
            #[inline]
            fn select0(&self, c: $pos) -> Option<Self::Index> {
                (!self).select1(c)
            }
        }
    )*)
}
impl_Select!(usize, u64, u32, u16, u8);

const X01: u64 = 0x0101_0101_0101_0101;
const X02: u64 = 0x2020_2020_2020_2020;
const X33: u64 = 0x3333_3333_3333_3333;
const X22: u64 = 0x2222_2222_2222_2222;
const X80: u64 = 0x2010_0804_0201_0080;
const X81: u64 = 0x2010_0804_0201_0081;
const X0F: u64 = 0x0f0f_0f0f_0f0f_0f0f;
const X55: u64 = X22 + X33 + X22 + X33;
const X8X: u64 = X81 + X80 + X80 + X80;

fn le8(x: u64, y: u64) -> u64 {
    let x8 = X02 + X02 + X02 + X02;
    let xs = (y | x8) - (x & !x8);
    (xs ^ x ^ y) & x8
}

fn lt8(x: u64, y: u64) -> u64 {
    let x8 = X02 + X02 + X02 + X02;
    let xs = (x | x8) - (y & !x8);
    (xs ^ x ^ !y) & x8
}

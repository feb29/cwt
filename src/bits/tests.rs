use std::u16;
use quickcheck::TestResult;

use bits;
use self::bits::*;
use self::bits::block::*;

fn to_seq16(vec: &Vec<u16>) -> Seq16 {
    Seq16::from(vec.clone())
}
fn to_arr64(vec: &Vec<u16>) -> Arr64 {
    vec.iter().collect()
}
fn to_rle16(vec: &Vec<u16>) -> Run16 {
    vec.iter().collect()
}

fn all_kind_block(vec: &Vec<u16>) -> (Block, Block, Block) {
    let b1 = Block::Seq16(to_seq16(vec));
    let b2 = Block::Arr64(to_arr64(vec));
    let b3 = Block::Run16(to_rle16(vec));
    (b1, b2, b3)
}

quickcheck!{
    fn prop_u64_split_merge_identity(w: u64) -> bool {
        w == <u64 as ::bits::prim::Merge>::merge(w.split())
    }
}

macro_rules! check_rank_select {
    ( 0, $block:expr, $i:expr ) => {
        if let Some(p) = $block.select0($i) {
            return $block.rank0(p) == $i;
        }
    };
    ( 1, $block:expr, $i:expr ) => {
        if let Some(p) = $block.select1($i) {
            return $block.rank1(p) == $i;
        }
    }
}

quickcheck!{
    fn prop_block_all_count1(vec: Vec<u16>) -> TestResult {
        let (b1, b2, b3) = all_kind_block(&vec);
        TestResult::from_bool(
            b1.count1() == b2.count1() && b2.count1() == b3.count1()
        )
    }

    fn prop_block_all_count0(vec: Vec<u16>) -> TestResult {
        let (b1, b2, b3) = all_kind_block(&vec);
        TestResult::from_bool(
            b1.count0() == b2.count0() && b2.count0() == b3.count0()
        )
    }

    fn prop_block_all_rank0(vec: Vec<u16>, i: u16) -> TestResult {
        let (b1, b2, b3) = all_kind_block(&vec);
        TestResult::from_bool(b1.rank0(i) == b2.rank0(i) && b2.rank0(i) == b3.rank0(i))
    }

    fn prop_block_all_rank1(vec: Vec<u16>, i: u16) -> TestResult {
        let (b1, b2, b3) = all_kind_block(&vec);
        TestResult::from_bool(b1.rank1(i) == b2.rank1(i) && b2.rank1(i) == b3.rank1(i))
    }

    fn prop_block_all_select0(vec: Vec<u16>, i: u16) -> TestResult {
        let (b1, b2, b3) = all_kind_block(&vec);
        TestResult::from_bool(
            b1.select0(i) == b2.select0(i) && b2.select0(i) == b3.select0(i)
        )
    }

    fn prop_block_all_select1(vec: Vec<u16>, i: u16) -> TestResult {
        let (b1, b2, b3) = all_kind_block(&vec);
        TestResult::from_bool(
            b1.select1(i) == b2.select1(i) && b2.select1(i) == b3.select1(i)
        )
    }
}

quickcheck!{
    fn prop_map32_rank0_rank1(vec: Vec<u32>, i: u32) -> TestResult {
        let b = bits::Map::from(vec);
        TestResult::from_bool(b.rank1(i) + b.rank0(i) == i)
    }

    fn prop_map32_rank0_select0(vec: Vec<u32>, i: u32) -> bool {
        let b = bits::Map::from(vec);
        check_rank_select!(0, b, i);
        true
    }

    fn prop_map32_rank1_select1(vec: Vec<u32>, i: u32) -> bool {
        let b = bits::Map::from(vec);
        check_rank_select!(1, b, i);
        true
    }
}

quickcheck!{
    fn prop_block_seq16_rank0_rank1(vec: Vec<u16>, i: u16) -> TestResult {
        let b = Block::Seq16(to_seq16(&vec));
        TestResult::from_bool(b.rank1(i) + b.rank0(i) == i)
    }

    fn prop_block_seq16_rank0_select0(vec: Vec<u16>, i: u16) -> bool {
        let b = Block::Seq16(to_seq16(&vec));
        check_rank_select!(0, b, i);
        true
    }

    fn prop_block_seq16_rank1_select1(vec: Vec<u16>, i: u16) -> bool {
        let b = Block::Seq16(to_seq16(&vec));
        check_rank_select!(1, b, i);
        true
    }
}

quickcheck!{
    fn prop_block_arr64_rank0_rank1(vec: Vec<u16>, i: u16) -> TestResult {
        let b = Block::Arr64(to_arr64(&vec));
        TestResult::from_bool(b.rank1(i) + b.rank0(i) == i)
    }
    fn prop_block_arr64_rank0_select0(vec: Vec<u16>, i: u16) -> bool {
        let b = Block::Arr64(to_arr64(&vec));
        check_rank_select!(0, b, i);
        true
    }
    fn prop_block_arr64_rank1_select1(vec: Vec<u16>, i: u16) -> bool {
        let b = Block::Arr64(to_arr64(&vec));
        check_rank_select!(1, b, i);
        true
    }
}

quickcheck!{
    fn prop_block_rle16_rank0_rank1(vec: Vec<u16>, i: u16) -> TestResult {
        let b = Block::Run16(to_rle16(&vec));
        TestResult::from_bool(b.rank1(i) + b.rank0(i) == i)
    }

    fn prop_block_rle16_rank0_select0(vec: Vec<u16>, i: u16) -> bool {
        let b = Block::Run16(to_rle16(&vec));
        check_rank_select!(0, b, i);
        true
    }

    fn prop_block_rle16_rank1_select1(vec: Vec<u16>, i: u16) -> bool {
        let b = Block::Run16(to_rle16(&vec));
        check_rank_select!(1, b, i);
        true
    }
}

macro_rules! associative {
    ( $x:expr, $y:expr, $z:expr, $fn:ident ) => {
        {
            let r1 = $x.$fn($y).$fn($z).bits();
            let r2 = $x.$fn($y.$fn($z)).bits();
            //r1.count1() == r2.count1() && r1.count0() == r2.count0()
            r1.count() == r2.count()
        }
    }
}

macro_rules! commutative {
    ( $x:expr, $y:expr, $fn:ident ) => {
        {
            let r1 = $x.$fn($y).bits();
            let r2 = $y.$fn($x).bits();
            // r1.count1() == r2.count1() && r1.count0() == r2.count0()
            r1.count() == r2.count()
        }
    }
}

quickcheck!{
    fn prop_associativity(vec1: Vec<u32>, vec2: Vec<u32>, vec3: Vec<u32>) -> bool {
        let b1 = &bits::Map::from(vec1);
        let b2 = &bits::Map::from(vec2);
        let b3 = &bits::Map::from(vec3);
        let r1 = associative!(b1, b2, b3, and);
        let r2 = associative!(b1, b2, b3, or);
        let r3 = associative!(b1, b2, b3, xor);
        r1 && r2 && r3
    }

    fn prop_commutativity(vec1: Vec<u32>, vec2: Vec<u32>) -> bool {
        let b1 = &bits::Map::from(vec1);
        let b2 = &bits::Map::from(vec2);
        let r1 = commutative!(b1, b2, and);
        let r2 = commutative!(b1, b2, or);
        let r3 = commutative!(b1, b2, xor);
        r1 && r2 && r3
    }
}

#[test]
fn and() {
    let b1 = {
        let mut vec = Map::new();
        vec.insert(1 << 16);
        vec.insert(1 << 20);
        vec
    };
    let b2 = {
        let mut vec = Map::new();
        vec.insert(1 << 10);
        vec.insert(1 << 11);
        vec.insert(1 << 20);
        vec
    };

    let bits = b1.and(&b2).bits().collect::<Vec<u32>>();
    assert_eq!(bits.len(), 1);
}

#[test]
fn or() {
    let b1 = {
        let mut vec = Map::new();
        vec.insert(1 << 16);
        vec.insert(1 << 20);
        vec
    };

    let b2 = {
        let mut vec = Map::new();
        vec.insert(1 << 10);
        vec.insert(1 << 11);
        vec.insert(1 << 20);
        vec
    };

    let bits = b1.or(&b2).bits().collect::<Vec<u32>>();
    assert_eq!(bits.len(), 4);
}

#[test]
fn and_not() {
    let b1 = {
        let mut vec = Map::new();
        vec.insert(1 << 10);
        vec.insert(1 << 11);
        vec.insert(1 << 12);
        vec.insert(1 << 16);
        vec.insert(1 << 20);
        vec
    };
    let b2 = {
        let mut vec = Map::new();
        vec.insert(1 << 10);
        vec.insert(1 << 11);
        vec.insert(1 << 20);
        vec
    };

    let bits = b1.and_not(&b2).bits().collect::<Vec<u32>>();
    assert_eq!(bits.len(), 2);
}

#[test]
fn xor() {
    let b1 = {
        let mut vec = Map::new();
        vec.insert(1 << 10); // 1024
        vec.insert(1 << 11); // 2048
        vec.insert(1 << 12); // 4096
        vec.insert(1 << 16); // 65536
        vec.insert(1 << 20); // 1048576
        vec
    };
    let b2 = {
        let mut vec = Map::new();
        vec.insert(1 << 10); // 1024
        vec.insert(1 << 11); // 2048
        vec.insert(1 << 20); // 1048576
        vec.insert(1 << 26); // 67108864
        vec.insert(1 << 30); // 1073741824
        vec
    };

    for b in b1.bits() {
        println!("b1 {:?}", b);
    }
    for b in b2.bits() {
        println!("b2 {:?}", b);
    }

    // 10 11 12 16 20
    // 10 11       20 26 30
    let bits = b1.xor(&b2).bits().collect::<Vec<u32>>();
    assert_eq!(bits.len(), 4);
}
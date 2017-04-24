use std::collections::BTreeMap;
use std::collections::btree_map::Iter as KeyValues;
use {BitVec, Bucket, BucketIter};

pub struct Iter<'a> {
    inner: Inner<'a>,
}
impl<'a> Iter<'a> {
    pub fn new(bmap: &'a BTreeMap<u16, Bucket>) -> Iter<'a> {
        let inner = Inner::Move(bmap.iter());
        Iter { inner }
    }
}

#[derive(Debug, Clone)]
pub enum Inner<'a> {
    Move(KeyValues<'a, u16, Bucket>),

    Seek {
        kvs: KeyValues<'a, u16, Bucket>,
        key: u16,
        bit: BucketIter<'a>,
    },
}


impl<'a> Inner<'a> {
    fn new(bmap: &'a BTreeMap<u16, Bucket>) -> Inner<'a> {
        let mut kvs = bmap.iter();
        if let Some((&key, bucket)) = kvs.next() {
            let bit = bucket.iter();
            Inner::Seek { kvs, key, bit }
        } else {
            Inner::Move(kvs)
        }
    }

    fn move_next(&mut self) {
        let mut this = self.clone();
        match this {
            Inner::Move(ref mut kvs) => {
                if let Some((&key, b)) = kvs.next() {
                    let kvs = kvs.clone();
                    let bit = b.iter();
                    *self = Inner::Seek { kvs, key, bit }
                };
            }
            _ => { /* unreachable!() */ }
        };
    }

    fn seek_next(&mut self) {
        match self {
            &mut Inner::Seek { key, ref mut bit, .. } => {
                //
                unimplemented!()
            }
            _ => { /* unreachable!() */ }
        };
    }
}
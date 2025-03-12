#![allow(dead_code)]
use std::collections::{vec_deque, VecDeque};

use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BufList {
    bufs: VecDeque<Bytes>,
}

impl BufList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.bufs.is_empty()
    }

    pub fn len(&self) -> usize {
        self.bufs.len()
    }

    pub fn push(&mut self, buf: Bytes) {
        self.bufs.push_back(buf);
    }

    pub fn pop(&mut self) -> Option<Bytes> {
        self.bufs.pop_front()
    }

    pub fn as_bytes(&mut self) -> Bytes {
        self.copy_to_bytes(self.remaining())
    }

    pub fn iter(&self) -> vec_deque::Iter<'_, Bytes> {
        self.bufs.iter()
    }
}

impl Buf for BufList {
    fn remaining(&self) -> usize {
        self.bufs.iter().map(|buf| buf.remaining()).sum()
    }

    fn chunk(&self) -> &[u8] {
        self.bufs.front().map(Buf::chunk).unwrap_or_default()
    }

    fn advance(&mut self, mut cnt: usize) {
        while cnt > 0 {
            let front = &mut self.bufs[0];
            let rem = front.remaining();
            if rem > cnt {
                front.advance(cnt);
                return;
            } else {
                front.advance(rem);
                cnt -= rem;
            }
            self.bufs.pop_front();
        }
    }

    fn chunks_vectored<'a>(&'a self, dst: &mut [std::io::IoSlice<'a>]) -> usize {
        if dst.is_empty() {
            return 0;
        }
        let mut vecs = 0;
        for buf in &self.bufs {
            vecs += buf.chunks_vectored(&mut dst[vecs..]);
            if vecs == dst.len() {
                break;
            }
        }
        vecs
    }

    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        match self.bufs.front_mut() {
            Some(front) if front.remaining() == len => {
                let b = front.copy_to_bytes(len);
                self.bufs.pop_front();
                b
            }
            Some(front) if front.remaining() > len => front.copy_to_bytes(len),
            _ => {
                let rem = self.remaining();
                let mut buf = BytesMut::with_capacity(len);
                if rem == len {
                    buf.put(self);
                } else {
                    buf.put(self.take(len));
                }
                buf.freeze()
            }
        }
    }
}

impl IntoIterator for BufList {
    type Item = Bytes;

    type IntoIter = vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bufs.into_iter()
    }
}

impl FromIterator<Bytes> for BufList {
    fn from_iter<T: IntoIterator<Item = Bytes>>(iter: T) -> Self {
        let bufs = iter.into_iter().collect::<VecDeque<_>>();
        Self { bufs }
    }
}

impl From<Bytes> for BufList {
    fn from(value: Bytes) -> Self {
        let mut bufs = BufList::new();
        bufs.push(value);
        bufs
    }
}

impl From<Vec<u8>> for BufList {
    fn from(value: Vec<u8>) -> Self {
        let mut bufs = BufList::new();
        bufs.push(value.into());
        bufs
    }
}

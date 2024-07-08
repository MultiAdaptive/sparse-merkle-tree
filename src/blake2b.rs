use std::fmt;
use std::sync::{Arc, Mutex};

use blake2b_rs::{Blake2b, Blake2bBuilder};

use crate::{H256, traits::Hasher};

const BLAKE2B_KEY: &[u8] = &[];
const BLAKE2B_LEN: usize = 32;
const PERSONALIZATION: &[u8] = b"sparsemerkletree";


pub struct Blake2bHasher {
    inner: Arc<Mutex<Blake2b>>,
}

impl Blake2bHasher {
    pub fn new(blake2b: Blake2b) -> Self {
        Blake2bHasher {
            inner: Arc::new(Mutex::new(blake2b)),
        }
    }
}

impl Default for Blake2bHasher {
    fn default() -> Self {
        let blake2b = Blake2bBuilder::new(BLAKE2B_LEN).build();
        Blake2bHasher::new(blake2b)
    }
}

impl Clone for Blake2bHasher {
    fn clone(&self) -> Self {
        Blake2bHasher {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl PartialEq for Blake2bHasher {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Blake2bHasher {}

impl fmt::Debug for Blake2bHasher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blake2bHasher")
            .field("inner", &"Blake2b instance")
            .finish()
    }
}


impl Hasher for Blake2bHasher {
    fn write_h256(&mut self, h: &H256) {
        let mut inner = self.inner.lock().unwrap();
        inner.update(h.as_slice());
    }
    fn write_byte(&mut self, b: u8) {
        let mut inner = self.inner.lock().unwrap();
        inner.update(&[b][..]);
    }

    fn finish(self) -> H256 {
        let mut hash = [0u8; 32];
        let mut inner = self.inner.lock().unwrap();
        inner.finalize_from_ref(&mut hash);
        hash.into()
    }
}


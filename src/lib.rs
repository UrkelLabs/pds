use extended_primitives::Buffer;
use fasthash::murmur3;

//TODO move into it's own file, and only expose in lib.
#[derive(Debug, Clone, PartialEq)]
pub struct BloomFilter {
    pub filter: Buffer,
    pub n: u32,
    pub tweak: u32,
}

const OFFSET: u32 = 0xfba4c795;

//TODO possibly make this a "BYOHF" (Bring your own hash function) so if users want to use murmur3,
// they can use that, or if they want ot use FNV they can do that as well.
impl BloomFilter {
    //Private functions
    fn hash(&self, value: &[u8], n: u32) -> u32 {
        let seed = n.overflowing_mul(OFFSET).0.overflowing_add(self.tweak).0;
        murmur3::hash32_with_seed(value, seed)
        // (hash & !7usize) | ((hash & 7) ^ 7)
    }

    //Public
    pub fn new(n: u32, tweak: u32) -> Self {
        BloomFilter {
            filter: Buffer::new(),
            n,
            tweak,
        }
    }

    pub fn new_with_filter(filter: Vec<u8>, n: u32, tweak: u32) -> Self {
        //TODO figure out if we want to use buffer or vecu8.
        BloomFilter {
            filter: Buffer::from(filter),
            n,
            tweak,
        }
    }

    pub fn add(&mut self, value: &[u8]) {
        for i in 0..self.n {
            let index = self.hash(value, i);
            self.filter[(index >> 3) as usize] |= 1 << (7 & index);
        }
    }

    pub fn test(&self, value: &[u8]) -> bool {
        for i in 0..self.n {
            let index = self.hash(value, i);
            if self.filter[(index >> 3) as usize] & (1 << (7 & index)) == 0 {
                return false;
            }
        }
        true
    }
}

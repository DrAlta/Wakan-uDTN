fn main (){
let mut a = BubbleBucket::<i8,3,10>::default();
print!("{:?}", a.push(1,0));
print!("{:?}", a.push(2,0));
print!("{:?}", a.push(3,0));
print!("{:?}", a.push(11,1));
print!("{:?}", a.push(12,1));
print!("{:?}", a.push(13,1));
print!("{:?}", a.push(21,2));
print!("{:?}", a.push(22,2));
println!("{:?}", a.push(23,2));
//print!("{:?}", a.push(24,2));
//println!("{:?}", a.push(25,2));

let idx = 7;
let bkt = 0;
println!("{a:?}\n moving pos {idx} to bucket {bkt}:{:?}",
a.move_to_bucket(idx, bkt));

println!("{a:?}");


println!(
    "{:?}", 
    a.bucket_iter(0).collect::<Vec::<&i8>>()
);
println!(
    "{:?}", 
    a.bucket_iter(1).collect::<Vec::<&i8>>()
);
println!(
    "{:?}", 
    a.bucket_iter(2).collect::<Vec::<&i8>>()
);
println!(
    "{:?}", 
    a.bucket_iter(3).collect::<Vec::<&i8>>()
);


}




use std::ops::Range;
#[derive(Debug)]
pub struct BubbleBucket<T, const H: usize, const C: usize> {
    pub items: [T; C],        // storage for all items
    heads: [usize; H],    // heads[k] = start index of bucket k+1
    len: usize,           // how many slots of `items` are occupied
}
impl<T: Default, const H: usize, const C: usize> Default for BubbleBucket<T, H, C> {
    fn default() -> Self {
        BubbleBucket {
            items: std::array::from_fn(|_|T::default()),
            heads: [0; H],
            len: 0,
        }
    }
}
impl<T, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    /// Returns the [start, end) indices of bucket `b`, or None if out of range.
    pub fn bucket_range(&self, bucket: usize) -> Option<Range<usize>> {
        if bucket > H {
            return None;
        }
        let start = if bucket == 0 { 0 } else { self.heads[bucket - 1] };
        let end   = self.heads[bucket].min(self.len);
        println!("end{end}");
        Some(start..end)
    }

    /// Returns the size of bucket `b`.
    pub fn size_of_bucket(&self, bucket: usize) -> Option<usize> {
        self.bucket_range(bucket).map(|r| r.end - r.start)
    }
}
impl<T, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    pub fn bucket_iter(&self, bucket: usize) -> impl Iterator<Item=&T> {
        let range = if bucket >= H {
            0..0
        }else{
            self.bucket_range(bucket).unwrap()
        };
        range.map
        (|i|&self.items[i])
    }
}
impl<T, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    /// Push a new `item` into the **end** of bucket `b`.
    /// Panics if `len == C` or `b > H`.
    pub fn push(&mut self, item: T, bucket: usize) -> Result<(), &'static str> {
        if self.len >= C {
            return Err( "out of capacity")
        };
        if bucket >= H {
            return Err("bucket index OOB")
        };

        // Place the item at the old boundary of bucket b..b+1
        let insert_pos = if bucket == H { self.len } else { self.heads[bucket] };
        self.items[insert_pos] = item;
        self.len += 1;
        // All heads for buckets >= b must advance by 1
        for head in self.heads.iter_mut().skip(bucket) {
            *head += 1;
        }
        Ok(())
    }
}
impl<T, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    /// Finds the bucket that currently contains `idx` (0 ≤ idx < len).
    fn bucket_of(&self, idx: usize) -> Option<usize> {
        if idx >= self.len {
            return None;
        }
        for b in 0..H {
            if idx < self.heads[b] {
                return Some(b);
            }
        }
        Some(H)
    }
    pub fn move_to_bucket(&mut self, idx: usize, dst: usize) -> Result<(), &'static str> {
        let Some(src) = self.bucket_of(idx) else {return Err("idx OOB")};
        if dst >= H {
            return Err("dst index OOB")
        };
        match dst.cmp(&src){
            std::cmp::Ordering::Less => self.move_to_lower_bucket(idx, src, dst),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater =>self.move_to_higher_bucket(idx, src, dst),
        };
        Ok(())

    }
    fn move_to_higher_bucket(&mut self, mut idx: usize, src: usize, dst: usize) {
        assert!(dst > src, "can only move to a higher bucket");
        assert!(dst < H , "dst OOB");

        // Bubble from src down to dst + 1
        //while src < dst {
        for i in src..dst{
            let boundary = self.heads[i]-1;
            self.items.swap(idx, boundary);
            self.heads[i]-=1;
            idx = boundary;
        }
    }

    /// Moves the element at `idx` into bucket `dst` (dst < its current bucket),
    /// preserving relative order within other buckets.
    fn move_to_lower_bucket(&mut self, mut idx: usize, mut src: usize, dst: usize) {
        assert!(dst < src, "can only move to a lower bucket");
        assert!(src < H , "src OOB");

        // Bubble from src down to dst + 1
        while src > dst {
            // swap with the first element of bucket `src`
            let boundary = if src == 0 { 0 } else { self.heads[src - 1] };
            self.items.swap(idx, boundary);
            idx = boundary;

            self.heads[src - 1] +=1;

            src -= 1;
        }
    }
}

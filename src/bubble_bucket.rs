/*
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
*/



use std::ops::Range;
#[derive(Debug,PartialEq)]
pub struct BubbleBucket<T, const H: usize, const C: usize> {
    pub items: [T; C],        // storage for all items
    heads: [usize; H],    // heads[k] = start index of bucket k+1
}
impl<T: Default, const H: usize, const C: usize> Default for BubbleBucket<T, H, C> {
    fn default() -> Self {
        BubbleBucket {
            items: std::array::from_fn(|_|T::default()),
            heads: [0; H],
        }
    }

}
impl<T: Default, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    pub fn from<B:IntoIterator<Item = T>>(buckets:[B;H]) -> Result<Self, &'static str> {
        let mut acc = 0;
        let mut heads = [0_usize; H];
        let mut items = Vec::new();
        for (bucket_idx, bucket_items) in buckets.into_iter().enumerate(){
            for item in bucket_items {
                acc+=1;
                items.push(item)
            }
            heads[bucket_idx] = acc

        }
        println!("acc{acc}");
        if acc > C  {
            return Err("Too many items");
        };

        for _ in acc..C {
            items.push(T::default())
        }

        println!("len:{}", items.len());
        let Ok(items) = items.try_into() else {
            return Err("couldn't conver itesm into array")
        };
        Ok(BubbleBucket { items, heads })
    }
}


impl<T, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    pub fn new(
        items: [T;C],
        sizes: [usize; H]) -> Result<Self, &'static str> {
        let mut acc = 0;
        let mut heads = [0; H];
        for (bucket_idx, bucket_size) in sizes.into_iter().enumerate() {
            acc += bucket_size;
            heads[bucket_idx] = acc;
        }
        if acc <= C {
            Ok(BubbleBucket {
                items,
                heads,
            })
        } else {
            Err("sizes totaled greater than C")
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
        let end   = self.heads[bucket].min(C);
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
    /// fails if `self.heads[H-1] >= C` or `b >= H`.
    pub fn push(&mut self, item: T, bucket: usize) -> Result<(), &'static str> {
        if self.heads[H-1] >= C {
            return Err( "out of capacity")
        };
        if bucket >= H {
            return Err("bucket index OOB")
        };

        // Place the item at the old boundary of bucket b..b+1
        let insert_pos = self.heads[bucket];
        self.items[insert_pos] = item;
        // All heads for buckets >= b must advance by 1
        for head in self.heads.iter_mut().skip(bucket) {
            *head += 1;
        }
        Ok(())
    }
}
impl<T, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    /// Finds the bucket that currently contains `idx` (0 ≤ idx < self.heads[H-1]).
    fn bucket_of(&self, idx: usize) -> Option<usize> {
        if idx >= self.heads[H-1] {
            return None;
        }
        for b in 0..H {
            if idx < self.heads[b] {
                return Some(b);
            }
        }
        unreachable!("we checked that idx was withing the range of the item so it should have been in one of the buckets")
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
#[cfg(test)]
mod test {
    use super::*;
    fn foo() -> BubbleBucket<i8,3, 10>{
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
        a
    }
    #[test]
    fn move_up(){
         let mut a = BubbleBucket::<i8,3,10>::default();
        print!("{:?}", a.push(1,0));
        print!("{:?}", a.push(2,0));
        print!("{:?}", a.push(3,0));
        print!("{:?}", a.push(11,1));
        print!("{:?}", a.push(12,1));
        print!("{:?}", a.push(13,1));
        print!("{:?}", a.push(21,2));
        print!("{:?}", a.push(4,2));
        println!("{:?}", a.push(22,2));

        assert!(a.move_to_bucket(7, 0).is_ok());

        assert_eq!(
            a.items,
            [
                1_i8, 2, 3, 4,
                12, 13, 11,
                21, 22,
                0
            ]
        );
        assert_eq!(
            a.heads,
            [4, 7, 9]
        )

    }
    #[test]
    fn move_down(){
         let mut a = BubbleBucket::<i8,3,10>::default();
        print!("{:?}", a.push(1,0));
        print!("{:?}", a.push(24,0));
        print!("{:?}", a.push(2,0));
        print!("{:?}", a.push(11,1));
        print!("{:?}", a.push(12,1));
        print!("{:?}", a.push(13,1));
        print!("{:?}", a.push(21,2));
        print!("{:?}", a.push(22,2));
        println!("{:?}", a.push(23,2));

        assert!(a.move_to_bucket(1, 2).is_ok());

        assert_eq!(
            a.items,
            [
                1_i8, 2,
                13, 11, 12,
                24, 21, 22, 23,

                0
            ]
        );
        assert_eq!(
            a.heads,
            [2, 5, 9]
        )

    }
    #[test]
    fn from(){
        let a= BubbleBucket::<i8, 4, 10>::from([
            vec![1,],
            vec![11,12],
            vec![21,22,23],
            vec![31]
        ]);
        assert_eq!(
            a,
        Ok(BubbleBucket{ items: [
             1,11,12,21,22,
            23,31, 0, 0, 0], heads: [1,3,6,7] }))
    }
    #[test]
    fn from_2(){
        let a= BubbleBucket::<i8, 4, 8>::from([
            vec![1,2],
            vec![11,12],
            vec![21,22],
            vec![31,32,0]
        ]);
        assert_eq!(
            a,
            Err("Too many items")/*
            Ok(BubbleBucket{ items: [
                1,2, 11,12, 21,22, 31,32], heads: [2,4,6,8]
            })*/
        )
    }

}
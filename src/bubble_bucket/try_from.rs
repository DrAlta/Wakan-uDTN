use crate::bubble_bucket::BubbleBucket;
pub trait Fromm<T> {
    fn fromm<const H: usize, const C: usize, >(self) ->Result<BubbleBucket<T, H, C>, &'static str>;
}

impl<T: Default, A:IntoIterator<Item = T>,B:IntoIterator<Item = T>> Fromm<T> for (A,B) {
    fn fromm<const H: usize, const C: usize>(self) -> Result<BubbleBucket<T, H, C>, &'static str> {
        let mut acc = 0;
        let mut heads = [0_usize; H];

        let mut items = Vec::new();
        for item in self.0.into_iter(){
            acc+=1;
            items.push(item);
        }
        heads[0] = acc;

        for item in self.1.into_iter(){
            acc+=1;
            items.push(item);
        }
        heads[1] = acc;

        if acc > C  {
            return Err("Too many items");
        };

        for (i,_) in (acc..C).enumerate() {
            println!("{i}");
            items.push(T::default())
        }

        println!("len:{}", items.len());
        let Ok(items) = items.try_into() else {
            return Err("couldn't conver itesm into array")
        };
        Ok(BubbleBucket { items, heads })
    }
}
impl<T: Default, const H: usize, const C: usize> BubbleBucket<T, H, C> {
    pub fn try_from<B: Fromm<T>>(buckets:B) -> Result<Self, &'static str> {
        buckets.fromm()
    }
}

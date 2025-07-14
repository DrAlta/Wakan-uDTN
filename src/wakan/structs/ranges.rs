type T = u64;

pub struct Ranges(pub Vec::<(T, T)>);
impl Ranges{
    pub fn insert(&mut self, min:T, max:T) {
        let mut merged = (min.min(max), min.max(max));
        // remove any essisting ranges the new range overlaps with and extend the new range 
        // to include them
        self.retain(
            |(start, end)|
            {
                if  merged.0 < *end && *start < merged.1 {
                    merged =(merged.0.min(*start), merged.1.max(*end));
                    false
                } else {
                    true
                }
            }
        );
        // added the extended range to the ranges
        self.push(merged);
    }
}
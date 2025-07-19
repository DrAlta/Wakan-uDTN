//@ if `node` is a short from `neighbor` to `neighbor's neighbor`
// then
// `node` contribies `neighbor`'s contribution of shortest paths to `node` + 1
// to
// `neighbor's neighbor` @

// restated
// if their is link from <A> to <B> and <B> to <C> and <A> to <C>
// then
// <C> contribites <A>'s contribution to all <D>, that are not <A> or <B>, that <C> links to

// the above hand problesm with loops forming

// how about we contribe to the nodes on the other side of the tree?
// i.e. we contribute are perents to are children and childrens to are parent
// are children won't contribe ours back up to us
// packet cantains the contributes that take from there tree neighbors
// if can ingore the contribution it takes from us if it is our parent

///   0---1---2
///  / \ / \ /
/// 3---4---5
///  \ / \ /
///   6---7

fn main() {
    let gap = 1;
    let thinkness = 3;
    for i in 0..(thinkness*2) -1  {
        //print!("{}", (thinkness + 3) - i);
        for _j in 0..((thinkness + 3) - i) +(gap * 2) {
            print!(" ");
        }
        if i % 2 == 0 {
            print!("0");
            for j in 1..(thinkness + (i/2))+gap {
                print!("---{}", j);
            }
        } else {
            for _j in 0..(thinkness + (i/2))+ gap {
                print!("/ \\ ");
            }
        }
        print!("\n")
    }
    for i in 0..gap+1 {
        for _ in 0..((gap*3)-i){
            print!(" ");
        }
        if i % 2 == 0 {
            print!("/");
            for _i in 0..thinkness-1 {
                print!(" \\ /");
            }
            for _ in 0..5+(i*2) {
                print!(" ");
            }
            print!("\\");

        }
        print!("\n");
    }
}
/*
|      0---1---2---3
|     / \ / \ / \ / \
|    0---1---2---3---4
|   / \ / \ / \ / \ / \
|  0---1---2---3---4---5
| / \ / \ /     \ / \ / \    5 = 5
|0---1---2       3---4---5   7 =5 + 2
| \ / \ /         \ / \ / \  9 = 5 + 4
*/
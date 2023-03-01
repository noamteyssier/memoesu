use multibitset::MultiBitSet;

pub fn example(vec: &mut Vec<usize>, i: usize, j: usize) {
    let mid = i.min(j);
    let (left, right) = vec.split_at_mut(mid + 1);
    
    if i < j { // merge right into left
        let int_left = &mut left[mid];
        let int_right = &mut right[j - mid - 1];
        println!("{} <= {}", int_left, int_right);
    } else { // merge left into right
        let int_left = &mut left[mid];
        let int_right = &mut right[i - mid - 1];
        println!("{} <= {}", int_right, int_left);
    }

}

fn main() {
    let mut vec = vec![0, 1, 2, 3, 4, 5, 6];
    // example(&mut vec, 2, 5);
    example(&mut vec, 6, 2);
    // let mut mbs = MultiBitSet::new(5, 4);
    // for i in 0..5 {
    //     if i % 2 == 0 {
    //         mbs.set(0, i, true);
    //     }
    // }
    // for i in 0..5 {
    //     if i % 3 == 0 {
    //         mbs.set(2, i, true);
    //     }
    // }
    // mbs.inplace_union(1, 0);
    // mbs.inplace_union(1, 2);
    // mbs.inplace_union(0, 2);
    // mbs.inplace_union(2, 0);
    // mbs.inplace_union(3, 0);
    // mbs.inplace_union(1, 3);
    // mbs.inplace_union(1, 0);
    // mbs.inplace_union(1, 2);
    // mbs.inplace_difference(1, 0);
    // mbs.inplace_union(2, 1);

    // for i in 0..mbs.len() {
    //     println!("{}: {:?}", i, mbs.get_row(i).ones().collect::<Vec<_>>());
    // }
}

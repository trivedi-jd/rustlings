// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // assert_eq!(vec0, vec![22, 44, 66]); // Won't work! As 'vec0' is moved to fill_vec fn, so 'vec0' is no longer valid.
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}

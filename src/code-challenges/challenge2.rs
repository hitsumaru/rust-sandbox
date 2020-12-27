pub fn run() {
    println!("Challenge: are_similar");
}

pub fn are_similar(mut a: Vec<i32>, mut b: Vec<i32>) -> bool {
    let mut count = 0;
    let count_size = a.len() - 1;

    for x in 0..count_size {
        if a.iter().any(|&i| i == b[x]) {
            let index_a = x;
            let index_b = b.iter().position(|&r| r == a[x]);
            if !index_b.is_none() {
                if index_a != index_b.unwrap() {
                    count = count + 1;
                }
            }
        }
    }
    println!("count {}", count);
    if count > 2 {
        return false;
    } else {
        a.sort();
        b.sort();
        let matching = a.iter().zip(&b).filter(|&(a, b)| a == b).count();
        println!("matching {}", matching);
        println!("len {}", a.len());
        if matching != a.len() {
            return false;
        }
        return true;
    }
}

#[test]
fn test_similar_vec() {
    assert_eq!(are_similar(vec![4, 6, 3], vec![3, 4, 6]), false);
    // assert_eq!(are_similar(vec![1, 2, 3], vec![2, 1, 3]), true);
    // assert_eq!(are_similar(vec![1, 2, 3], vec![1, 10, 2]), false);
    assert_eq!(are_similar(vec![1, 2, 1], vec![2, 1, 1]), true);
    // assert_eq!(are_similar(vec![3, 2, 1], vec![2, 3, 1]), true);
    // assert_eq!(are_similar(vec![1, 2, 2], vec![2, 1, 1]), false);
    // assert_eq!(are_similar(vec![4, 6, 3], vec![3, 4, 6]), false);
    // assert_eq!(are_similar(vec![2, 3, 1], vec![1, 3, 2]), true);
    // assert_eq!(
    //     are_similar(
    //         vec![832, 998, 148, 570, 533, 561, 894, 147, 455, 279],
    //         vec![832, 998, 148, 570, 533, 561, 455, 147, 894, 279]
    //     ),
    //     true
    // );
}

fn reshape_matrix(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let r = r as usize;
    let c = c as usize;
    let rr = mat.len();
    let cc = mat[0].len();
    if r * c != rr * cc {
        return mat;
    }
    let mut res: Vec<Vec<i32>> = vec![vec![0; c]; r];
    for i in 0..r * c {
        res[i / c][i % c] = mat[i / cc][i % cc];
    }
    res
}

#[test]
fn test_reshape_matrix() {
    assert_eq!(
        reshape_matrix(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );
    assert_eq!(
        reshape_matrix(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![vec![1, 2], vec![3, 4]]
    );
}

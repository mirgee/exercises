pub fn pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    let mut rows = vec![];
    if num_rows == 0 {
        return rows;
    }
    rows.push(vec![1]);

    for _ in 1..num_rows {
        let prev = rows.last().unwrap();
        let mut new_row = vec![1];
        for i in 1..prev.len() {
            new_row.push(prev[i-1] + prev[i]);
        }
        new_row.push(1);
        rows.push(new_row);
    }
    
    return rows;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_triangle_0() {
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        let actual = pascal_triangle(5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pascal_triangle_1() {
        let expected = vec![vec![1]];
        let actual = pascal_triangle(1);
        assert_eq!(expected, actual);
    }
}

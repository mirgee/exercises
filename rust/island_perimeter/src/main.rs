fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let h = grid.len();
    let w = grid[0].len();
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 1 {
                if i == 0 || grid[i-1][j] == 0 { res += 1; }
                if j == w - 1 || grid[i][j+1] == 0 { res += 1; }
                if i == h - 1 || grid[i+1][j] == 0 { res += 1; }
                if j == 0 || grid[i][j-1] == 0 { res += 1; }
            }
        }
    }
    res as i32
}

#[test]
fn test_island_perimeter() {
    let grid = vec![vec![0,1,0,0],vec![1,1,1,0],vec![0,1,0,0],vec![1,1,0,0]];
    assert_eq!(island_perimeter(grid), 16);
    assert_eq!(island_perimeter(vec![vec![1]]), 4);
    assert_eq!(island_perimeter(vec![vec![1,0]]), 4);
}

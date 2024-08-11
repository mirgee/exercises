pub fn popularity_analysis(scores: Vec<i32>) -> bool {
    let mut non_decreasing = true;
    let mut non_increasing = true;

    for i in 0..scores.len() - 1 {
        if scores[i + 1] > scores[i] {
            non_increasing = false;
        } else if scores[i + 1] < scores[i] {
            non_decreasing = false;
        }
    }

    non_decreasing || non_increasing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popularity_analysis_0() {
        let movie_ratings = vec![
            vec![1, 2, 2, 3],
            vec![4, 5, 6, 3, 4],
            vec![8, 8, 7, 6, 5, 4, 4, 1],
        ];
        let expected = [true, false, true];
        for (i, ratings) in movie_ratings.into_iter().enumerate() {
            println!("{}", i);
            assert_eq!(popularity_analysis(ratings), expected[i]);
        }
    }
}

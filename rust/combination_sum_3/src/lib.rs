// https://leetcode.com/problems/combination-sum-iii

fn solve(solution: &mut Vec<Vec<i32>>, candidate: &mut Vec<i32>, k: i32, n: i32) {
    if candidate.len() == k as usize && candidate.iter().sum::<i32>() == n {
        solution.push(candidate.clone());
        return;
    }

    let start = if let Some(last) = candidate.last() {
        last + 1
    } else {
        1
    };

    for to_add in start..=9 {
        if candidate.contains(&to_add) {
            continue;
        }
        if candidate.iter().sum::<i32>() + to_add > n {
            break;
        }
        candidate.push(to_add);
        solve(solution, candidate, k, n);
        candidate.pop();
    }
}

pub fn combination_sum_3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    solve(&mut res, &mut vec![], k, n);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_3_0() {
        let expected = vec![vec![1, 2, 4]];
        let actual = combination_sum_3(3, 7);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_combination_sum_3_1() {
        let expected = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
        let actual = combination_sum_3(3, 9);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_combination_sum_3_2() {
        let expected = Vec::<Vec<i32>>::new();
        let actual = combination_sum_3(4, 1);
        assert_eq!(expected, actual);
    }
}

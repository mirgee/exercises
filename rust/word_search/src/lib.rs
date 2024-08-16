// https://leetcode.com/problems/word-search/

pub fn word_search(mut board: Vec<Vec<char>>, word: String) -> bool {
    fn solve(board: &mut Vec<Vec<char>>, word: &str, row: usize, col: usize) -> bool {
        if word.len() == 0 { return true; }

        let tmp = board[row][col];
        board[row][col] = '#';

        if row > 0 && board[row-1][col] == word.chars().next().unwrap() {
            if solve(board, &word[1..], row-1, col) {
                return true;
            }
        }

        if row + 1 < board.len() && board[row+1][col] == word.chars().next().unwrap() {
            if solve(board, &word[1..], row+1, col) {
                return true;
            }
        }

        if col > 0 && board[row][col-1] == word.chars().next().unwrap() {
            if solve(board, &word[1..], row, col-1) {
                return true;
            }
        }

        if col + 1 < board[0].len() && board[row][col+1] == word.chars().next().unwrap() {
            if solve(board, &word[1..], row, col+1) {
                return true;
            }
        }

        board[row][col] = tmp;

        false
    }
    
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if word.chars().next().unwrap() == board[row][col] {
                if solve(&mut board, &word[1..], row, col) {
                    return true;
                }
            }
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search_0() {
        let expected = true;
        let actual = word_search(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_string(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_word_search_1() {
        let expected = true;
        let actual = word_search(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "SEE".to_string(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_word_search_2() {
        let expected = false;
        let actual = word_search(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCB".to_string(),
        );
        assert_eq!(expected, actual);
    }
}

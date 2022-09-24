use std::collections::HashMap;

fn hash_map() -> HashMap<&'static str, usize> {
    HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
    ])
}

fn determine_color_of_chessboard_square(coordinates: String) -> bool {
    let c: Vec<char> = coordinates.chars().into_iter().collect();
    let x: usize = hash_map().get(c[0].to_string().as_str()).unwrap().clone();
    let y = c[1].to_digit(10).unwrap();
    if x % 2 == 0 {
        if y % 2 == 0 {
            false
        } else {
            true
        }
    } else {
        if y % 2 == 0 {
            true
        } else {
            false
        }
    }
}

#[test]
fn test_determine_color_of_chessboard_square() {
    assert_eq!(determine_color_of_chessboard_square("a1".to_string()), false);
    assert_eq!(determine_color_of_chessboard_square("h3".to_string()), true);
    assert_eq!(determine_color_of_chessboard_square("c7".to_string()), false);
}

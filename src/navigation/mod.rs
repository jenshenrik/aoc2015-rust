pub fn read_directions(directions: &str) -> i32 {
    let mut i = 0;
    for c in directions.chars() {
        i += move_cursor(c);
    }

    i
}

pub fn find_basement(directions: &str) -> i32 {
    let mut p = 0;
    for (i, c) in directions.chars().enumerate() {
        p += move_cursor(c);

        if p == -1 {
            return (i + 1) as i32;
        }
    }

    -1
}

fn move_cursor(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_1() {
        let result = read_directions("(())");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_direction_2() {
        let result = read_directions("(((");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_direction_3() {
        let result = read_directions("(()(()(");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_direction_4() {
        let result = read_directions("))(((((");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_direction_5() {
        let result = read_directions("())");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_direction_6() {
        let result = read_directions("))(");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_direction_7() {
        let result = read_directions(")())())");
        assert_eq!(result, -3);
    }

    #[test]
    fn test_find_position_1() {
        let result = find_basement(")");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_position_2() {
        let result = find_basement("()())");
        assert_eq!(result, 5);
    }
}

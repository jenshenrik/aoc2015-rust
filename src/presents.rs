use std::cmp::min;

pub fn calculate_gift_wrapping(w: i32, l: i32, h: i32) -> i32 {
    let (side_a, side_b, side_c) = calculate_sides(w, l, h);
    let min_side = min(side_a, min(side_b, side_c));

    ((side_a + side_b + side_c) * 2) + min_side
}

fn calculate_sides(w: i32, l: i32, h: i32) -> (i32, i32, i32) {
    (w * l, w * h, l * h)
}

pub fn calculate_ribbon(w: i32, l: i32, h: i32) -> i32 {
    let perimeter = calculate_smallest_perimeter(w, l, h);
    let volume = calculate_volume(w, l, h);

    perimeter + volume
}

fn calculate_smallest_perimeter(w: i32, l: i32, h: i32) -> i32 {
    let p1 = calculate_perimeter(w, l);
    let p2 = calculate_perimeter(w, h);
    let p3 = calculate_perimeter(l, h);

    min(p1, min(p2, p3))
}

fn calculate_perimeter(w: i32, l: i32) -> i32 {
    (w + l) * 2
}

fn calculate_volume(w: i32, l: i32, h: i32) -> i32 {
    w * l * h
}

#[cfg(test)]
mod tests {
    use crate::presents::{calculate_gift_wrapping, calculate_ribbon};

    #[test]
    fn test_calculate_wrapping_1() {
        let result = calculate_gift_wrapping(2, 3, 4);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_calculate_wrapping_2() {
        let result = calculate_gift_wrapping(1, 1, 10);
        assert_eq!(result, 43);
    }

    #[test]
    fn test_calculate_ribbon_1() {
        let result = calculate_ribbon(2, 3, 4);
        assert_eq!(result, 34);
    }

    #[test]
    fn test_calculate_ribbon_2() {
        let result = calculate_ribbon(1, 1, 10);
        assert_eq!(result, 14);
    }
}

use md5::Digest;

pub fn mine(key: &str, must_match: &str) -> usize {
    let mut coin = 0;
    loop {
        if try_coin(key, coin, must_match) {
            break;
        } else {
            coin += 1;
        }
    }

    coin
}

fn try_coin(key: &str, coin: usize, must_match: &str) -> bool {
    let input = format!("{key}{coin}");

    let hash = format!("{:x}", md5::Md5::digest(input));

    hash.starts_with(must_match)
}

#[cfg(test)]
mod tests {
    use crate::adventcoin::{mine, try_coin};

    #[test]
    fn test_try_coin_1() {
        assert!(try_coin("abcdef", 609043, "00000"));
    }

    #[test]
    fn test_try_coin_2() {
        assert!(try_coin("pqrstuv", 1048970, "00000"));
    }

    #[test]
    fn test_mine_1() {
        let result = mine("abcdef", "00000");

        assert_eq!(result, 609043);
    }

    #[test]
    fn test_mine_2() {
        let result = mine("pqrstuv", "00000");

        assert_eq!(result, 1048970);
    }
}

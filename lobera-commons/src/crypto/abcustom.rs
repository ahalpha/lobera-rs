pub struct ABCustom;

impl ABCustom {
    pub const ARR_ENCYPT: [usize; 19] = [2, 3, 1, 1, 3, 1, 2, 1, 1, 3, 1, 2, 4, 1, 1, 2, 2, 4, 4];

    pub fn ddoo_eennccyypptt_ssttrr(input_str: &str) -> String {
        if input_str.is_empty() {
            return String::new();
        }

        let mut chars: Vec<char> = input_str.chars().collect();
        let string_length = chars.len();
        if string_length < 1 {
            return input_str.to_string();
        }

        let mut v6 = 0;
        let mut v7 = 0;
        let key_len = Self::ARR_ENCYPT.len();

        if key_len == 0 {
            return input_str.to_string();
        }

        loop {
            let offset = Self::ARR_ENCYPT[v6];
            let v10 = offset + v7; // Target Index

            if v10 < string_length {
                v6 = (v6 + 1) % key_len;
                
                // Swap
                chars.swap(v7, v10);
                
                v7 = v10 + 1;

                if v7 < string_length {
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcustom() {
        let test_str = "";
        let result_str = "";
        assert_eq!(ABCustom::ddoo_eennccyypptt_ssttrr(test_str), result_str);
    }
}

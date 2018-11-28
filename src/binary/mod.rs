// 定义
// 如果一个字符串只包含'0'和'1'，那么这个字符串定义为二进制字符串

const DOT: u8 = 46;
const ZERO: u8 = 48;
const ONE: u8 = 49;

// 判断包含小数点的字符串是不是二进制字符串
pub fn is_binary_string(_input: &str) -> bool {
    // 过滤头尾控制符和空格
    let input = _input.trim();

    // 超过两个小数点，肯定不是一个数字
    // 最后一个小数点和地一个小数点所在的位置不一样，肯定不是一个数字
    if input.rfind(".") != input.find(".") {
        return false;
    }

    let bytes = input.as_bytes();

    // 如果以小数点开头，或者以小数点结尾
    if bytes.starts_with(&[DOT]) || bytes.ends_with(&[DOT]) {
        return false;
    }

    for ele in bytes.iter() {
        match [DOT, ZERO, ONE].contains(ele) {
            true => continue,
            false => return false,
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::is_binary_string;

    #[test]
    pub fn bin_str_p() {
        assert_eq!(is_binary_string("11001100.1010101010"), true);
        assert_eq!(is_binary_string(".110011001010101010"), false);
        assert_eq!(is_binary_string("110011001010101010."), false);
        assert_eq!(is_binary_string("     11001100.1010101010"), true);
        assert_eq!(is_binary_string("     11001100.    1010101010"), false);
        assert_eq!(is_binary_string("     11001100.1010101010    "), true);
        assert_eq!(is_binary_string("11001100.1010101012"), false);
    }
}

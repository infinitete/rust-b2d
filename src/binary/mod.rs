// 定义
// 如果一个字符串只包含'0'和'1'，那么这个字符串定义为二进制字符串

const DOT: u8 = 46;
const ZERO: u8 = 48;
const ONE: u8 = 49;

pub struct BinaryString<'a> {
    pub raw: &'a str,
    value: Result<f64, &'a str>,
}

impl<'a> BinaryString<'a> {
    pub fn new(raw: &str) -> BinaryString {
        BinaryString {
            raw: raw,
            value: Err(""),
        }
    }

    pub fn is_binary_string(&self) -> bool {
        match self.raw.find(".") {
            Some(pos) => {
                if pos > 31 {
                    return false;
                }
            }

            None => {}
        }

        is_binary_string(&self.raw)
    }

    pub fn to_decimal(&mut self) -> Result<f64, &str> {
        if !self.is_binary_string() {
            return Err("Parse Error");
        }

        if self.value.is_ok() {
            return self.value;
        }

        self.value = binary2decimal(&self.raw);
        self.value
    }
}

// 判断包含小数点的字符串是不是二进制字符串
fn is_binary_string(_input: &str) -> bool {
    // 过滤头尾控制符和空格
    let input = _input.trim();

    // 空字符串不是肯定一个数字
    // 超过两个小数点，肯定不是一个数字
    // 最后一个小数点和地一个小数点所在的位置不一样，肯定不是一个数字
    if input.len() == 0 || input.rfind(".") != input.find(".") {
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

fn binary2decimal(bs: &str) -> Result<f64, &str> {
    let bytes = bs.as_bytes();
    // let len = bs.len() as u32;

    // let mut value = 0f64;

    // for (i, item) in bytes.iter().enumerate() {
    //     // 符号位
    //     if i == 0 {
    //         continue;
    //     }

    //     // 之所以 len - (i + 1)
    //     // 是因为:
    //     //         第一位是符号位
    //     //         slice下标从0开始
    //     let pow = 2u32.pow(len - (i as u32) - 1) as f64;
    //     value += ((item - ZERO) as f64) * pow;
    // }

    // if bytes.first().unwrap() == &48 {
    //     return Ok(value);
    // }

    // Ok(-value)

    if !bs.contains(".") {
        return Ok(inteiger_part(bs));
    }

    let parts = bs.split(".").into_iter();
    let mut value = 0f64;

    for (i, item) in parts.enumerate() {
        if i == 0 {
            value += inteiger_part(item);
            continue;
        }

        if bytes.starts_with(&[ONE]) {
            value -= decimal_part(item);
        } else {
            value += decimal_part(item);
        }
    }

    Ok(value)
}

fn inteiger_part(ip: &str) -> f64 {
    let bytes = ip.as_bytes();
    let len = ip.len() as u32;

    let mut value = 0f64;

    for (i, item) in bytes.iter().enumerate() {
        // 符号位
        if i == 0 {
            continue;
        }

        // 之所以 len - (i + 1)
        // 是因为:
        //         第一位是符号位
        //         slice下标从0开始
        let pow = 2u32.pow(len - (i as u32) - 1) as f64;
        value += ((item - ZERO) as f64) * pow;
    }

    if bytes.first().unwrap() == &48 {
        return value;
    }

    -value
}

fn decimal_part(dp: &str) -> f64 {
    let bytes = dp.as_bytes();
    let mut value = 0f64;

    for (i, item) in bytes.iter().enumerate() {
        let zo = (item - ZERO) as f64;
        let times = i + 1;
        let pow = 2u32.pow(times as u32) as f64;
        let dn: f64 = zo / pow;

        value += dn as f64
    }

    println!("小数部分： {}", value);

    return value;
}

#[cfg(test)]
mod test {
    use super::is_binary_string;
    use super::BinaryString;

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

    #[test]
    pub fn binary_string_test() {
        assert!(BinaryString::new("11001100.1010101010").is_binary_string());
        assert!(!BinaryString::new(".110011001010101010").is_binary_string());
        assert!(!BinaryString::new("110011001010101010.").is_binary_string());
        assert!(BinaryString::new("     11001100.1010101010").is_binary_string());
        assert!(!BinaryString::new("     11001100.    1010101010").is_binary_string());
        assert!(BinaryString::new("     11001100.1010101010    ").is_binary_string());
        assert!(!BinaryString::new("11001100.1010101012").is_binary_string());
    }

    #[test]
    pub fn binary2decimal() {
        assert_eq!(
            BinaryString::new("0.1010101010").to_decimal().unwrap(),
            -4009.166666906327f64
        );
    }
}

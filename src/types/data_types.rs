#[derive(Debug, PartialEq)]
pub struct Err {
    msg: String,
}

impl Err {
    pub fn new(msg: &str) -> Err {
        Err {
            msg: msg.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrType {
    InvalidStringEncode(Err),
    InvalidBulkStringEncode(Err),
    InvalidInteger(Err),
    InvalidEncodeStr(Err),
}

impl ErrType {
    pub fn print(&self) -> String {
        //TODO: handle each Data type later
        match self {
            ErrType::InvalidStringEncode(e) => e.msg.clone(),
            ErrType::InvalidBulkStringEncode(e) => e.msg.clone(),
            ErrType::InvalidInteger(e) => e.msg.clone(),
            ErrType::InvalidEncodeStr(e) => e.msg.clone(),
        }
    }
}

#[derive(Debug)]
struct RString {
    data: Option<String>,
    size: usize,
}

impl RString {
    pub fn from_str(encoded_str: &str, bulk: bool) -> Result<RString, ErrType> {
        let components: Vec<&str> = encoded_str.split_terminator("\r\n").collect();
        println!("components: {:?}", components);
        let r_string_size: (Option<String>, usize) = match bulk {
            true => {
                if components.len() < 1 {
                    return Err(ErrType::InvalidBulkStringEncode(Err::new(
                        "invalid data for string len less than 1",
                    )));
                }

                let cap = match components[0].parse::<i8>() {
                    Ok(c) => c,
                    Err(e) => {
                        return Err(ErrType::InvalidBulkStringEncode(Err::new(
                            e.to_string().as_str(),
                        )))
                    }
                };

                let mut cap_usize = cap as usize;

                let result: Option<String> = match cap {
                    -1 => {
                        cap_usize = 0;
                        None
                    }
                    _ => {
                        let mut tmp = String::with_capacity(cap_usize);
                        let str_part: Vec<char> = components[1]
                            .as_bytes()
                            .iter()
                            .map(|b| char::from(*b))
                            .collect();

                        for i in 0..cap_usize {
                            tmp.insert(i, str_part[i])
                        }

                        Some(tmp)
                    }
                };

                (result, cap_usize)
            }

            false => {
                if components.len() != 1 {
                    return Err(ErrType::InvalidStringEncode(Err::new(
                        "invalid data for string len not eq 1",
                    )));
                }

                (Some(components[0].to_string()), components[0].len())
            }
        };

        Ok(RString {
            data: r_string_size.0,
            size: r_string_size.1,
        })
    }
}
#[derive(Debug)]
struct RInteger {
    data: i64,
}

impl RInteger {
    pub fn from_str(encoded_str: &str) -> Result<RInteger, ErrType> {
        let components: Vec<&str> = encoded_str.split_terminator("\r\n").collect();
        let data: i64 = match components[0].parse::<i64>() {
            Ok(int) => int,
            Err(e) => return Err(ErrType::InvalidInteger(Err::new(e.to_string().as_str()))),
        };
        Ok(RInteger { data: data })
    }
}

struct RNull {
    data: Option<()>,
}

impl RNull {
    pub fn from_str() -> Result<RNull, ()> {
        Ok(RNull { data: None })
    }
}

struct RBool {
    data: bool,
}

impl RBool {
    pub fn from_str(encoded_str: &str) -> Result<RBool, ErrType> {
        let components: Vec<&str> = encoded_str.split_terminator("\r\n").collect();
        let components_size: usize = components.len();

        if components_size != 1 {
            return Err(ErrType::InvalidEncodeStr(Err::new("invalid size")));
        }

        let data: bool = match components[0] {
            "t" => true,
            "f" => false,
            _ => return Err(ErrType::InvalidEncodeStr(Err::new("invalud char"))),
        };
        Ok(RBool { data: data })
    }
}

#[derive(Clone, Copy)]
enum INFINITY {
    Pos,
    Neg,
    Null,
}

struct RDouble {
    data: Option<f64>,
    exponent: i32,
    // precesion: u8,
    inf: INFINITY,
}

impl RDouble {
    pub fn from_str(encoded_str: &str) -> Result<RDouble, ErrType> {
        let components: Vec<&str> = encoded_str.split_terminator("\r\n").collect();
        let components_size: usize = components.len();

        if components_size != 1 {
            return Err(ErrType::InvalidEncodeStr(Err::new("invalid size")));
        }

        let double_part_lower = components[0].to_ascii_lowercase();
        let result: (Option<f64>, i32, INFINITY) = match double_part_lower.contains("e") {
            true => {
                let double_exponent_vec: Vec<&str> =
                    double_part_lower.split_terminator("e").collect();
                let integer_fractional: f64 = double_exponent_vec[0].parse::<f64>().unwrap();
                let exponent: i32 = double_exponent_vec[1].parse::<i32>().unwrap();

                (Some(integer_fractional), exponent, INFINITY::Null)
            }

            false => {
                let res: (Option<f64>, INFINITY) = match double_part_lower.as_str() {
                    "inf" => (None, INFINITY::Pos),
                    "-inf" => (None, INFINITY::Neg),
                    _ => {
                        let integer_fractional: f64 = double_part_lower.parse::<f64>().unwrap();
                        (Some(integer_fractional), INFINITY::Null)
                    }
                };
                (res.0, 0, res.1)
            }
        };

        Ok(RDouble {
            data: result.0,
            exponent: result.1,
            inf: result.2,
        })
    }
}

trait My {
    fn ok(&self);
}

struct RArray {
    data: Vec<Types>,
    size: usize,
}

impl RArray {
    pub fn from_str(command: &str) -> Result<RArray, ErrType> {
        let size_elements_str = command.split_once("\r\n").unwrap();

        let size = match size_elements_str.0.parse::<usize>() {
            Ok(s) => s,
            Err(e) => return Err(ErrType::InvalidEncodeStr(Err::new("size is not parseable"))),
        };

        let mut types_str: Vec<&str> = size_elements_str.1.split_inclusive("\r\n").collect();

        let data: Vec<Types> = types_str
            .iter_mut()
            .map(|v| Types::from_str(v).unwrap())
            .collect();

        Ok(RArray { data: data, size })
    }
}

enum Types {
    Integer(RInteger),
    String(RString),
    Null(RNull),
    Bool(RBool),
    Double(RDouble),
    Array(RArray),
}

impl Types {
    fn from_str(command: &str) -> Result<Types, ErrType> {
        if command.len() < 2 {
            return Err(ErrType::InvalidEncodeStr(Err::new("ok")));
        }
        let parseable: &str = &command[1..];
        let type_identifier: u8 = command.as_bytes()[0];
        let parsed_type = match type_identifier {
            b'+' => Types::String(RString::from_str(parseable, false).unwrap()),
            b':' => Types::Integer(RInteger::from_str(parseable).unwrap()),
            b'*' => Types::Array(RArray::from_str(parseable).unwrap()),
            b'_' => Types::Null(RNull::from_str().unwrap()),
            b'#' => Types::Bool(RBool::from_str(parseable).unwrap()),
            b',' => Types::Double(RDouble::from_str(parseable).unwrap()),
            _ => {
                return Err(ErrType::InvalidEncodeStr(Err::new(
                    "unparseable command format",
                )))
            }
        };

        Ok(parsed_type)
    }
}
#[cfg(test)]
mod tests {
    use crate::types::data_types::{
        Err, ErrType, RArray, RBool, RDouble, RInteger, RString, INFINITY,
    };

    use super::Types;

    #[test]
    fn valid_simple_str() {
        let command = "OK\r\n";
        let r_string: RString = RString::from_str(command, false).unwrap();
        assert_eq!(r_string.data, Some("OK".to_string()));
    }

    #[test]
    fn invalid_simple_str() {
        let command = "\r\n";
        let r_string: RString = RString::from_str(command, false).unwrap();
        assert_eq!(r_string.data, Some("".to_string()));
    }

    #[test]
    fn valid_bulk_str() {
        let command = "5\r\nhello\r\n";
        let r_string: RString = RString::from_str(command, true).unwrap();
        assert_eq!(r_string.data, Some("hello".to_string()));
        assert_eq!(r_string.size, 5);
    }

    // #[test]
    // fn invalid_bulk_str_cap() {
    //     let command = "s\r\nhello\r\n";
    //     let r_string_err: ErrType = RString::from_str(command, true).unwrap_err();
    //     assert_eq!(r_string_err, ErrType::InvalidBulkStringEncode(Err::new(ParseIntError::)));
    // }

    #[test]
    fn valid_null_bulk_str() {
        let command = "-1\r\n";
        let r_string: RString = RString::from_str(command, true).unwrap();
        assert_eq!(r_string.data, None);
        assert_eq!(r_string.size, 0);
    }

    #[test]
    fn valid_neg_integer() {
        let command = "-89\r\n";
        let r_string: RInteger = RInteger::from_str(command).unwrap();
        assert_eq!(r_string.data, -89);
    }

    #[test]
    fn valid_pos_integer() {
        let command = "89\r\n";
        let r_string: RInteger = RInteger::from_str(command).unwrap();
        assert_eq!(r_string.data, 89);
    }

    //TODO: handle test for integer

    #[test]
    fn valid_true_bool() {
        let command = "t\r\n";
        let r_string: RBool = RBool::from_str(command).unwrap();
        assert_eq!(r_string.data, true);
    }

    #[test]
    fn valid_false_bool() {
        let command = "f\r\n";
        let r_string: RBool = RBool::from_str(command).unwrap();
        assert_eq!(r_string.data, false);
    }
    //TODO: handle test for bool

    #[test]
    fn valid_rdouble() {
        let command = "10.67\r\n";
        let r_string: RDouble = RDouble::from_str(command).unwrap();
        assert_eq!(r_string.data, Some(10.67));
    }

    #[test]
    fn valid_rdouble_exponent() {
        let command = "-123.45E+6\r\n";
        let r_string: RDouble = RDouble::from_str(command).unwrap();
        assert_eq!(r_string.data, Some(-123.45 as f64));
        assert_eq!(r_string.exponent, 6);
    }

    #[test]
    fn valid_rdouble_pos_inf() {
        let command = "inf\r\n";
        let r_string: RDouble = RDouble::from_str(command).unwrap();
        let same = match r_string.inf {
            INFINITY::Pos => true,
            _ => false,
        };
        assert_eq!(same, true);
    }
    #[test]
    fn valid_rdouble_neg_inf() {
        let command = "-inf\r\n";
        let r_string: RDouble = RDouble::from_str(command).unwrap();
        let same = match r_string.inf {
            INFINITY::Neg => true,
            _ => false,
        };
        assert_eq!(same, true);
    }

    //TODO: handle test for double

    #[test]
    fn valid_array_pos_integer() {
        let command = "2\r\n:1\r\n:2\r\n";
        let r_array: RArray = RArray::from_str(command).unwrap();

        let is_same_size: bool = match r_array.size {
            2 => true,
            _ => false,
        };
        assert_eq!(is_same_size, true);

        if is_same_size {
            for i in 0..r_array.size {
                let is_integer: (bool, i64) = match &r_array.data[i] {
                    Types::Integer(I) => (true, I.data),
                    _ => (false, 0),
                };

                assert_eq!(is_integer.0, true);
                assert_eq!(is_integer.1, (i + 1) as i64);
            }
        }
    }

    #[test]
    fn valid_array_neg_integer() {
        let command = "2\r\n:-1\r\n:-2\r\n";
        let r_array: RArray = RArray::from_str(command).unwrap();

        let is_same_size: bool = match r_array.size {
            2 => true,
            _ => false,
        };
        assert_eq!(is_same_size, true);

        if is_same_size {
            let expected = vec![-1, -2];
            for i in 0..r_array.size {
                let is_integer: (bool, i64) = match &r_array.data[i] {
                    Types::Integer(I) => (true, I.data),
                    _ => (false, 0),
                };

                assert_eq!(is_integer.0, true);
                assert_eq!(is_integer.1, expected[i]);
            }
        }
    }

    #[test]
    fn valid_array_i64() {
        let command = "2\r\n:-1\r\n:2\r\n";
        let r_array: RArray = RArray::from_str(command).unwrap();

        let is_same_size: bool = match r_array.size {
            2 => true,
            _ => false,
        };
        assert_eq!(is_same_size, true);

        if is_same_size {
            let expected = vec![-1, 2];
            for i in 0..r_array.size {
                let is_integer: (bool, i64) = match &r_array.data[i] {
                    Types::Integer(I) => (true, I.data),
                    _ => (false, 0),
                };

                assert_eq!(is_integer.0, true);
                assert_eq!(is_integer.1, expected[i]);
            }
        }
    }

    #[test]
    fn valid_array_simple_string() {
        let command = "2\r\n+yes\r\n+no\r\n";
        let r_array: RArray = RArray::from_str(command).unwrap();

        let is_same_size: bool = match r_array.size {
            2 => true,
            _ => false,
        };
        assert_eq!(is_same_size, true);

        if is_same_size {
            let expected = vec!["yes", "no"];
            for i in 0..r_array.size {
                let is_string: (bool, Option<String>) = match &r_array.data[i] {
                    Types::String(s) => (true, s.data.clone()),
                    _ => (false, None),
                };

                assert_eq!(is_string.0, true);
                assert_eq!(is_string.1, Some(expected[i].to_string()));
            }
        }
    }
    //TODO: handle bulk string testing
    //TODO: handle null array

    #[test]
    fn valid_array_bool() {
        let command = "2\r\n#t\r\n#f\r\n";
        let r_array: RArray = RArray::from_str(command).unwrap();

        let is_same_size: bool = match r_array.size {
            2 => true,
            _ => false,
        };
        assert_eq!(is_same_size, true);

        if is_same_size {
            let expected = vec![true, false];
            for i in 0..r_array.size {
                let is_string: (bool, bool) = match &r_array.data[i] {
                    Types::Bool(b) => (true, b.data),
                    _ => (false, false),
                };

                assert_eq!(is_string.0, true);
                assert_eq!(is_string.1, expected[i]);
            }
        }
    }

    #[test]
    fn valid_array_double() {
        let command = "5\r\n,1.23\r\n,1.23E+2\r\n,12.3E-2\r\n,inf\r\n,-inf\r\n";
        let r_array: RArray = RArray::from_str(command).unwrap();

        let is_same_size: bool = match r_array.size {
            5 => true,
            _ => false,
        };
        assert_eq!(is_same_size, true);

        if is_same_size {
            let expected = vec![
                RDouble {
                    data: Some(1.23),
                    exponent: 0,
                    inf: INFINITY::Null,
                },
                RDouble {
                    data: Some(1.23),
                    exponent: 2,
                    inf: INFINITY::Null,
                },
                RDouble {
                    data: Some(12.3),
                    exponent: -2,
                    inf: INFINITY::Null,
                },
                RDouble {
                    data: None,
                    exponent: 0,
                    inf: INFINITY::Pos,
                },
                RDouble {
                    data: None,
                    exponent: 0,
                    inf: INFINITY::Neg,
                },
            ];
            for i in 0..r_array.size {
                let is_double: (bool, Option<f64>, i32, INFINITY) = match &r_array.data[i] {
                    Types::Double(d) => (true, d.data, d.exponent, d.inf.clone()),
                    _ => (false, None, 0, INFINITY::Pos),
                };

                assert_eq!(is_double.0, true);
                assert_eq!(is_double.1, expected[i].data);
                assert_eq!(is_double.2, expected[i].exponent);
                // assert_eq!(is_double.3, expected[i].inf);
            }
        }
    }
}

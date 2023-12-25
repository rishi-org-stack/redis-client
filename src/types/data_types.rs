struct RString {
    data: Option<String>,
    size: usize,
}

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
pub enum ErrType {
    InvalidStringEncode(Err),
    InvalidBulkStringEncode(Err),
}

impl ErrType {
    pub fn print(&self) -> String {
        //TODO: handle each Data type later
        match self {
            ErrType::InvalidStringEncode(e) => e.msg.clone(),
            ErrType::InvalidBulkStringEncode(e) => e.msg.clone(),
        }
    }
}

impl RString {
    pub fn from_str(encoded_str: &str, bulk: bool) -> Result<RString, ErrType> {
        let components: Vec<&str> = encoded_str.split("\r\n").collect();
        let r_string_size = match bulk {
            true => {
                if components.len() != 2 {
                    return Err(ErrType::InvalidBulkStringEncode(Err::new(
                        "invalid data for string",
                    )));
                }

                let str_part: Vec<char> = components[1]
                    .as_bytes()
                    .iter()
                    .map(|b| char::from(*b))
                    .collect();

                let cap = components[0].parse::<usize>().unwrap();
                let mut result = String::with_capacity(cap);

                for i in 0..cap {
                    result.insert(i, str_part[i])
                }

                (result, cap)
            }

            false => {
                if components.len() != 1 {
                    return Err(ErrType::InvalidStringEncode(Err::new(
                        "invalid data for string",
                    )));
                }

                (components[0].to_string(), components[0].len())
            }
        };

        Ok(RString {
            data: Some(r_string_size.0),
            size: r_string_size.1,
        })
    }
}

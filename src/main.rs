pub mod types;

fn main() {
    println!("Hello, world!");
    let mut v = vec![0; 3];
    v[0] = 1;
    v[1] = 2;
    v[2] = 3;
    let x = v.iter().next().unwrap();
    println!("x: {}", x);
    let y = v.iter().next().unwrap();
    println!("y: {}", y);
    let z = v.iter().next().unwrap();
    println!("z: {}", z)
}
// string
// +OK\r\n
// simple ERROR
// -Error message\r\n
// Integer
// :[<+|->]<value>\r\n

//     The colon (:) as the first byte.
//     An optional plus (+) or minus (-) as the sign.
//     One or more decimal digits (0..9) as the integer's unsigned, base-10 value.
//     The CRLF terminator.
// BULK string
// $<length>\r\n<data>\r\n

//     The dollar sign ($) as the first byte.
//     One or more decimal digits (0..9) as the string's length, in bytes, as an unsigned, base-10 value.
//     The CRLF terminator.
//     The data.
//     A final CRLF.
// NULL BULK string
// $-1\r\n

// ARRAY
// *<number-of-elements>\r\n<element-1>...<element-n>

//     An asterisk (*) as the first byte.
//     One or more decimal digits (0..9) as the number of elements in the array as an unsigned, base-10 value.
//     The CRLF terminator.
//     An additional RESP type for every element of the array.
// ex-1
//     *5\r\n
//     :1\r\n
//     :2\r\n
//     :3\r\n
//     :4\r\n
//     $5\r\n
//     hello\r\n
// ex-2
//     *2\r\n
//     *3\r\n
//     :1\r\n
//     :2\r\n
//     :3\r\n
//     *2\r\n
//     +Hello\r\n
//     -World\r\n
// ex-3 null array
//     *-1\r\n
//     array With null
//         *-1\r\n

// NULLS
//     _\r\n

// BOOLEAN
//     #<t|f>\r\n

// DOUBLE
//     ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n

// ex-1
//     ,1.23\r\n
// ex-2
//     :10\r\n
//     ,10\r\n
// ex-2 [infinty and nan]
//     ,inf\r\n
//     ,-inf\r\n
//     ,nan\r\n

// Big Numbers
//     ([+|-]<number>\r\n
//     ex-1
//     (3492890328409238509324850943850943825024385\r\n

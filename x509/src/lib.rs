#![no_std]
extern crate alloc;
use alloc::string::String;
use x509_parser::pem::Pem;
use ascii_converter::string_to_decimals;

pub fn get_result(input: &str) -> String {
    let data = string_to_decimals(&input).expect("Could not read file");
    let mut text = String::new();
    for pem in Pem::iter_from_buffer(&data) {
        let pem = pem.expect("Reading next PEM block failed");
        let x509 = pem.parse_x509().expect("X.509: decoding DER failed");
        // x509是整个证书的解析，其中extension应该和格式相关
        let test = x509.extensions()[5].value;
        for d in test.iter(){
            if !(d >=  &32 && d <= &126) {
                break;
            } else {
                text.push(*d as char);                
            }
        }
    }
    text
}


#[cfg(test)]
mod tests {
    use  super::*;
    use alloc::string::String;
    use alloc::string::ToString;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use crate::loongarch_decomplier::data_flow::*;

pub fn analyse_extern_function(name: &String, parameters: &Vec<RegisterRecord>) -> (usize, bool) {
    let data = &*data_flow::DATA.lock().unwrap();
    match name.as_str() {
        "printf" | "scanf" | "sprintf" | "fprintf"  => {
            let param = &parameters[0];
            let string = match param {
                RegisterRecord::Number(number) => {
                    match data_flow::get_c_string_from_data(number.value as usize, data) {
                        Some(string) => string,
                        None => "".to_string(),
                    }
                }
                RegisterRecord::Symbol(symbol) => {
                    return (0, false);
                }
            };

            let mut n = 1;
            for s in string.as_bytes() {
                if *s as char == '%' {
                    n += 1; 
                }
            }

            (n, false) 
        }

        "rand" => (0, true),
        "time" => (1, true),
        "srand" => (1, true),
        "puts" => (1, true),
        //"malloc" => (1, true),
        _ => {
            (0, false)
        }
    }
}

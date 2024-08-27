use std::process;

#[derive(Debug, Clone)]
pub enum LoongArchError {
    NOTELFFILE,
    UNKNOWNENDIAN(u8),
    INVALIDELFVERSION(u8),
    NOTLOONGARCH,
    NOTLOONGARCH32,
    NOTLOONGARCH64,
}

#[derive(Debug, Clone)]
pub enum LoongArchWarning {
    NOTFOUNDSECTIONS,
    SECTIONNUMBERABNORMAL(usize, usize),
}

#[derive(Debug, Clone)] 
pub enum LoongArchOk {
    Ok,
    UNFINISH,
}


pub fn handle_loongarch_error(error: LoongArchError) {
    eprintln!("error code: {:?}", error);
    match &error {
        LoongArchError::NOTELFFILE => {
            eprintln!("not a elf file");
        }
        LoongArchError::UNKNOWNENDIAN(x) => {
            eprintln!("not big-endian or little-endian, found {}", x);
        }
        LoongArchError::INVALIDELFVERSION(v) => {
            eprintln!("invalid elf version: {}", v);
        }
        LoongArchError::NOTLOONGARCH => {
            eprintln!("not a loongarch file");
        }
        LoongArchError::NOTLOONGARCH32 => {
            eprintln!("not a loongarch 32 bit file");
        }
        LoongArchError::NOTLOONGARCH64 => {
            eprintln!("not a loongarch 64 bit file");
        }
    }
    process::exit(-1);
}

pub fn handle_loongarch_warning(warning: LoongArchWarning) {
    println!("warning code: {:?}", warning);
    match &warning {
        LoongArchWarning::NOTFOUNDSECTIONS => {
            println!("section tables offset abnormol");        
        }
        LoongArchWarning::SECTIONNUMBERABNORMAL(n1, n2) => {
            println!("the number of section tables is abnormal, expect {}, found {}", n1, n2);
        }
    }
}

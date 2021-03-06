use std::fs::File;
use std::io;
use std::io::Read;

fn read_pbn_file(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::read_pbn_file;

    #[test]
    fn test_read_pbn_file() {
        let pbn = read_pbn_file("files/pbn/basic_20.pbn").expect("file basic_20.pbn not found");

        println!("file:\n{}", pbn);
        assert!(pbn.contains("OKbridge Tournament"));
    }

    #[test]
    fn do_it() {
        let pbn = read_pbn_file("files/pbn/inc.pbn").expect("file inc.pbn not found");

        println!("file:\n{}", pbn);
        assert!(pbn.contains("OKbridge Tournament"));
    }
}

#[cfg(test)]
mod cli {
    use std::process::Command;
    use assert_cmd::prelude::*;

    #[test]
    fn main_deal_deals() {
        let mut cmd = Command::cargo_bin("bridge").unwrap();
        let _assert = cmd
            .arg("-d")
            .assert();
        let _assert = cmd
            .arg("--deal")
            .assert();
    }
}
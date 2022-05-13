use std::error::Error;

fn toml2json() -> Result<String, Box<dyn Error>> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::str::FromStr;

struct BRecord {
}

impl FromStr for BRecord {
    type Err = ();
    fn from_str(s: &str) -> Result<BRecord, ()> {
        Ok(BRecord {})
    }
}

#[cfg(test)]
mod tests {
    use BRecord;

    #[test]
    fn it_works() {
        let record = "B1414065016925N00953112EA021640228700309".parse::<BRecord>();
        assert!(record.is_ok());
    }
}

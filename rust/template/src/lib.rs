pub fn {{ crate_name }}() -> {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_{{ crate_name }}_0() {
        let expected = todo!();
        let actual = {{ crate_name }}();
        assert_eq!(expected, actual);
    }
}

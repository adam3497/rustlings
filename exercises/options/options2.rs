// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        let mut word = "";
        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(value) = optional_target {
            word = value;
        }

        assert_eq!(word, target);
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`s into while let and if let
        while let Some(i) = optional_integers.pop() {
            if let Some(value) = i {
                assert_eq!(value, cursor);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}

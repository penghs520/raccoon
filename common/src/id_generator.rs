use nanoid::nanoid;


const ALPHABET: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

pub fn generate_id() -> String {
    nanoid!(16, &ALPHABET)
}

#[cfg(test)]
mod tests {
    use crate::id_generator::generate_id;
    use std::collections::HashSet;

    #[test]
    fn it_generates_id() {
        let mut set = HashSet::new();
        for j in 0..1000_000 {
            set.insert(generate_id());
        }
        assert_eq!(1000_000, set.len());
    }
}
pub fn hash(string: &str) -> u8 {
    string
        .as_bytes()
        .into_iter()
        .fold(0u32, |acc, &new| {
            let widden: u32 = new.into();
            ((acc + widden) * 17) % 256
        })
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn example() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn verification_sequence() {
        let sequence = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let hashed: Vec<_> = sequence.split(",").map(|string| hash(string)).collect();
        assert_eq!(vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231], hashed);
    }
}

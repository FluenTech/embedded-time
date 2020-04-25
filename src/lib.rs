#![cfg_attr(not(test), no_std)]

mod duration;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

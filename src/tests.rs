mod formats {
    use crate::{Format, Formats};

    #[test]
    fn lookup() {
        let fmt = Formats::default();
        assert_eq!(fmt.lookup(0), (0usize, &Format::default()));
        assert_eq!(fmt.lookup(1), (0usize, &Format::default()));
    }
}

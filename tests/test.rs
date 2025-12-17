#[test]
fn test() {
    use enumly::Enumly;

    #[derive(Debug, Enumly, PartialEq, Eq)]
    enum Foo {
        A,
        B,
        C,
    }

    let len = Foo::COUNT;
    let list = Foo::VARIANTS;
    assert_eq!(len, 3);
    assert_eq!(list, &[Foo::A, Foo::B, Foo::C]);
}

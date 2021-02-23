fn first() {
    use strong::{validators::Email, Strong, StrongBuf, Validator};

    fn login(_email: &Strong<Email>, _password: &Strong<Password>) {}

    let email: StrongBuf<Email> = StrongBuf::<Email>::validate("a".into()).unwrap();
    let password: &Strong<Password> = Strong::<Password>::validate("b").unwrap();
    login(&email, password);

    enum Password {}
    impl Validator for Password {
        type Err = std::convert::Infallible;
    }
}

#[cfg(test)]
mod tests {
    use const_strum::ConstStr;

    #[derive(ConstStr)]
    enum TestEnum {
        Var1,
        Var2(u32),
        Var3 { a: u32, b: u32 },
    }

    const RESULT_NO_ARGS: &str = TestEnum::Var1.const_to_string();

    #[test]
    fn no_args() {
        assert_eq!(RESULT_NO_ARGS, "Var1");
    }

    const RESULT_WITH_ARGS: &str = TestEnum::Var2(42).const_to_string();

    #[test]
    fn with_args() {
        assert_eq!(RESULT_WITH_ARGS, "Var2");
    }

    const RESULT_WITH_ARGS_OBJ: &str = TestEnum::Var3 { a: 42, b: 69 }.const_to_string();

    #[test]
    fn with_args_obj() {
        assert_eq!(RESULT_WITH_ARGS_OBJ, "Var3");
    }
}

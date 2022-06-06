
#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use crate::{parser, vm::{Vm, Value}, compiler::compile};

    fn build_vm(source_code: &str) -> Vm {
        let ast = parser::parse_text(&source_code).unwrap();
        log::debug!("{:#?}", ast);
        compile(ast)
    }

    #[test]
    fn test_compares() {
        let code = r#"
            var x = 1;
            var y = 2;
            var b = x is y;
            var c = x not y;
            var d = x < y;
            var e = x > y;
            var f = x <= y;
            var g = x >= y;
            var h = x == y;
            var i = x != y;
            "#;

        let mut vm = build_vm(code);

        vm.work();

        let v = vm.get_variable("b");

        match v {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected boolean")
        };

        let v = vm.get_variable("c");

        match v {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected boolean")
        };

        let v = vm.get_variable("d");

        match v {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected boolean")
        };

        let v = vm.get_variable("e");

        match v {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected boolean")
        };

        let v = vm.get_variable("f");

        match v {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected boolean")
        };

        let v = vm.get_variable("g");

        match v {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected boolean")
        };

        let v = vm.get_variable("h");

        match v {
            Value::Boolean(b) => assert_eq!(b, false),
            _ => panic!("Expected boolean")
        };

        let v = vm.get_variable("i");

        match v {
            Value::Boolean(b) => assert_eq!(b, true),
            _ => panic!("Expected boolean")
        };
    }

    #[test]
    fn test_math_operations() {
        // env_logger::Builder::new()
        //     .filter_level(LevelFilter::Debug)
        //     .init();

        let code = r#"
        var a = 1 +2 * (6 / 3);
        "#;

        let mut vm = build_vm(code);

        vm.work();

        let v = vm.get_variable("a");

        match v {
            Value::Number(n) => assert_eq!(n, 5.0),
            _ => panic!("Expected number")
        };
    }

    #[test]
    fn test_while_loop() {
        let code = r#"
        var i = 0;
        while i < 10 begin
            print(i);
            set i = i + 1;
        end.
        "#;

        let mut vm = build_vm(code);

        let actions = vm.work();

        let v = vm.get_variable("i");

        match v {
            Value::Number(n) => assert_eq!(n, 10.0),
            _ => panic!("Expected number")
        };

        assert_eq!(actions.len(), 11);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser, vm::{Vm, Value}, compiler::compile};

    fn build_vm(source_code: &str) -> Vm {
        let ast = parser::parse_text(&source_code).unwrap();
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
}
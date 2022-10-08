#[macro_export]
macro_rules! param_test {
    ($test_name:ident ($($name:ident : $values:expr),* $(,)?) $test:block) => {
        #[test]
        fn $test_name() {
            $(
                let $name = $values;
            )*
            for ($($name),*) in cartesian!($($name.iter()),*) {
                (|| {
                    $test
                })()
            }
        }
    }
}

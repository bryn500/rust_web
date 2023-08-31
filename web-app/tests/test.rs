// write tests in the files themselves, integration tests if building a library can go here
#[cfg(test)]
pub mod unit_tests {
    struct TestData {
        name: String,
        is: bool,
        num: i32,
    }

    enum TestEnum {
        Test1(TestData),
        Test2(i32),
        Green,
    }

    #[test]
    fn test_enum1() {
        let test_data = TestEnum::Test1(TestData {
            is: false,
            name: String::from("test"),
            num: 1,
        });

        match_enum(test_data);

        assert_eq!(1, 1);
    }

    #[test]
    fn test_enum2() {
        let test_data = TestEnum::Test2(1);

        match_enum(test_data);

        assert_eq!(1, 1);
    }

    #[test]
    fn test_enum3() {
        let test_data = TestEnum::Green;

        match_enum(test_data);

        assert_eq!(1, 1);
    }

    fn match_enum(test_data: TestEnum) {
        match test_data {
            TestEnum::Test1(val) => match val.name.as_str() {
                "test" => {
                    println!("Property value is test");
                }
                _ => {
                    println!("Property value is other struct");
                }
            },
            TestEnum::Test2(val) => match val {
                1 => {
                    println!("Property value is 1");
                }
                _ => {
                    println!("Property value is other num");
                }
            },
            TestEnum::Green => {
                println!("Property value is green");
            }
        }
    }

    #[test]
    fn test_rust() {
        let mut tests = Vec::new();
        for i in 0..3 {
            tests.push(TestData {
                is: false,
                name: String::from("test"),
                num: (i as i32),
            });

            println!("{}", tests[i].num);
            println!("{}", tests[i].is);
            println!("{}", tests[i].name);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn test_markdown() {
        let html: String = markdown::to_html("__I am markdown__");
        assert_eq!(&html, "<p><strong>I am markdown</strong></p>\n");
    }
}

// write tests in the files themselves, integration tests if building a library can go here
#[cfg(test)]
pub mod unit_tests {
    struct TestData {
        name: String,
        is: bool,
        num: i32,
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

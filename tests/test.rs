// write tests in the files themselves, integration tests if building a library can go here
#[cfg(test)]
pub mod unit_tests {
    #[test]
    fn test_markdown() {
        let html: String = markdown::to_html("__I am markdown__");
        assert_eq!(&html, "<p><strong>I am markdown</strong></p>\n");
    }
}

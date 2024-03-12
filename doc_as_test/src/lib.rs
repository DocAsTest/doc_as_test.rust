extern crate backtrace;
use std::fs;
use std::path::Path;

pub struct DocAsTest {
    name: String,
    test_name: String,
    content: String,
    docs_path: String,
    docs_extension: String,
}

impl DocAsTest {
    pub fn new(name: &str, test_name: &str) -> DocAsTest {
        DocAsTest {
            name: name.to_string(),
            test_name: test_name.to_string(),
            content: String::new(),
            docs_path: "./docs".to_string(),
            docs_extension: "adoc".to_string(),
        }
    }

    pub fn write(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn content(&self) -> String {
        format!("= {}\n\n{}", self.name, self.content).to_string()
    }

    pub fn approve(&self) {
        let method_name = &self.test_name;
        let received_filename = format!(
            "{}/{}_received.{}",
            &self.docs_path, method_name, self.docs_extension
        );
        let approved_filename = format!(
            "{}/{}_approved.{}",
            self.docs_path, method_name, self.docs_extension
        );
        // println!("received_filename {}", received_filename);
        // println!("approved_filename {}", approved_filename);

        let expected_content = self
            .get_file_contents(&approved_filename)
            .trim_end_matches('\n')
            .to_string();
        let current_content = self.content().trim_end_matches('\n').to_string();

        if Path::new(&received_filename).exists() {
            if let Err(err) = fs::remove_file(&received_filename) {
                println!(
                    "Could not remove received file {}: {}",
                    &received_filename, err
                );
            }
        }

        if current_content == expected_content {
            return;
        }

        // println!("Expected: {}\n", expected_content);
        // println!("Current: {}\n", current_content);

        if let Err(err) = fs::write(received_filename, current_content) {
            panic!("Failed to write current file contents: {}", err);
        }
        // launch_diff(command, &method_name, approvals_dir); // TODO: Show how to diff yourself

        panic!("Strings are not identical");
    }

    fn get_file_contents(&self, filename: &str) -> String {
        if Path::new(filename).exists() {
            fs::read_to_string(filename)
                .map_err(|err| panic!("Failed to read file contents: {}", err))
                .unwrap()
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use doc_as_test_derive::doc_as_test;

    /// Using lib approvals
    /// https://github.com/aleksandrpak/approvals
    #[test]
    fn my_test_approvals_xxx() {
        let mut doc = format!("= {}\n\n", "Using Approvals");
        doc.push_str("xy");
        doc.push('z');

        approvals::approve(&doc);
    }

    /// Using DocAsTest
    #[test]
    fn my_test_approvals_with_doc_as_test() {
        let mut doc = DocAsTest::new("Using DocAsTest", "my_test_approvals_with_doc_as_test");
        doc.write("x");
        doc.write("y");
        doc.write("z");

        doc.approve();
    }

    // Using DocAsTest macro
    #[doc_as_test(Using DocAsTest macro)]
    fn my_test_approvals_with_macro() {
        doc.write("x");
        doc.write("y");
        doc.write("z");
    }
}

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

    pub fn title(&self) -> String {
        let mut c = self.name.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().replace('_', " "),
        }
    }

    pub fn content(&self) -> String {
        format!("= {}\n\n{}", self.title(), self.content).to_string()
    }

    pub fn approve(&self) {
        let received_filename = self.received_filename();
        let approved_filename = self.approved_filename();

        fs::create_dir_all(Path::new(&approved_filename).parent().unwrap()).unwrap();
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

        if let Err(err) = fs::write(&received_filename, &current_content) {
            panic!("Failed to write current file contents: {}", err);
        }

        let (message, line) = self.extract_diff(&expected_content, &current_content);
        panic!(
            "Expecting content of file {approved_filename}:{line} but got the one in {received_filename}:{line}\n{message}",
        );
    }

    fn received_filename(&self) -> String {
        let method_name = &self.test_name.replace("::", "/");
        format!(
            "{}/{}_received.{}",
            &self.docs_path, method_name, self.docs_extension
        )
    }

    fn approved_filename(&self) -> String {
        let method_name = &self.test_name.replace("::", "/");
        format!(
            "{}/{}_approved.{}",
            &self.docs_path, method_name, self.docs_extension
        )
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

    fn extract_diff(&self, left_content: &str, right_content: &str) -> (String, u32) {
        let mut line: u32 = 0;
        let mut left_lines = left_content.split('\n');
        let mut right_lines = right_content.split('\n');

        loop {
            match (left_lines.next(), right_lines.next()) {
                (Some(left), Some(right)) => {
                    line += 1;
                    if left != right {
                        return (
                            format!("Line {line}:\n    left : {left}\n    right: {right}"),
                            line,
                        );
                    };
                }
                (None, Some(_)) => {
                    return (
                        format!("Right is greater than left from line {}.", line + 1),
                        line,
                    );
                }
                (Some(_), None) => {
                    return (
                        format!("Left is greater than right from line {}.", line + 1),
                        line,
                    );
                }
                (None, None) => return (String::new(), line),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use doc_as_test_derive::doc_as_test;
    use toml::Table;

    /// Using DocAsTest
    #[test]
    fn basic_usage() {
        let mut doc = DocAsTest::new("Using DocAsTest", "doc_as_test::tests::basic_usage");
        doc.write("xyz");

        doc.approve();
    }
    // >>> minimal_example
    #[doc_as_test()]
    fn sample_doc_as_test_usage() {
        doc.write("xyz");
    }
    // <<< minimal_example

    // Using DocAsTest macro
    #[doc_as_test(title = "Using DocAsTest macro specifing a title")]
    fn using_macro_with_a_title() {
        doc.write("xyz");
    }

    #[doc_as_test()]
    fn usage() {
        doc.write(":source-highlighter: highlight.js\n");
        doc.write("\n");
        doc.write("This module allow you to create DocAsTest tests.\n");
        doc.write("\n");
        doc.write("Here a simple usage example:\n");

        let file_path = format!("./Cargo.toml");
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let value = contents.parse::<Table>().unwrap();
        let crate_name = &value["package"]
            .get("name")
            .map(|v| v.as_str().unwrap())
            .unwrap();
        let filename = file!().to_string().replace(&format!("{crate_name}/"), "");
        let code = extract_file_part(&filename, "minimal_example");
        doc.write("\n.Source code\n[source,rust,indent=0]\n----\n");
        doc.write(&code);
        doc.write("\n----\n");
        doc.write("\n");

        let approved_filename =
            DocAsTest::new("", "doc_as_test::tests::sample_doc_as_test_usage").approved_filename();
        println!(">>> {}", approved_filename);
        let approved_file_content = fs::read_to_string(&approved_filename).unwrap();
        doc.write(&format!(".Approved file ({approved_filename})\n"));
        doc.write("[source,asciidoc]\n----\n");
        doc.write(&approved_file_content);
        doc.write("\n----\n");
    }

    fn extract_file_part(filename: &str, tag: &str) -> String {
        let code = std::fs::read_to_string(filename).unwrap();
        let lines = code.split('\n');
        let mut lines = lines
            .into_iter()
            .skip_while(|line| !line.contains(&format!("// >>> {tag}")));
        lines.next();
        let mut code_part = String::new();
        while let Some(line) = lines.next() {
            if line.contains(&format!("// <<< {tag}")) {
                break;
            }

            code_part.push_str(&format!("{}\n", line));
        }

        code_part
    }

    #[doc_as_test]
    fn extract_diff() {
        let left = "ABC\nDEF";
        let right = "ABC\nGHI";
        let (diff, line) = doc.extract_diff(left, right);

        doc.write(&format!(".Left\n----\n{left}\n----\n"));
        doc.write("\n");
        doc.write(&format!(".Right\n----\n{right}\n----\n"));
        doc.write("\n");
        doc.write(&format!(".Diff\n----\n{diff}\n----\n"));
    }

    #[doc_as_test]
    fn extract_diff_when_one_is_longer() {
        let shortest = "ABC\nDEF";
        let longest = "ABC\nDEF\nGHI";

        doc.write(&format!(".Shortest\n----\n{shortest}\n----\n"));
        doc.write("\n");
        doc.write(&format!(".Longest\n----\n{longest}\n----\n"));
        doc.write("\n");
        let (diff, line) = doc.extract_diff(shortest, longest);
        doc.write(&format!(
            ".With right the longest one.\n----\n{diff}\n----\n"
        ));
        doc.write("\n");
        let (diff, line) = doc.extract_diff(longest, shortest);
        doc.write(&format!(
            ".With left the longest one.\n----\n{diff}\n----\n"
        ));
    }
}

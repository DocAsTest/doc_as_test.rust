mod file_system;
mod method_name;

#[cfg(test)]
mod tests;

use file_system::{get_dir_path, get_file_contents, write_actual};
use method_name::error::Error as GetMethodNameError;

pub fn approve(actual: &str) {
    let approvals_dir = "approvals"; // TODO: configure

    let method_name = extract_method_name(method_name::get(1)); // TODO: Make enum for get() parameter
    let dir_path = get_dir_path(approvals_dir);
    let contents = get_file_contents(&method_name, dir_path);

    if actual.trim_right_matches('\n') == contents.trim_right_matches('\n') {
        return;
    }

    write_actual(actual, &method_name, dir_path);
    // launch_diff(command, &method_name, approvals_dir); // TODO: Show how to diff yourself

    panic!("Strings are not identical");
}

fn extract_method_name(result: Result<String, GetMethodNameError>) -> String {
    match result {
        Ok(method_name) => method_name,
        Err(err) => match err {
            GetMethodNameError::NoSymbolName => panic!("Could not retrieve method name"),
            GetMethodNameError::NoMethodSeparator => panic!("Invalid format of method name"),
            GetMethodNameError::InvalidLevel => {
                panic!("Invalid level of searching for method name")
            }
            GetMethodNameError::Utf8(utf8_err) => {
                panic!("Could not decode UTF8 method name: {}", utf8_err)
            }
            GetMethodNameError::Demangle(fmt_err) => {
                panic!("Could not demangle method name: {}", fmt_err)
            }
        },
    }
}

// fn launch_diff(command: &str, method_name: &str, dir_name: &str) {
//     let received = format!("{}/{}_received.txt", dir_name, method_name);
//     let approved = format!("{}/{}_approved.txt", dir_name, method_name);

//     // TODO: replace match like this with unwrap_or_else
//     match Command::new(command).arg(received).arg(approved).status() {
//         Err(err) => panic!("Failed to launch diff tool: {}", err),
//         _ => {}
//     }
// }

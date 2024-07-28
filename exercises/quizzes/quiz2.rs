// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}
use std::collections::HashMap;

mod my_module {
    use super::Command;
    use super::HashMap;

    // TODO: Complete the function.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let hash: HashMap<String, Command> = HashMap::from_iter(input);
        let mut str_vec: Vec<String> = Vec::new();

        for (st, cmd) in &hash {
            let new_st = match cmd {
                Command::Uppercase => st.to_uppercase(),
                Command::Trim => st.trim().to_string(),
                Command::Append(n) => {
                    let mut st_str = st.clone(); 
                    st_str.push_str(&"bar".repeat(*n));
                    st_str.to_string()
                }
            };

            str_vec.push(new_st);
        }

        str_vec
    }
}
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}

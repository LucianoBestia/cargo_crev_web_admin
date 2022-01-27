//! automation_tasks_rs for cargo_crev_web_admin

use cargo_auto_lib::*;
// use unwrap::unwrap;

/// automation_tasks_rs with_lib
fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion. Take care to keep them in sync with the changes.

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build"{
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_docs();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
User defined automation tasks for `cargo_crev_web_admin`:
cargo auto build - builds the crate in debug mode, fmt
cargo auto release - builds the crate in release mode, version from date, fmt, strip
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
"#);
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc","test", "commit_and_push"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["with_lib"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion.

// region: tasks

fn task_build() {
    auto_version_from_date();
    run_shell_command("cargo fmt");
    run_shell_command("cargo build");
    println!(
        r#"After `cargo auto build`, run it
`target/debug/cargo_crev_web_admin -h`. If ok, then 
`cargo auto release`
"#);
}

fn task_release() {
    auto_version_from_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
    run_shell_command(&format!(
        "strip target/release/{package_name}",
        package_name = package_name()
    ));    
    println!(r#"After `cargo auto release`, run it
`target/release/cargo_crev_web_admin -h`. If ok, then 
`cargo auto doc`"#);
}

fn task_docs() {
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_md_to_doc_comments();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items --open",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    // message to help user with next move
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. If ok, then 
`cargo auto test`
"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
   
    println!(
        r#"
After `cargo auto test`, if ok, then 
`cargo auto commit_and_push "message"`
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("Error: message for commit is mandatory"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"
After `cargo auto commit_and_push "message"`
copy to the cargo_crev_web server
"#
            );
        }
    }
}

// endregion: tasks

// cargo_crev_web_admin lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_crev_web_admin
//!
//! **Admin CLI for cargo_crev_web**  
//! ***version: 2023.605.1106 date: 2023-06-05 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo_crev_web_admin/)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-850-green.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-119-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-103-purple.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-36-orange.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
//!
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/blob/main/LICENSE)
//! [![Rust](https://github.com/bestia-dev/cargo_crev_web_admin/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/cargo_crev_web_admin/)
//! ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/911031110.svg)
//!
//! Hashtags: #rustlang #buildtool #developmenttool #web #admin #cli  
//! My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## cargo_crev_web_admin CLI
//!
//! The web app cargo_crev_web on <https://web.crev.dev> fetches all proof repos it can find and shows the crate reviews online.  
//! Some admin tasks are needed and I don't want them to be accessible on the web.  
//! This will be a CLI app that can be used when logged on the linux terminal over SSH.  
//! So is sure that only an admin, who can log in on to the server, can use this tasks.
//!
//! Some tasks need the crev passphrase. Put it in the env variable before starting the CLI:  
//! `$  export CREV_PASSPHRASE=xxx`  
//! Add a space before the command to avoid to be saved in the bash history.  
//!
//! ## Development
//!
//! I use [cargo-auto](https://crates.io/crates/cargo-auto) for automation tasks in rust language. Install it:
//!
//! ```bash
//! cargo install cargo-auto
//! ```
//!
//! List user-defined automation tasks in `automation_tasks_rs`:
//!
//! ```bash
//! cargo auto
//! ```
//!
//! ## bash auto-completion
//!
//! This executable is prepared for auto-completion in bash.  
//! Run this command to define auto-completion in bash for the current session:  
//! Or add it to `.bashrc` file to be executed n every session start.
//!
//! ```bash
//! complete -C "cargo_crev_web_admin completion" cargo_crev_web_admin
//! ```
//!
//! To make it permanent add this command to the file `~/.bashrc` or some other file that runs commands on bash initialization.  
//!
//! ## Prepare development environment
//!
//! In the development environment inside a container I need the `cargo-crev` binary to run the commands. Fortunately there is a binary release already compiled:
//!
//! ```bash
//! curl -L -s https://github.com/crev-dev/cargo-crev/releases/download/v0.23.3/cargo-crev-v0.23.3-x86_64-unknown-linux-musl.tar.gz --output /tmp/cargo-crev.tar.gz
//! tar -xzv --no-same-owner --strip-components=1 -C ~/.cargo/bin -f /tmp/cargo-crev.tar.gz cargo-crev-v0.23.3-x86_64-unknown-linux-musl/cargo-crev
//! rm /tmp/cargo-crev.tar.gz
//! chmod +x ~/.cargo/bin/cargo-crev
//! git config --global core.editor "nano"
//! ```
//!
//! Now I need to import the `CrevId` from the server (ssh agent already has my ssh identity to connect to the server):  
//!
//! ```bash
//! scp luciano_bestia@bestia.dev:/home/luciano_bestia/.config/crev/ids/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU.yaml .
//! # Connecting standard input to the file with <
//! cargo-crev crev id import <UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU.yaml
//! cargo-crev crev id current
//! rm UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU.yaml
//! ```
//!
//! I need the ssh keys from the server to connect to the github remote repository.  
//!
//! ```bash
//! scp luciano_bestia@bestia.dev:/home/luciano_bestia/.ssh/web_crev_dev_for_github.pub ~/.ssh/
//! scp luciano_bestia@bestia.dev:/home/luciano_bestia/.ssh/web_crev_dev_for_github ~/.ssh/
//! # Be careful to not commit any secrets or private keys to github!
//! chmod 400 ~/.ssh/web_crev_dev_for_github
//! # Add the ssh key to your running ssh-agent
//! ssh-add ~/.ssh/web_crev_dev_for_github
//! # configure the remote repository
//! cargo-crev crev id set-url https://github.com/web-crev-dev/crev-proofs
//! # To test add a `dpc` as trusted
//! cargo-crev crev trust https://github.com/dpc/crev-proofs
//! # Now check the dir with
//! cargo-crev crev repo dir
//! ```
//!
//! I got: `~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w`
//! It looks crev changed the dir from ~/.config/crev to ~/.local/share/crev in some version. Be careful!
//!
//! On every session I will need to add the ssh key to the running ssh-agent:
//!
//! ```bash
//! ssh-add ~/.ssh/web_crev_dev_for_github
//! ```
//!
//! Copy the new crev data from the server for developing and debugging. The web.crev.dev has a special crev-id and should not interfere with other crev-ids on the system.  
//!
//! ```bash
//! rm ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/blocklisted_repos.json
//! scp luciano_bestia@bestia.dev:/var/www/webapps/cargo_crev_web/blocklisted_repos.json ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/blocklisted_repos.json
//! ls -l ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU
//!
//! rm -r ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust/
//!
//! scp -r luciano_bestia@bestia.dev:/home/luciano_bestia/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust/ ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/
//! ls -l ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/trust/
//! # list only the directly trusted repos
//! cargo-crev crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1
//! ```
//!
//! This should list around 80 directly trusted proof-repos that are used on the server.  
//!
//! ## TODO
//!
//! Integrity - warnings if a review have incorrect url or ID
//!
//! ## cargo-crev reviews and advisory
//!
//! Please, spread this info !\
//! Open source code needs a community effort to express trustworthiness.\
//! Start with reading the reviews of the crates on [web.crev.dev](https://web.crev.dev/rust-reviews/crates). \
//! Then install the GUI [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) or the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\.  
//! Your personal reviews are most important. If you have a boss, he will sooner or later ask you to show him your reviews for all the dependencies you use. With [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) you have a basic tool to do that. \
//! Write your reviews! Describe the crates you trust and why. Or warn about the crate versions you think are dangerous. Publish and share your opinion with other developers.\
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod blocklisted_repos_mod;
mod find_repos_with_reviews_mod;
mod list_new_repos_mod;
mod my_trusted_repos_mod;
mod utils_mod;

// re-export
pub use find_repos_with_reviews_mod::find_repos_with_reviews_on_github;
pub use list_new_repos_mod::list_new_repos;
pub use utils_mod::*;

// use unwrap::unwrap;
use crate::{blocklisted_repos_mod::BlocklistedRepos, my_trusted_repos_mod::MyTrustedRepos};
use lazy_static::lazy_static;

lazy_static! {
    // The Linux home folder ~ or /home/username
    pub static ref HOME_DIR:std::path::PathBuf = home::home_dir().unwrap();
    pub static ref CREV_REMOTES_DIR: std::path::PathBuf = HOME_DIR.join(".cache/crev/remotes");
}

/// list the explicit trusted reviewers from cargo-crev command
pub fn trusted_from_crev_command() {
    println!("List of explicit trusted reviewers from the cargo-crev command");
    println!("Warning: It shows also implicitly myself as high trust.");
    println!(
        "$ cargo-crev crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1"
    );
    println!("");

    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.list_from_crev_command();

    let line_count = count_newlines(&output);
    println!("{output}\nLine count: {line_count}");
}

/// list the explicit trusted reviewers from the /trust/*.crev files
pub fn trusted_list() {
    println!("List of explicit trusted reviewers from the /trust/*.crev files");
    println!("");

    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.list_from_files();

    let line_count = count_newlines(&output);
    println!("{output}\nLine count: {line_count}");
}

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

/// delete fetched repos from /remote/ if they are not in trusted_list
pub fn delete_untrusted_repos() {
    println!("Delete fetched repos from /remote/ if they are not in trusted_list.");
    let mut output = String::new();
    let my_trusted_repos = MyTrustedRepos::new();
    let trusted_list = my_trusted_repos.list_from_files();

    for entry in CREV_REMOTES_DIR.read_dir().unwrap() {
        let entry = entry.unwrap();
        let entry_name = entry.file_name();
        let entry_name = entry_name.to_string_lossy();
        let mut is_found = false;
        for trusted_url in trusted_list.lines() {
            let trusted_name = trusted_url
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_end_matches(".git")
                .replace("/", "_")
                .replace(".", "_")
                .replace("~", "_");
            if entry_name
                .to_lowercase()
                .starts_with(&trusted_name.to_lowercase())
            {
                is_found = true;
            }
        }
        if is_found == false {
            output.push_str(&format!("rm -rf {:#?}\n", &entry.path()));
        }
    }
    if !output.is_empty() {
        println!("Run these commands manually in bash:\n{output}");
    }

    println!("delete_untrusted_repos finished.");
}

/// fetch the explicit trusted reviewers from the /trust/*.crev files
pub fn fetch() {
    println!("Fetch the explicit trusted reviewers from the /trust/*.crev files");
    println!(
        "Warning: It will try to fetch also `myself`, but the local folder is deleted on purpose."
    );
    println!(
        "$ cargo-crev crev repo fetch trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1"
    );
    println!("");

    let output = std::process::Command::new("cargo-crev")
        .args([
            "crev",
            "repo",
            "fetch",
            "trusted",
            "--high-cost",
            "1",
            "--medium-cost",
            "1",
            "--low-cost",
            "1",
            "--depth",
            "1",
        ])
        .output()
        .unwrap();
    let output = format!(
        "{} {}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    );
    let line_count = count_newlines(&output);
    println!("{output}\nLine count: {line_count}");
}

/// add new trusted repo
pub fn trusted_add(repo_url: &str) {
    println!("Add a trusted repo url.");
    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.trusted_add(repo_url);

    println!("{output}");
}

/// delete from trusted repo
pub fn trusted_delete(repo_url: &str) {
    println!("Delete from trusted repo.");
    let my_trusted_repos = MyTrustedRepos::new();
    my_trusted_repos.trusted_delete(repo_url);
}

/// web app reads and reindex new or changed data
pub fn reindex() {
    println!("Web app reads and reindex new or changed data.");
    // curl --silent https://bestia.dev/rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/
    let _output = std::process::Command::new("curl")
        .arg("--silent")
        .arg("https://bestia.dev/rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/")
        .output()
        .unwrap();
    println!("Reindex finished.");
}

/// after changing trust files it is mandatory to publish this repo
pub fn publish_to_github() {
    println!("After changing trust files it is mandatory to publish this repo.");
    println!("Because crev uses the fetched files from remotes only, not the local copy, even for my repo.");
    let output = std::process::Command::new("cargo-crev")
        .arg("crev")
        .arg("publish")
        .output()
        .unwrap();
    let output = format!(
        "{} {}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    );
    println!("{output}");
    if output.contains("git@github.com: Permission denied (publickey).") {
        println!("If you don't have permission to write to github, then we need to run ssh-agent and ssh-add:");
        println!("$ eval `ssh-agent`; ssh-add ~/.ssh/bestia2_for_github");
        println!("Enter your ssh passphrase for github and finally repeat:");
        println!("$ cargo_crev_web_admin publish");
    }
}

/// list of blocklisted
pub fn blocklisted_list() {
    println!("List of blocklisted");
    println!("");

    let bl = BlocklistedRepos::read_from_default_file();
    let mut output = String::new();
    for x in bl.list().iter() {
        output.push_str(&x.0);
        output.push_str("      ");
        output.push_str(&x.1);
        output.push('\n');
    }
    let line_count = count_newlines(&output);
    println!("{output}\nLine count: {line_count}");
}

/// add new blocklist repo
pub fn blocklisted_add(repo_url: &str, note: &str) {
    println!("Add blocklisted repo url.");
    let mut bl = BlocklistedRepos::read_from_default_file();
    bl.add(repo_url, note);
    bl.write();
    println!("Added to blocklist.");
}

/// delete from blocklist repo
pub fn blocklisted_delete(repo_url: &str) {
    println!("Delete from blocklisted repo.");
    let mut bl = BlocklistedRepos::read_from_default_file();
    bl.delete(repo_url);
    bl.write();
    println!("Deleted from blocklist.");
}

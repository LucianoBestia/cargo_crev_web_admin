// my_trusted_repos_mod.rs

// public object/interface to work with `my trusted repos`
// and avoid thinking about folders, crev files, proofs and yaml

mod crev_file_mod;
mod crev_proof_mod;
mod my_trust_crev_files_mod;
mod trust_yaml_proofs_mod;

pub struct MyTrustedRepos {
    my_trust_crev_files: my_trust_crev_files_mod::MyTrustCrevFiles,
}

impl MyTrustedRepos {
    // constructor
    pub fn new() -> MyTrustedRepos {
        MyTrustedRepos {
            my_trust_crev_files: my_trust_crev_files_mod::MyTrustCrevFiles::new(),
        }
    }
    // delete trusted repo
    pub fn delete_trust(&self, repo_url: &str) {
        for mut crev_file in self.my_trust_crev_files.iter_my_trust_crev_file() {
            crev_file.delete_url(repo_url).unwrap();
        }
    }
    // add trusted repo
    pub fn add_trust(&self, repo_url: &str) -> String {
        // if it already exists, delete the old one, because `cargo crev trust` only adds new even for same repo_url
        self.delete_trust(repo_url);
        let output = std::process::Command::new("cargo")
            .args(["crev", "trust", "--level", "low"])
            .arg(repo_url)
            .output()
            .unwrap();
        let output = format!(
            "{} {}",
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        );
        // return
        output
    }
    pub fn list_from_files(&self) -> String {
        let output = String::new();

        //return
        output
    }
    pub fn list_from_crev_command(&self) -> String {
        let output = std::process::Command::new("cargo")
            .args([
                "crev",
                "id",
                "query",
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
        output
    }
}

use std::path::PathBuf;

fn main() {
    let Some(proj) = directories::ProjectDirs::from("", "", "Firefox") else {return; };

    let cookies = proj.data_dir().join(
        ["Profiles", "2cv69g10.default-release", "cookies.sqlite"]
            .iter()
            .collect::<PathBuf>(),
    );

    if cookies.is_file() {
        println!("We found the cookies, at {}", cookies.display());
    }
}

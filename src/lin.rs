use std::env;
use std::path::PathBuf;
use std::process::Command;

use BaseDirectories;
use ProjectDirectories;
use strip_qualification;

pub fn base_directories() -> BaseDirectories {
    let home_dir         = env::home_dir().unwrap();
    let cache_dir        = env::var("XDG_CACHE_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".cache"));
    let config_dir       = env::var("XDG_CONFIG_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".config"));
    let data_dir         = env::var("XDG_DATA_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".local/share"));
    let data_roaming_dir = data_dir.clone();
    let runtime_dir      = env::var("XDG_RUNTIME_DIR").ok().and_then(is_absolute_path);
    let executables_dir  = { let mut new_dir = data_dir.clone(); new_dir.pop(); new_dir.push("bin"); new_dir };
    let fonts_dir        = data_dir.join("fonts");

    BaseDirectories {
        home_dir:         home_dir,
        cache_dir:        cache_dir,
        config_dir:       config_dir,
        data_dir:         data_dir,
        data_roaming_dir: data_roaming_dir,
        runtime_dir:      runtime_dir,
        desktop_dir:      run_xdg_user_dir_command("DESKTOP"),
        documents_dir:    run_xdg_user_dir_command("DOCUMENTS"),
        download_dir:     run_xdg_user_dir_command("DOWNLOAD"),
        music_dir:        run_xdg_user_dir_command("MUSIC"),
        pictures_dir:     run_xdg_user_dir_command("PICTURES"),
        public_dir:       run_xdg_user_dir_command("PUBLICSHARE"),
        templates_dir:    Some(run_xdg_user_dir_command("TEMPLATES")),
        videos_dir:       run_xdg_user_dir_command("VIDEOS"),
        executables_dir:  Some(executables_dir),
        fonts_dir:        Some(fonts_dir)
    }
}

impl ProjectDirectories {
    pub fn from_unprocessed_string(value: &str) -> ProjectDirectories {
        let project_name = String::from(value);
        let home_dir                 = env::home_dir().unwrap();
        let project_cache_dir        = env::var("XDG_CACHE_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".cache")).join(&value);
        let project_config_dir       = env::var("XDG_CONFIG_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".config")).join(&value);
        let project_data_dir         = env::var("XDG_DATA_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".local/share")).join(&value);
        let project_roaming_data_dir = project_data_dir.clone();
        let project_runtime_dir      = env::var("XDG_RUNTIME_DIR").ok().and_then(is_absolute_path).unwrap().join(&value);

        ProjectDirectories {
            project_name:             project_name,
            project_cache_dir:        project_cache_dir,
            project_config_dir:       project_config_dir,
            project_data_dir:         project_data_dir,
            project_data_roaming_dir: project_roaming_data_dir,
            project_runtime_dir:      Some(project_runtime_dir)
        }
    }

    pub fn from_qualified_project_name(qualified_project_name: &str) -> ProjectDirectories {
        let name = strip_qualification(qualified_project_name).to_lowercase();
        ProjectDirectories::from_unprocessed_string(name.trim())
    }

    pub fn from_project_name(project_name: &str) -> ProjectDirectories {
        let name = trim_and_replace_spaces_with_hyphens_then_lowercase(project_name);
        ProjectDirectories::from_unprocessed_string(&name)
    }
}

fn is_absolute_path(path: String) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}

fn run_xdg_user_dir_command(arg: &str) -> PathBuf {
    let mut out  = Command::new("xdg-user-dir").arg(arg).output().expect("failed to execute process").stdout;
    let out_len = out.len();
    out.truncate(out_len - 1);
    PathBuf::from(String::from_utf8(out).unwrap())
}

fn trim_and_replace_spaces_with_hyphens_then_lowercase(name: &str) -> String {
    let mut buf = String::with_capacity(name.len());
    let mut parts = name.split_whitespace();
    let mut current_part = parts.next();
    while current_part.is_some() {
        let value = current_part.unwrap().to_lowercase();
        buf.push_str(&value);
        current_part = parts.next();
        if current_part.is_some() {
            buf.push('-');
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use lin::trim_and_replace_spaces_with_hyphens_then_lowercase;

    #[test]
    fn test_trim_and_replace_spaces_with_hyphens_then_lowercase() {
        let input1    = "Bar App";
        let actual1   = trim_and_replace_spaces_with_hyphens_then_lowercase(input1);
        let expected1 = "bar-app";
        assert_eq!(expected1, actual1);

        let input2    = "BarApp-Foo";
        let actual2   = trim_and_replace_spaces_with_hyphens_then_lowercase(input2);
        let expected2 = "barapp-foo";
        assert_eq!(expected2, actual2);

        let input3    = " Bar App ";
        let actual3   = trim_and_replace_spaces_with_hyphens_then_lowercase(input3);
        let expected3 = "bar-app";
        assert_eq!(expected3, actual3);

        let input4    = "  Bar  App  ";
        let actual4   = trim_and_replace_spaces_with_hyphens_then_lowercase(input4);
        let expected4 = "bar-app";
        assert_eq!(expected4, actual4);
    }
}

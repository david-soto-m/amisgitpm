use agpm_interactions::Interactor;
use agpm_pm::PrjManager;
use agpm_project::{Project, UpdatePolicy};
use agpm_store::Store;
use amisgitpm::Directories;
use std::io::Read;
use std::path::{Path, PathBuf};
use subprocess::Exec;
use thiserror::Error;

type TestInteracts = Interactor<TestDirs>;
type TestProjectsStore = Store<TestDirs, Project>;
type TestProjectManager = PrjManager<Project, TestDirs, TestProjectsStore, TestInteracts>;

#[derive(Debug, Error)]
enum EmptyError {}

struct TestDirs {}
impl Directories for TestDirs {
    type Error = EmptyError;
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
    fn projects_db(&self) -> PathBuf {
        Path::new("../test_sandbox/config/projects").to_path_buf()
    }
    fn src(&self) -> PathBuf {
        Path::new("../test_sandbox/cache/src").to_path_buf()
    }
    fn git(&self) -> PathBuf {
        Path::new("../test_sandbox/cache/git").to_path_buf()
    }
    fn old(&self) -> PathBuf {
        Path::new("../test_sandbox/cache/old").to_path_buf()
    }
}

/// This is needed because the interactions crate is imported with the feature
/// suggestions
impl agpm_suggestions::SuggestionsDirs for TestDirs {
    fn suggestions(&self) -> PathBuf {
        Path::new("../test_sandbox/config/suggestions").to_path_buf()
    }
}

impl TestDirs {
    fn bin(&self) -> PathBuf {
        Path::new("../test_sandbox/bin").to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use agpm_pm::PMError;
    use amisgitpm::{PMOperations, PMProgrammatic};

    #[test]
    fn install_uninstall_project() {
        let mut pm = TestProjectManager::new().unwrap();
        pm.get_dirs().projects_db();
        let prj = Project {
            name: "Hello-crate".into(),
            dir: "Hello-crate".into(),
            url: "https://github.com/zwang20/rust-hello-world.git".into(),
            ref_string: "refs/heads/master".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ../../..".into()],
            uninstall_script: vec!["cargo uninstall --root ../../..".into()],
        };
        pm.install(prj.clone()).unwrap();
        assert!(TestDirs::new()
            .unwrap()
            .bin()
            .join("rust-hello-world")
            .exists());
        assert!(
            if let Err(PMError::Common(amisgitpm::CommonPMErrors::AlreadyExisting)) =
                pm.install(prj.clone())
            {
                true
            } else {
                false
            }
        );
        pm.uninstall(&prj.name).unwrap();
    }

    #[test]
    fn updates() {
        let dir = std::fs::canonicalize(Path::new("./projects/git_upd2")).unwrap();
        assert_eq!(
            Exec::shell("bash 0_start.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        let mut url: String = "file://".into();
        url.push_str(&dir.to_str().unwrap());
        let prj = Project {
            name: "git_upd2".into(),
            dir: "git_upd2".into(),
            url,
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec![],
            uninstall_script: vec![],
        };
        let mut pm = TestProjectManager::new().unwrap();
        pm.install(prj).unwrap();
        let mut epoch = String::new();
        std::fs::File::open(dir.join("dates.txt"))
            .unwrap()
            .read_to_string(&mut epoch)
            .unwrap();
        let epoch = epoch.trim().parse::<i64>().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(
            Exec::shell("bash 1_update.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        let mut epoch2 = String::new();
        std::fs::File::open(dir.join("dates.txt"))
            .unwrap()
            .read_to_string(&mut epoch2)
            .unwrap();
        let epoch2 = epoch2.trim().parse::<i64>().unwrap();
        assert!(epoch2 > epoch);
        pm.update("git_upd2").unwrap();
        let mut epoch2 = String::new();
        std::fs::File::open(
            TestDirs::new()
                .unwrap()
                .src()
                .join("git_upd2")
                .join("dates.txt"),
        )
        .unwrap()
        .read_to_string(&mut epoch2)
        .unwrap();
        let epoch2 = epoch2.trim().parse::<i64>().unwrap();
        assert!(epoch2 > epoch);
        assert_eq!(
            Exec::shell("bash 2_finish.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        pm.uninstall("git_upd2").unwrap();
    }

    #[test]
    fn update_downgrade() {
        let dir = std::fs::canonicalize(Path::new("./projects/git_upd")).unwrap();
        assert_eq!(
            Exec::shell("bash 0_start.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        let mut url: String = "file://".into();
        url.push_str(&dir.to_str().unwrap());
        let prj = Project {
            name: "git_upd".into(),
            dir: "git_upd".into(),
            url,
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec![],
            uninstall_script: vec![],
        };
        let mut pm = TestProjectManager::new().unwrap();
        pm.install(prj).unwrap();
        let mut epoch = String::new();
        std::fs::File::open(dir.join("dates.txt"))
            .unwrap()
            .read_to_string(&mut epoch)
            .unwrap();
        let epoch = epoch.trim().parse::<i64>().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(
            Exec::shell("bash 1_update.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        let mut epoch2 = String::new();
        std::fs::File::open(dir.join("dates.txt"))
            .unwrap()
            .read_to_string(&mut epoch2)
            .unwrap();
        let epoch2 = epoch2.trim().parse::<i64>().unwrap();
        assert!(epoch2 > epoch);
        pm.update("git_upd").unwrap();
        let mut epoch2 = String::new();
        std::fs::File::open(
            TestDirs::new()
                .unwrap()
                .src()
                .join("git_upd")
                .join("dates.txt"),
        )
        .unwrap()
        .read_to_string(&mut epoch2)
        .unwrap();
        let epoch2 = epoch2.trim().parse::<i64>().unwrap();
        assert!(epoch2 > epoch);
        let mut epoch3 = String::new();
        pm.restore("git_upd").unwrap();
        std::fs::File::open(
            TestDirs::new()
                .unwrap()
                .src()
                .join("git_upd")
                .join("dates.txt"),
        )
        .unwrap()
        .read_to_string(&mut epoch3)
        .unwrap();
        let epoch3 = epoch3.trim().parse::<i64>().unwrap();
        assert!(epoch3 < epoch2);
        assert_eq!(epoch3, epoch);
        assert_eq!(
            Exec::shell("bash 2_finish.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        pm.uninstall("git_upd").unwrap();
    }

    #[test]
    fn get_one_get_many_edit() {
        let dir = std::fs::canonicalize(Path::new("./projects/setable")).unwrap();
        assert_eq!(
            Exec::shell("bash 0_start.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
        let mut url: String = "file://".into();
        url.push_str(&dir.to_str().unwrap());
        let mut prj = Project {
            name: "a".into(),
            dir: "a".into(),
            url,
            ref_string: "refs/heads/main".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec![],
            uninstall_script: vec![],
        };
        let mut pm = TestProjectManager::new().unwrap();
        pm.install(prj.clone()).unwrap();
        prj.name = "b".into();
        prj.dir = "b".into();
        pm.install(prj.clone()).unwrap();
        prj.name = "c".into();
        prj.dir = "c".into();
        pm.install(prj.clone()).unwrap();
        pm.get_one("a").unwrap();
        pm.get_one("b").unwrap();
        pm.get_one("c").unwrap();
        assert!(pm.get_all().len() >= 3); // because they are in parallel, we can only be sure that is greater than three
        prj.name = "d".into();
        prj.dir = "a".into();
        pm.edit("a", prj).unwrap();
        pm.get_one("d").unwrap();
        pm.uninstall("d").unwrap();
        pm.uninstall("b").unwrap();
        pm.uninstall("c").unwrap();
        assert_eq!(
            Exec::shell("bash 2_finish.sh")
                .cwd(&dir)
                .join()
                .unwrap()
                .success(),
            true
        );
    }
}

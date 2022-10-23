use crate::{dirutils::PMDirsImpl, interaction::Interactor, projects::ProjectTable};
mod error;
pub use error::PMError;

pub struct PackageManagerDefault {}

use amisgitpm_types_traits::*;

impl PMOperations for PackageManagerDefault {
    type Dirs = PMDirsImpl;
    type Error = PMError;
    fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
impl PMBasics for PackageManagerDefault {
    type Store = ProjectTable;
    type ErrorC = PMError;
}
impl PMExtended for PackageManagerDefault {}
impl PMInteractive for PackageManagerDefault {
    type Interact = Interactor;
    type ErrorI = PMError;
}

#[cfg(test)]
mod tests {
    use crate::package_management::{
        PMError, PackageManagerDefault,
    };
    use amisgitpm_types_traits::*;
    use std::{fs::canonicalize, io::prelude::*, path::PathBuf};
    use subprocess::Exec;
    #[test]
    fn install_uninstall_project() {
        let pm = PackageManagerDefault {};
        let prj = Project {
            name: "Hello-crate".into(),
            dir: "Hello-crate".into(),
            url: "https://github.com/zwang20/rust-hello-world.git".into(),
            ref_string: "refs/heads/master".into(),
            update_policy: UpdatePolicy::Always,
            install_script: vec!["cargo install --path . --root ~/.local".into()],
            uninstall_script: vec!["cargo uninstall --root ~/.local".into()],
        };
        pm.install(&prj).unwrap();
        assert!(directories::BaseDirs::new()
            .unwrap()
            .home_dir()
            .join(".local/bin/rust-hello-world")
            .exists());
        assert!(
            if let Err(PMError::Commons(CommonError::AlreadyExisting)) = pm.install(&prj) {
                true
            } else {
                false
            }
        );
        pm.uninstall(&prj.name).unwrap();
    }
    #[test]
    fn updates() {
        let dir = canonicalize(PathBuf::from(".").join("tests/projects/git_upd")).unwrap();
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
        let pm = PackageManagerDefault::new().unwrap();
        let a = <PackageManagerDefault as PMOperations>::Dirs::new().unwrap();
        pm.install(&prj).unwrap();
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
        std::fs::File::open(a.src_dirs().join("git_upd").join("dates.txt"))
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
        pm.uninstall("git_upd").unwrap();
    }
}

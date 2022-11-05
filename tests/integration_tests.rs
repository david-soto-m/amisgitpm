// use crate::{
//         dirutils::PMDirs,
//         package_management::{
//             CommonError, PMError, PackageManagementBase, PackageManagementCore,
//             PackageManagerDefault,
//         },
//         projects::{Project, UpdatePolicy},
//     };
//
// use std::{fs::canonicalize, io::prelude::*, path::PathBuf};
// use subprocess::Exec;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    // use crate::origins::SuggestionsTable;
    // use std::path::{Path, PathBuf};
    // #[test]
    // fn makes_suggestions() {
    //     let table = SuggestionsTable::new().unwrap();
    //     let len = table
    //         .get_suggestions(&Path::new("tests/projects/mess_project"))
    //         .len();
    //
    //     assert_eq!(len, 3);
    // }
    // #[test]
    // fn all_build_aux_json_is_correct() {
    //     SuggestionsTable::new().unwrap();
    //     assert!(true)
    // }
    // #[test]
    // fn gets_different_sections() {
    //     let hx = super::get_build_suggestions(&PathBuf::from("tests/mdowns/Helix.md")).unwrap();
    //     assert_eq!(hx[0].len(), 48);
    //     let swave =
    //         super::get_build_suggestions(&PathBuf::from("tests/mdowns/Shortwave.md")).unwrap();
    //     assert_eq!(swave.len(), 2);
    //     assert_eq!(swave[0].len(), 10);
    //     assert_eq!(swave[1].len(), 26);
    // }

    // #[test]
    // fn install_uninstall_project() {
    //     let pm = PackageManagerDefault {};
    //     let prj = Project {
    //         name: "Hello-crate".into(),
    //         dir: "Hello-crate".into(),
    //         url: "https://github.com/zwang20/rust-hello-world.git".into(),
    //         ref_string: "refs/heads/master".into(),
    //         update_policy: UpdatePolicy::Always,
    //         install_script: vec!["cargo install --path . --root ~/.local".into()],
    //         uninstall_script: vec!["cargo uninstall --root ~/.local".into()],
    //     };
    //     pm.install(&prj).unwrap();
    //     assert!(directories::BaseDirs::new()
    //         .unwrap()
    //         .home_dir()
    //         .join(".local/bin/rust-hello-world")
    //         .exists());
    //     assert!(
    //         if let Err(PMError::Commons(CommonError::AlreadyExisting)) = pm.install(&prj) {
    //             true
    //         } else {
    //             false
    //         }
    //     );
    //     pm.uninstall(&prj.name).unwrap();
    // }
    // #[test]
    // fn updates() {
    //     let dir = canonicalize(PathBuf::from(".").join("tests/projects/git_upd")).unwrap();
    //     assert_eq!(
    //         Exec::shell("bash 0_start.sh")
    //             .cwd(&dir)
    //             .join()
    //             .unwrap()
    //             .success(),
    //         true
    //     );
    //     let mut url: String = "file://".into();
    //     url.push_str(&dir.to_str().unwrap());
    //     let prj = Project {
    //         name: "git_upd".into(),
    //         dir: "git_upd".into(),
    //         url,
    //         ref_string: "refs/heads/main".into(),
    //         update_policy: UpdatePolicy::Always,
    //         install_script: vec![],
    //         uninstall_script: vec![],
    //     };
    //     let pm = PackageManagerDefault::new().unwrap();
    //     let a = <PackageManagerDefault as PackageManagementBase>::Dirs::new();
    //     pm.install(&prj).unwrap();
    //     let mut epoch = String::new();
    //     std::fs::File::open(dir.join("dates.txt"))
    //         .unwrap()
    //         .read_to_string(&mut epoch)
    //         .unwrap();
    //     let epoch = epoch.trim().parse::<i64>().unwrap();
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    //     assert_eq!(
    //         Exec::shell("bash 1_update.sh")
    //             .cwd(&dir)
    //             .join()
    //             .unwrap()
    //             .success(),
    //         true
    //     );
    //     let mut epoch2 = String::new();
    //     std::fs::File::open(dir.join("dates.txt"))
    //         .unwrap()
    //         .read_to_string(&mut epoch2)
    //         .unwrap();
    //     let epoch2 = epoch2.trim().parse::<i64>().unwrap();
    //     assert!(epoch2 > epoch);
    //     pm.update("git_upd").unwrap();
    //     let mut epoch2 = String::new();
    //     std::fs::File::open(a.src_dirs().join("git_upd").join("dates.txt"))
    //         .unwrap()
    //         .read_to_string(&mut epoch2)
    //         .unwrap();
    //     let epoch2 = epoch2.trim().parse::<i64>().unwrap();
    //     assert!(epoch2 > epoch);
    //     assert_eq!(
    //         Exec::shell("bash 2_finish.sh")
    //             .cwd(&dir)
    //             .join()
    //             .unwrap()
    //             .success(),
    //         true
    //     );
    //     pm.uninstall("git_upd").unwrap();
    // }
}
}

use crate::dbmanager::Project;
use git2::Repository;

pub fn config_repo(_repo: Repository) -> Project {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

//! This couple of traits define how projects are stored and interacted with by
//! project managers.

/// A trait that is used to know to set up a project
pub trait ProjectIface: Clone {
    /// Get the name of the project
    fn get_name(&self) -> &str;
    /// Get the directory name to use for this project
    fn get_dir(&self) -> &str;
    /// Get the url from which to clone the project
    fn get_url(&self) -> &str;
    /// Get the reference string
    fn get_ref_string(&self) -> &str;
    /// Get the install script
    fn get_install(&self) -> &[String];
    /// Get the uninstall script
    fn get_uninstall(&self) -> &[String];
}

/// How to interact with however your projects are stored
/// The idea is that you can implement this trait with any technology you want
/// to use. Any kind of database, a xml document, a collection of json docs...
pub trait ProjectStore<T: ProjectIface>
where
    Self: Sized,
{
    /// Your custom Error type
    type Error: std::error::Error;
    /// A function to start up your store
    fn new() -> Result<Self, Self::Error>;
    /// Add an item to the store
    fn add(&mut self, prj: T) -> Result<(), Self::Error>;
    /// Remove an item from the store
    fn remove(&mut self, prj_name: &str) -> Result<(), Self::Error>;
    /// Get a reference to an item inside the store
    fn get_ref<'a>(&'a self, prj_name: &str) -> Option<&'a T>;
    /// Return a cloned instance of a project in the store
    fn get_clone(&self, prj_name: &str) -> Option<T>;
    /// Replace the project that used to go by the `old_prj_name` name with the `new_prj` item
    /// # Errors
    /// This function will return an error when there is a problem removing an item or adding a
    /// new item
    fn edit(&mut self, old_prj_name: &str, new_prj: T) -> Result<(), Self::Error> {
        self.remove(old_prj_name)?;
        self.add(new_prj)?;
        Ok(())
    }
    /// If a directory name is free for use
    fn check_dir_free(&self, dir: &str) -> bool;
    /// If a name is free for use
    fn check_name_free(&self, prj_name: &str) -> bool;
    /// check if a combination of directory and name are both free for use
    fn check_unique(&self, prj_name: &str, dir: &str) -> bool {
        self.check_dir_free(dir) && self.check_name_free(prj_name)
    }
    /// Return an iterator over refereneces of Project Items
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a>;
    /// Check if there are elements in the store
    fn is_empty(&self) -> bool;
}

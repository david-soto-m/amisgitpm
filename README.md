# Why

There are several similar tools to this one. Depending on your use case you
might want to check them out instead of this crate

- [grab](https://github.com/jmhodges/grab)
- [git-grab](https://crates.io/crates/git-grab)
- [gitpm](https://github.com/lukluk/gitpm)
- [git-repo-updater](https://github.com/earwig/git-repo-updater)
- [huber](https://github.com/innobead/huber)
- [topgrade](https://github.com/r-darwish/topgrade)

However, I have published this crate. My reasons are:

1. I wanted to know what I had installed from git.
1. I wanted to use latest versions of the software.
1. I wanted to have some assistance with the build process.
1. I wanted if possible to automatically rebuild and install the latest updates.
1. I wanted to uninstall the software.
1. I wanted to be able to always go back to the previous version


# Known bugs

- You can only install via https clones and not ssh. Given that you
shouldn't really be interacting with the repos this shouldn't be a mayor
problem, but it's a bummer


# TODO

- Document, Document, and Document
- Test everything
- Test manual operations
- make suggestions optional?

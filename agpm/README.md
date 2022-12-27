# agpm

This is an executable complete implementation of an [`amisgitpm`](amisgitpm)
project manager. It uses the pieces from:

- [`agpm_dirs`](agpm_dirs)
- [`agpm_project`](agpm_project)
- [`agpm_store`](agpm_store)
- [`agpm_pm`](agpm_pm)
    - [`agpm_interactions`](agpm_interactions)
        - [`agpm_suggestions`](agpm_suggestions)

It admits the following commands

Usage: agpm \<COMMAND\>

|Command | Description|
|-|-|
|install             |Install a new git repo. It installs from URLs of two kinds                             |
|update              |Update project(s)                                                                      |
|update-suggestions  |Update the suggestions, downloading all of them, and substituting those already present|
|uninstall           |Uninstall a project                                                                    |
|restore             |Get the last version of the project                                                    |
|reinstall           |Uninstall then install a project                                                       |
|rebuild             |Run the build instructions of a project                                                |
|clean               |Remove all srcs with no project associated                                             |
|edit                |Edit the configuration of a project                                                    |
|list                |Show the list of installed applications and their version                              |
|bootstrap           |Install amisgitpm with amisgitpm, check that everything is in place                    |
|help                |Print this message or the help of the given subcommand(s)                              |
|  -h, --help        |Print help information                                                                 |
|  -V, --version     |Print version information                                                              |

A good way to interact with this package manager programatically is to use the
types provided in the library part of this crate

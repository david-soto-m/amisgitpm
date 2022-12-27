# agpm_suggestions

This crate provides a 2 functions, and a trait

The trait `SuggestionsDirs` is an 'add on' to the `Directories` trait, adding
one function to get the directory where the suggestions are supposed to be stored

Then it provides a function to download the suggestions and another to get them
for a given directory

In said directory it explores conformity to known structures (such as having a
Makefile or a meson.build files), and the information available in different
*.md.

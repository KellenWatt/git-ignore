# git-ignore

One of no doubt many plugins for git that handle the .gitignore file.

### Installation

To install, run

```sh
# The --release is optional, but makes for a slimmer binary
$ cargo build --release
``` 

then make sure the resulting binary is in your `PATH`, making sure it's 
called `git-<something>` (`git-ignore` by default).

### Usage

Simply run 

```sh
git ignore <patterns>
```

in your git repository of choice, and it will add those patterns to your 
.gitignore file, creating one for that repository if it needs to. Patterns that 
are duplicates of ones already in an existing .gitignore will be skipped, but
all patterns are otherwise valid.

If one of the patterns is an existing, valid path, then that path relative to
the git root is added to .gitignore instead of the plain pattern.

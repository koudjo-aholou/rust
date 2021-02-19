#Rust Notes
##Commandes
### Create a new project with Cargo
..* cargo new my_new_project && cd $_

### Create a new project with Cargo in an existing Git repository
..* cargo new my_new_project --vcs=git

>It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository; you can override this behavior by using cargo new --vcs=git.

### Building, Running, Checking a Cargo Project
..* cargo build
..* cargo run
..* cargo check

### Building for Release
..* cargo build --release
 > Info
 > This command will create an executable in target/release
 >
 
## Git.ignore
>
> // Generated files
>   /target/

> //  # The library shouldn't decide about the exact versions of 
> //  # its dependencies, but let the downstream crate decide.
>  Cargo.lock

##Tools
..* The Rust community’s crate registry : https://crates.io/
..* Linter : https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

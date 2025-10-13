# Simple Site Builder (ssb)

My simple static site builder that 
uses Jinja style templates. Mainly
a personal project but feel free
to poke around. 

details at [ssb.alanwsmith.com](https://ssb.alanwsmith.com/)

## Release Process

- First off, don't use `cargo install --path .`
Build a release file and drop it on your path
next to the prior versions. Process for
that is as follows:

- Get the prior version number by looking in
the releases dir. 

- Bump the version number in the Cargo.toml file
(Everything starts with `0` right now. The
second number is the "Major" version slot which
indicates breaking changes. The third number
is for both Minor and Patch bumps. Once a 
1.0.0 is release it'll move to regular
semantic versioning.)

- If it's a breaking change, bump the version
number of the `src/bin/ssb-x-x-x.rs` file.

- Run the `build-release` script. 

- Copy the built artifact onto your path. 
(I'm currently using `~/binaries` which 
I've added to my zsh config file. 

- Make a commit on the `main` git branch
that includes the release binary file. 


## TODO

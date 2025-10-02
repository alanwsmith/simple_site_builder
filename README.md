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

- [] Don't pre-highlight files. Or, maybe
cache them if you do? The process bogs
down on things like p5.min.js. 

- [] Add tests

- [] Set up the builder to restart itself if
it crashes.

- [] Auto run scripts in `support/pre` 
and `support/post` before and after each
build. 

- [] Split out build process from server process
and use a command line flag to do just the build
(e.g. to make testing easier). 

- [] Add a JavaScript Minifier. 

- [] Documentation

- [] Image optimization where raw files are stored
wherever in the `content` dir, but an output
dir with each file available based on name
the same way you do it in Neopoligen is
made available (make it a `.images` dir name
or some other dot directory to hide it? 
not sure that matters, but the other key
is to let templates call any image with
something like `[` `@ image(NAME) @` `]`)

- [] Ability to include files. (need
to think through security, but maybe
it's that you have an `includes` directory
that has sym links out to the files
you want to include? Is that safe?)

- [POSSIBLY DONE] Load any `.json` file in the content
directory so it's data can be accessed
in templates. 

- [] Load data from the `support/data` directory
and make it available (probably in the
same namespace as the JSONs from the content
tree)

- [] Run basic find/replace over files
to update text strings. (Note that there's
no escaping. Changing names is the way
to avoid collisions)

- [] Use the `support/find-replace` dir to 
set up find and replace strings to do in
files. 

- [] Set up a function to call out to
`support/plugins` processes. 



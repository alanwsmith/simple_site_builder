# Simple Site Builder (ssb)

My simple static site builder that 
uses Jinja style templates. Mainly
a personal project but feel free
to poke around. 

details at [ssb.alanwsmith.com](https://ssb.alanwsmith.com/)

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



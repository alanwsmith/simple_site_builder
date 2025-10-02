# Support Directory

## TODO

- [] Ingest and `.json` files in the `data`
directory and make them available in the
templates. (e.g. `data/config.json` is where
config stuff should be stored)

- [] Load all the files in the `file-replace`
directory by filename and then use them
to do find replaces in all the text based
files prior doing the built process. 

- [] Provide a way to shell out to
plugins using stdin/stdout.

- [] Run any scripts in the `pre` directory
before doing the rest of the build
process. 

- [] Run any scripts in the `post` directory
after the build process runs. 

- [] `include-links` is an initial idea to
have a place to store symbolic links to 
files that are outside the site's dir
to make them available in templates. 
Need to examine the security implications
of that idea before implementing. 





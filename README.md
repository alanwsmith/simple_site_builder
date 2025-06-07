# ssbuild

A personal, bare-bones static site builder that 
uses jinja style templates

## Overview

This is my quick site builder. It'll eventually
be replaced by [Neopoligen](https://www.neopoligen.com/). 
It lets me quickly fire up site using templates 
in the mean time.

(It also servers as a small thing to practice on
to learn more about rust, tokio, static site builders
in general, etc...)

Feel free to poke around to see how things work, but
I probably wouldn't use it if I were you. It's super
customized for the way I want to use it.

## Details 

- [x] Runs in the current directory

- [x] Pulls templates from `./templates`

- [x] Pulls .html files from `./content`,
processes them as templates in MiniJinja
and drops the output in `./docs` with the same
file name/path that the source file had in `./content`.

- [] Copies non .html files from `./content/`
directly to `./docs` with the same file name/path.







- TODO: Serves a dev version of the site out
of a "docs" folder in the current directory.

- TODO: Loads templates from the "templates"
folder in the current directory.

- TODO: Runs any executable files in a 
"scripts" directory" sorted by when things
change (TBD on filtering this so content
doesn't trigger scripts which make content
that trigger scripts etc...)

- TODO: Runs ".html" files in the "content" folder
through minijinja and puts the results
in the "docs" folder with the same filename. 

- TODO: Copies other files from the "content" folder
into the "docs" folder unchanged

- DONE: Looks for a free port to serve on between
5444 and 6000

- TODO: Servers a built-in 404 page if a file can't
be found.




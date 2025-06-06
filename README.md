# ssbuild

A bare-bones static site builder that 
uses jinja style templates


## Notes

- TODO: Runs in the current directory.

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




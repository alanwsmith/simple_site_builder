# Simple Site Builder (ssb)

My simple static site builder that 
uses Jinja style templates. Mainly
a personal project but feel free
to poke around. 

details at [ssb.alanwsmith.com](https://ssb.alanwsmith.com/)

## Deploying a New Version

- Update the version number in Cargo.toml

    If the API doesn't change make it
    a patch level update: x.x.PATCH

    If it's a big change, do the second
    number in the rust tradition:
    x.UPDATE.x

- If it's a big bump, also update
the `[[bin]] name` in Cargo.toml
to `ssb-0-VERSION`. 

- Run `build-release`

- commit the changes to the repo 
(since the binaries are stored in the repo)

- Copy the output from 

    ./content/releases/.../0.VERSION.PATH/ssb-0-VERSION

    to:

    ~/binaries/ssb-0-VERSION

- Update the symbolic link to the binary. 

- deploy the site (e.g. ``push updated to version 0.9.X``)




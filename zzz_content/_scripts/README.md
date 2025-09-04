# Scripts

Anything executable in this directory gets
run before each site build. 

The files are run in order alphabetical
sort order with all letters lowercased. 

Using leading zeros for ordering is
recommended. For example:

0010-some-script.py
0020-another-script.py

etc...

Going up by tens allows for dropping
in other files with relative ease. Like:


0010-some-script.py
0015-added-script.py
0020-another-script.py

Outputting `.json` files to the `_data`
directory makes the data available 
for inclusion in templates. 


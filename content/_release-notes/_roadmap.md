## Roadmap

- TODO: Update so that leading
single underscore dirs and files
get published but two or more
are only used internally
(i.e. `_example` goes to prod,
but `__example` does not)


- TODO: Update so that multiple 
changes debounce properly with
large directory moves. Thinking
the best way to do this might 
be to push change requests
onto an array and then just clear
the array on each run. 


- Parse HTML based tags with 
`Hinja` - MiniJinja, but with HTML 
tags



##Student: Toma-Ioan Dumitrescu
##Group: 321CA

This project implements basic commands present in linux terminal: pwd, echo, cat, cp, mv, rm, rmdir,
chmod, mkdir, touch, cat, ls, for simplified input cases (parameters).

The idea consist in retaining the list of arguments and calling the appropriate std::fs functions in
order to perform the requested operations. For returning value, the program uses use std::process::exit.

pwd -> neglect rare errors (current path has been deletes etc), display the path of current_dir()

echo -> input cornercases, if -n is present => print with print!, not println!
		Take a slice of arg_list, after -n or after echo if -n is not present, clone it because of
borrowing, collect the elements of the list in a string array and join the elements in a string
where elements are separated by " ".

print_file -> auxiliary function for cat, transforms the content of a file in a string (quote *)

cat -> for every filename, apply print_file and check for errors

mkdir -> for all directories in arg_list, call create_dir fs function, check errors and exit the
		program with the appropriate code

rmdir -> same inference from mkdir

chmod -> change permissions when numerical mask is used. Unfortunately, this works for string case
only when initial permissions do not matter.

rm -> remove a file or a directory based on the options. As we can have maximum 2 options, we can
analyse exactly 2 arguments after rm if they are -r, -R, --dir or -d. Error exit only after the for
loop, as some files may still be deleted. Use of the functions fpath.exists() or fpath.is_dir() to
check some information about the parameter.

touch -> works only when there are no options. Use open options with new, create (quote **)

ls -> reading actual path and find all directories, use a for loop to print them. Only simple
ls implemented.

Bibliography:
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html
https://doc.rust-lang.org/rust-by-example/index.html (for *, **)

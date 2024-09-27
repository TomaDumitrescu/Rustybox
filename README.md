## Copyright 2024 Toma-Ioan Dumitrescu

### Description

This project implements the following commands present in linux terminal: pwd, echo (-n), cat, cp (-r), mv, rm (-r), rmdir, chmod (numerical or string mask), mkdir, touch (-a, -c), ls (-r, -a), grep (-i), ln (-s), using only functions from std and regex, without directly executing the Linux commands (example: exec).

### Implementation

The idea consists in retaining the list of arguments and calling the appropriate std::fs functions in
order to perform the requested operations. For returning value, the program uses use std::process::exit. Most of the functions for directories iterator and filesystem operations were
found by searching on https://doc.rust-lang.org/stable/std/fs/index.html. Basic error handling
for the commands with appropriate exit codes.

pwd -> display the path of current_dir().

echo -> input cornercases, if -n is present => print with print!, not println!
		Take a slice of arg_list, after -n or after echo if -n is not present, clone it because of
borrowing, collect the elements of the list in a string array and join the elements in a string
where elements are separated by " ".

print_file -> auxiliary function for cat, transforms the content of a file in a string, by
returning a Result<String> value: File::open(Path)?.read_to_string(mut string var).

cat -> for every filename, apply print_file and check for errors.

mkdir -> for all directories in arg_list, call create_dir fs function, check errors and exit the
		program with the appropriate code, fs::create_dir(&String).

rmdir -> same inference from mkdir, fs::remove_dir(&String).

retrieve_file_perm -> auxiliary function for chmod that uses fs::metadata(path)? and
.permissions() to get the current permissions of the file.

chmod -> change permissions for the given file. If the mask is numerical, convert the
numerical string mask to an octal mask, using u32::from_str_radix(num, base), change to the
appropriate type with fs::Permissions::from_mode(octal_mask) and then use
fs::set_permissions(path, perm). In the other case, the string ugoa+-rwx mask is converted
to a numerical octal mask, using the definition of the permission mask. When adding permissions,
perform | bitwise operation between initial permission mask and new numerical mask and when
removing permissions, & is done.

rm_r -> auxiliary function for rm in the case of deleting recursively a directory. It obtains
an interator for the current directory with fs::read_dir(&Path)?, then for each file_node
found by the iterator, perform again rm_r if it is a directory (path.is_dir()) or just rm if
it is a file (path.is_file()). After this loop, the empty directory read is removed.

rm -> remove a file or a directory based on the options. As we can have maximum 2 options, we can
analyse exactly 2 arguments after rm if they are -r, -R, --dir or -d. Error exit only after the for
loop, as some files may still be deleted. Use of the functions fpath.exists() or fpath.is_dir() to
check some information about the parameters. The operation is done via fs::remove_file(&Path)
or by fs::remove_dir(&Path) if the directory flag was set in the command header.

touch -> with no options, it creates the file using OpenOptions::new().create(true).write(true)
.open(&Path). For touch -c, -a, -m, opening the file in read or write mode and modifying the file
and reversing modifications are the techniques used for updating atime or mtime given the case.

ls_r -> auxiliary function for ls command. Principle: traversing a directory using an iterator
and add to an array each directory found. After the loop, apply again ls_r over the array of
saved directories. The paths will be printed according to ls rustybox format specification.
PathBuf used for the correlations between refs and Path variables.

ls -> checks for -a or -r flags and operates by traversing all nodes of the current directory
in a shallow manner. The command ls -l is not implemented and classified as an invalid
command.

ln -> for hard link it uses fs::hardlink(file, link_name) and for symbolic link,
fs::symlink(file, link_name).

grep -> String expression expr is converted to a regex, the file is opened and an iterator that
extract file lines is created. Using expr.is_match(&line) from Regex, the line is verified to
contain the desired expression. If -i flag is set, then the condition to display the line is
!expr.is_match(&line).

cp_r -> helper for cp that creates the destination and intermediary folders (if needed) and
deep copies source components to destination using a directory iterator. If a directory
is found via dir_itr, then apply again cp_r with source as the found directory. If a file
is found by dir_itr, cp_r uses fs::copy(src, dest) for shallow copy. Since the files
contain the full relative path, the directories found by dir_itr will be created, because
of fs::create_dir_all() and of adding the respective directory to the relative path.

cp -> if the destination is a file, then simple fs::copy(src, dest) is used. If the destination
is a folder, then add the source file to the destination path and copy the content from src
to (path(dest) + path(src)).

mv -> if the source does not exists, then it is clearly an error (nothing to rename or to
move). If the destination exists, then it is clearly a rename operation to do, so fs::rename
(src, dest) is used. In the other case, construct cp and rm commands and obtain actual move
command.

### Bibliography:
https://doc.rust-lang.org/stable/std/index.html
https://doc.rust-lang.org/rust-by-example/std_misc/fs.html

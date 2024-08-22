use std::io;
use std::io::prelude::*;
use std::env;
use std::env::current_dir;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::os::unix::fs::PermissionsExt;
use regex::Regex;

// Prints the absolute path of the current directory
fn pwd()
{
	let working_path = current_dir().unwrap();
	println!("{}", working_path.display());
	exit(0);
}

// Prints the arguments of the command space-separated
fn echo(arg_list: &[String])
{
	// Echo with no parameters prints newline
	if arg_list.len() <= 2 {
		println!("");
		exit(0);
	}

	if arg_list[2] == "-n" {

		/* The first 3 arguments are skipped, cloned() because
		there are multiple elements (borrowing reasons) */
		let cloned_list = arg_list.iter().skip(3).cloned();

		// transform the array into a string with space separator
		let joined_elements = cloned_list.collect::<Vec<String>>().join(" ");

		print!("{}", joined_elements);
		exit(0);
	} else {
		let cloned_list = arg_list.iter().skip(2).cloned();
		let joined_elements = cloned_list.collect::<Vec<String>>().join(" ");

		println!("{}", joined_elements);
		exit(0);
	}
}

// Content of a file will be  retained as a String
fn print_file(fname: &Path) -> io::Result<String>
{
	let mut characters = String::new();
	let fpointer = File::open(fname);

	// ? because fpointer could not exist
	match fpointer?.read_to_string(&mut characters) {
		Err(_) => {
			exit(-20);
		}

		Ok(_) => Ok(characters),
	}
}

// Prints the content of a file
fn cat(arg_list: &[String])
{
	for arg in arg_list.iter().skip(2) {
		// Convert arg string to Path type
		match print_file(&Path::new(&arg)) {
			Ok(content) => {
				print!("{}", content);
			},

			Err(_) => {
				exit(-20);
			},
		}
	}

	exit(0);
}

// Creating directories from the parameter list in the file system
fn mkdir(arg_list: &[String])
{
	for arg in arg_list.iter().skip(2) {
		// fs function for creating directories
		match fs::create_dir(&arg) {
			Ok(_) => (),
			Err(_) => {
				exit(-30);
			},
		}
	}

	exit(0);
}


// Removes empty directories from the parameter list
fn rmdir(arg_list: &[String])
{
	for arg in arg_list.iter().skip(2) {
		match fs::remove_dir(&arg) {
			Err(_) => {
				exit(-60);
			},
			Ok(_) => (),
		}
	}

	exit(0);
}

// Using file metadata, integer mask permission is retrieved
fn retrieve_file_perm(fpath: &str) -> std::io::Result<u32> {
	let fmetadata = fs::metadata(fpath)?;
	let fperm = fmetadata.permissions();
	Ok(fperm.mode())
}

// Changes permissions to specific ones
fn chmod(arg_list: &[String])
{
	let fpath = &Path::new(&arg_list[arg_list.len() - 1]);
	let char1 = arg_list[2].chars().next().expect("_");

	// Mask is generally numeric the first character is numeric
	if char1.is_numeric() {
		if arg_list.len() != 4 {
			exit(-25);
		}

		// Covert a string to a number in octal base
		let mask = match u32::from_str_radix(&arg_list[2], 8) {
			Ok(ok) => ok,
			Err(_e) => exit(-25),
		};

		let perm = fs::Permissions::from_mode(mask);
		match fs::set_permissions(fpath, perm) {
			Ok(_ok) => {
			}
			Err(_e) => {
				exit(-25);
			}
		}

		exit(0);
	} else {
		if char1 != 'u' && char1 != 'g' && char1 != 'o' && char1 != 'a' {
			println!("Invalid command");
			exit(-1);
		}

		// retrieving initial permissions of the file
		let path = &arg_list[3];

		let mut init_perm = match retrieve_file_perm(path) {
			Ok(tmp_perm) => tmp_perm,
			Err(_) => {
				exit(-25);
			}
		};

		// Eliminate unnecessary digits
		init_perm = init_perm & 0o777;

		// Convert string mask to numeric mask
		let mut u = 0;
		let mut g = 0;
		let mut o = 0;
		let mut a = 0;
		let mut sgn = 0;
		let mut new_logic = 0;
		let mut o_bit = 0;
		let mut mask = 0;

		for c in arg_list[2].chars() {
			if c == '+' {
				new_logic = 1;
				sgn = 1;
			}
			if c == '-' {
				new_logic = 1;
			}
			if c == 'u' {
				u = 1;
			}
			if c == 'g' {
				g = 1;
			}
			if c == 'o' {
				o = 1;
			}
			if c == 'a' {
				a = 1;
			}
			if new_logic == 1 {
				if c == 'r' {
					o_bit += 4;
				}
				if c == 'w' {
					o_bit += 2;
				}
				if c == 'x' {
					o_bit += 1;
				}
			}
		}

		if u == 1 {
			mask += 10 * 10 * o_bit;
		}

		if g == 1 {
			mask += 10 * o_bit;
		}

		if o == 1 {
			mask += o_bit;
		}

		if a == 1 {
			mask += (1 + 10 + 10 * 10) * o_bit;
		}

		let o_str_mask = format!("{}", mask);
		let o_mask = match u32::from_str_radix(&o_str_mask, 8) {
			Ok(ok) => ok,
			Err(_e) => exit(-25),
		};
		let mut perm = fs::Permissions::from_mode(o_mask | init_perm);
		if sgn == 0 {
			perm = fs::Permissions::from_mode((!o_mask) & init_perm);
		}

		match fs::set_permissions(fpath, perm) {
			Ok(_) => {
			}
			Err(_) => {
				exit(-25);
			}
		}
	
		exit(0);
	}
}

// Remove recursive
fn rm_r(dir: &Path) {
	// Iterator for the current directory
	let dir_itr = fs::read_dir(dir).unwrap();

	// Traverse directory elements
	for node_err in dir_itr {
		let node = node_err.unwrap();
		let npath = node.path();

		if !npath.is_dir() {
			// Simple rm command
			let res = fs::remove_file(&npath);
			match res {
				Ok(_ok) => (),
				Err(_e) => {
					exit(-70);
				},
			}
		} else {
			// Necessary for removing non-empty directories
			rm_r(&npath);
		}
	}

	// Removing the current directory (now empty)
	match fs::remove_dir(&dir) {
		Err(_e) => {
			exit(-70);
		},
		Ok(_ok) => {},
	}
}

// Remove command for deleting files and directories
fn rm(arg_list: &[String])
{
	let mut idx = 2;

	// If an error is encountered, the command continues with next parameters
	let mut error = 0;

	if arg_list[2] == "-r" || arg_list[2] == "-R" || arg_list[2] == "--dir"
		|| arg_list[2] == "-d" || arg_list[2] == "--recursive" {
		idx += 1;

		if arg_list.len() <= 3 {
			println!("Invalid command");
			exit(-1);
		}
	}

	if arg_list.len() >= 4 && (arg_list[3] == "-r" || arg_list[3] == "-R" ||
		arg_list[3] == "--dir" || arg_list[3] == "-d" || arg_list[3] == "--recursive") {
		idx += 1;

		if arg_list.len() <= 4 {
			println!("Invalid command");
			exit(-1);
		}
	}

	let not_dir_flag = arg_list[2] != "--dir" && arg_list[2] != "-d" &&
	!(arg_list.len() >= 4 && (arg_list[3] == "--dir" || arg_list[3] == "-d"));

	let not_r_flag = arg_list[2] != "-r" && arg_list[2] != "-R" && arg_list[2] != "--recursive" &&
	!(arg_list.len() >= 4 && (arg_list[3] == "-r" || arg_list[3] == "-R" || arg_list[3] == "--recursive"));

	// idx was set to skip flags and to jump directly to file names
	for arg in arg_list.iter().skip(idx) {
		let fpath = &Path::new(&arg);
		if !fpath.exists() {
			error = 1;
			continue;
		}

		if fpath.is_dir() {
			if not_r_flag && not_dir_flag {
				error = 1;
				continue;
			}

			// Recursive flag
			if !not_r_flag {
				rm_r(fpath);
				continue;
			}

			// Remove directory flag (for empty ones)
			if !not_dir_flag {
				match fs::remove_dir(&fpath) {
					Err(_e) => {
						error = 1;
						continue;
					},
					Ok(_ok) => {},
				}
			}
		} else {
			match fs::remove_file(&arg) {
				Ok(_ok) => {}
				Err(_e) => {
					error = 1;
					continue;
				}
			}
		}
	}

	if error == 1 {
		exit(-70);
	} else {
		exit(0);
	}
}

// Creating a non-existent file or modifying file times
fn touch(arg_list: &[String])
{
	let mut skip_idx = 0;
	let param: bool = arg_list[2] == "-c" || arg_list[2] == "--no-create" ||
					arg_list[2] == "-a" || arg_list[2] == "-m";

	if arg_list.len() >= 3 && param {
		skip_idx += 1;
	}

	for arg in arg_list.iter().skip(2 + skip_idx) {
		let fpath = Path::new(&arg);

		// This will let changing atime and mtime sequentially
		let exists: bool = fpath.exists();
		if !exists && (arg_list[2] == "-c" || arg_list[2] == "--no-create") {
			exit(0);
		}

		if arg_list[2] == "-a" || arg_list[2] == "-c" || arg_list[2] == "--no-create" || exists {
			match fs::OpenOptions::new().read(true).open(fpath) {
				Ok(mut file) => {
					/* Just opening the file in read mode is not modifying the access time,
					so actually reading something (like 1 byte) from the file is required */
					let mut buff = [0; 1];
					let _ = file.read(&mut buff).unwrap();
				},
				Err(_) => {
					exit(-100);
				},
			}
		}

		if arg_list[2] == "-m" || arg_list[2] == "-c" || arg_list[2] == "--no-create" || exists {
			match fs::OpenOptions::new().write(true).open(fpath) {
				Ok(mut file) => {
					// Actual write to modify the mtime of the file
					let fsize = file.metadata().unwrap().len();
					match file.write_all(b"\n") {
						Ok(_) => (),
						Err(_) => (),
					}

					// Assure the writing was done
					file.flush().unwrap();

					// Truncate the file to remove the added content
					file.set_len(fsize).unwrap();
				},
				Err(_) => {
					exit(-100);
				},
			}

			continue;
		}

		// Creating non-existent file using fpath
		match OpenOptions::new().create(true).write(true).open(fpath) {
			Ok(_) => {},
			Err(_e) => {
				exit(-100);
			},
		}
	}

	exit(0);
}

// List directories recursive
fn ls_r(dir: &Path, hidden: bool, prefix: &mut String)
{
	let dir_itr = fs::read_dir(dir).unwrap();

	/* The traversed directories are added to an array to be
	again traversed, simulating ls -R command behavior */
	let mut dpaths: Vec<PathBuf> = Vec::new();

	// If -a flag, then list current and parent directories signs
	if hidden {
		// Prefix from the ls_r parameter completes the relative path
		let mut prefix_c1 = prefix.clone();
		prefix_c1.push_str(".");
		println!("{}", prefix_c1);

		// Clone needed to not modify prefix in wrong sense
		let mut prefix_c1 = prefix.clone();
		prefix_c1.push_str("..");
		println!("{}", prefix_c1);
	}

	for node_err in dir_itr {
		let node = node_err.unwrap();
		let npath = node.path();

		if npath.is_dir() {
			// Because of borrowing
			dpaths.push(npath.clone());
		}

		// Extract file name and convert it to a string
		let nname_opt = npath.file_name()
		.and_then(|fname| fname.to_str())
		.map(|fstr| fstr.to_string());

		match nname_opt {
			Some(fname_string) => {
				let mut prefix_c = prefix.clone();
				prefix_c.push_str(&fname_string);
				if fname_string.starts_with('.') {
					if !hidden {
						continue;
					}
	
					println!("{}", prefix_c);
				} else {
					println!("{}", prefix_c);
				}
			},
			None => (),
		}
	}

	for dir in dpaths {
		let string_dir = dir.file_name()
		.and_then(|fname| fname.to_str())
		.map(|fstr| fstr.to_string());

		match string_dir {
			Some(dname_string) => {
				let mut relative_path = prefix.clone();
				// Mark the advance in the file tree in the prefix path
				relative_path.push_str(&dname_string);
				relative_path.push_str("/");
				ls_r(&dir, hidden, &mut relative_path);
			},
			None => (),
		}
	}
}

// List directories command
fn ls(arg_list: &[String])
{
	let mut hidden: bool = false;
	let mut recursive: bool = false;
	let mut fpath_str = ".";
	let mut skip_idx = 0;

	for i in 0..=1 {
		if arg_list.len() >= 3 + i && (arg_list[2 + i] == "-a" || arg_list[2 + i] == "-all") {
			skip_idx += 1;
			hidden = true;
		}
	
		if arg_list.len() >= 3 + i && (arg_list[2 + i] == "-R" || arg_list[2 + i] == "--recursive") {
			skip_idx += 1;
			recursive = true;
		}
	}

	if arg_list.len() > 2 + skip_idx {
		fpath_str = &arg_list[2 + skip_idx];
	}

	let fpath = Path::new(fpath_str);

	if fpath.is_file() {
		println!("{}", arg_list[2 + skip_idx]);
	}

	if recursive {
		let empty_path = "";
		ls_r(fpath, hidden, &mut empty_path.to_string());
		exit(0);
	}

	if hidden {
		println!(".\n..");
	}

	match fs::read_dir(fpath) {
		Err(_err) => exit(-80),
		Ok(dirs) => for dir in dirs {
			let dpath = dir.unwrap().path();
			let fname_opt = dpath.file_name()
			.and_then(|fname| fname.to_str())
			.map(|fstr| fstr.to_string());

			match fname_opt {
				Some(fname_string) => {
					if fname_string.starts_with('.') {
						// Listing only if -a flag present
						if !hidden {
							continue;
						}

						println!("{}", fname_string);
					} else {
						println!("{}", fname_string);
					}
				},
				None => (),
			}
		},
	}
}

// Creates a hard or symbolic link to a file
fn ln(arg_list: &[String])
{
	if arg_list.len() <= 3 {
		exit(-50);
	}

	if arg_list.len() > 4 && arg_list[2] != "-s" && arg_list[2] != "--symbolic" {
		println!("Invalid command");
		exit(-1);
	}

	if (arg_list[2] == "-s" || arg_list[2] == "--symbolic") && arg_list.len() <= 4 {
		exit(-50);
	}

	if arg_list[2] == "-s" || arg_list[2] == "--symbolic" {
		match std::os::unix::fs::symlink(&arg_list[3], &arg_list[4]) {
			Ok(_) => (),
			Err(_) => {
				exit(-50);
			},
		}

		exit(0);
	}

	match fs::hard_link(&arg_list[2], &arg_list[3]) {
		Ok(_) => (),
		Err(_) => {
			exit(-50);
		}
	}

	exit(0);
}

// Printing all file lines that match a regex
fn grep(arg_list: &[String])
{
	let mut idx: usize = 2;
	if arg_list[idx] == "-i" {
		idx += 1;
	}

	// Convert string pattern to Regex type
	let expr = Regex::new(&arg_list[idx]).unwrap();

	let content = File::open(&arg_list[idx + 1]).unwrap();
	let content_itr = io::BufReader::new(content);
	let rows = content_itr.lines();

	for row_err in rows {
		let row = row_err.unwrap();
		if idx == 2 && expr.is_match(&row) {
			println!("{}", row);
		}

		// Complement of simple grep
		if idx == 3 && !expr.is_match(&row) {
			println!("{}", row);
		}
	}
}

// Copy recursive
fn cp_r(source: &Path, destination: &Path) {
	if !destination.exists() {
		// Creates all missing directories (/ separated) from the destination path
		let op_res = fs::create_dir_all(destination);
		match op_res {
			Ok(()) => (),
			Err(_) => {
				exit(-90);
			},
		}
	}

	let dir_itr = fs::read_dir(source).unwrap();
	for node_err in dir_itr {
		let node = node_err.unwrap();
		let spath = node.path();
		let dpath = destination.join(node.file_name());

		if !spath.is_dir() {
			// Shallow copy
			let res = fs::copy(&spath, &dpath);
			match res {
				Ok(_) => (),
				Err(_) => {
					exit(-90);
				}
			}
		} else {
			// Deep copy
			cp_r(&spath, &dpath);
		}
	}
}

// Copies a file or directory from source to destination
fn cp(arg_list: &[String]) {
	let not_recursive: bool = arg_list[2] != "-r" && arg_list[2] != "-R"
								&& arg_list[2] != "--recursive";

	if arg_list.len() != 4 && not_recursive {
		println!("Invalid command");
		exit(-1);
	}

	if arg_list.len() != 5 && !not_recursive {
		println!("Invalid command");
		exit(-1);
	}

	let rsource = Path::new(&arg_list[3]);

	// Deep copy needed only for directories
	if !not_recursive && rsource.is_dir() {
		let rdestination = Path::new(&arg_list[4]);
		let rdestination_full = if rdestination.exists() && rdestination.is_dir() {
			// Adds source folder to destionation path
			rdestination.join(rsource.file_name().unwrap())
		} else {
			rdestination.to_path_buf()
		};

		cp_r(rsource, &rdestination_full);

		exit(0);
	}

	let mut skip_arg = 0;
	if !not_recursive {
		skip_arg += 1;
	}

	let source = Path::new(&arg_list[2 + skip_arg]);
	let destination = Path::new(&arg_list[3 + skip_arg]);

	if !source.exists() || source.is_dir() {
		exit(-90);
	}

	if !destination.is_dir() {
		let res = fs::copy(source, destination);
		match res {
			Ok(_val) => (),
			Err(_err) => {
				exit(-90);
			},
		}
	} else {
		let res = fs::copy(source, destination.join(source.file_name().unwrap()));
		match res {
			Ok(_val) => (),
			Err(_err) => {
				exit(-90);
			},
		}
	}
}

// Renames or changes the file location to destination
fn mv(arg_list: &[String]) {
	if arg_list.len() != 4 {
		println!("Invalid command");
		exit(-1);
	}

	let source = Path::new(&arg_list[2]);
	let destination = Path::new(&arg_list[3]);

	if !source.exists() {
		exit(-40);
	}

	if !destination.exists() {
		match fs::rename(source, destination) {
			Ok(()) => (),
			Err(_err) => {
				exit(-40);
			},
		}
	} else {
		if source.is_dir() && !destination.is_dir() {
			exit(-40);
		}

		// Changing location = cp + rm command, eventually -r

		let cp_args = vec![
			String::from("skip_1"),
			String::from("cp"),
			String::from("-r"),
			arg_list[2].clone(),
			arg_list[3].clone(),
		];

		cp(&cp_args);

		let rm_args = vec![
			String::from("skip_1"),
			String::from("rm"),
			String::from("-r"),
			arg_list[2].clone(),
		];

		rm(&rm_args);
	}
}

fn main()
{
	let arg_list: Vec<String> = env::args().collect();

	if arg_list.len() <= 1 {
		println!("Invalid command");
		exit(-1);
	}

	// Command selection menu
	match (&arg_list[1]).as_str() {
		"pwd" => {
			pwd();
		}

		"echo" => {
			echo(&arg_list);
		}

		"cat" => {
			cat(&arg_list);
		}

		"mkdir" => {
			mkdir(&arg_list);
		}

		"rmdir" => {
			rmdir(&arg_list);
		}

		"chmod" => {
			chmod(&arg_list);
		}

		"rm" => {
			rm(&arg_list);
		}

		"touch" => {
			touch(&arg_list);
		}

		"ls" => {
			ls(&arg_list);
		}

		"ln" => {
			ln(&arg_list);
		}

		"grep" => {
			grep(&arg_list);
		}

		"cp" => {
			cp(&arg_list);
		}

		"mv" => {
			mv(&arg_list);
		}

		_ => {
			println!("Invalid command");
			exit(-1);
		}
	}
}

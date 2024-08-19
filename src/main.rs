use std::io;
use std::io::prelude::*;
use std::env;
use std::env::current_dir;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::exit;
use std::os::unix::fs::PermissionsExt;

// very rare errors for this command
fn pwd()
{
	let actd_p = current_dir().unwrap();
	println!("{}", actd_p.display());
	exit(0);
}

fn echo(arg_list: &[String])
{
	// echo with no parameters prints newline
	if arg_list.len() <= 2 {
		println!("");
		exit(0);
	}

	// strings can be compared in rust with ==
	if arg_list[2] == "-n" {

		/* top 3 left arguments are skipped, sort of a slice, cloned because
		there are multiple elements (clone() works for single elements)
		cloned() => used because of borrowing */
		let cloned_list = arg_list.iter().skip(3).cloned();

		// transform the array into a string with space separator
		let joined_elements = cloned_list.collect::<Vec<String>>().join(" ");

		// unlike println!, print! does not add new line
		print!("{}", joined_elements);
		exit(0);
	} else {
		let cloned_list = arg_list.iter().skip(2).cloned();
		let joined_elements = cloned_list.collect::<Vec<String>>().join(" ");

		println!("{}", joined_elements);
		exit(0);
	}
}

fn print_file(fname: &Path) -> io::Result<String>
{
	let mut characters = String::new();
	let fpointer = File::open(fname);

	// ? because fpointer could not exist
	match fpointer?.read_to_string(&mut characters) {
		Err(_e) => {
			exit(-20);
		}

		Ok(_ok) => Ok(characters),
	}
}

fn cat(arg_list: &[String])
{
	for arg in arg_list.iter().skip(2) {
		// convert arg to path type
		match print_file(&Path::new(&arg)) {
			Ok(characters) => {
				print!("{}", characters);
			},

			Err(_e) => {
				exit(-20);
			},
		}
	}

	exit(0);
}

fn mkdir(arg_list: &[String])
{
	for arg in arg_list.iter().skip(2) {
		// fs function for creating directories called repeatedly
		match fs::create_dir(&arg) {
			Ok(_ok) => {},
			Err(_e) => {
				exit(-30);
			},
		}
	}

	exit(0);
}

fn rmdir(arg_list: &[String])
{
	for arg in arg_list.iter().skip(2) {
		match fs::remove_dir(&arg) {
			Err(_e) => {
				exit(-60);
			},
			Ok(_ok) => {},
		}
	}

	exit(0);
}

fn retrieve_file_perm(fpath: &str) -> std::io::Result<u32> {
    let fmetadata = fs::metadata(fpath)?;
    let fperm = fmetadata.permissions();
    Ok(fperm.mode())
}

// observe that mask is numeric if and only if first character is numeric
fn chmod(arg_list: &[String])
{
	let fpath = &Path::new(&arg_list[arg_list.len() - 1]);
	let char1 = arg_list[2].chars().next().expect("_");
	if char1.is_numeric() {
		if arg_list.len() != 4 {
			exit(-25);
		}

		// function to covert a string to a number in a specified base (octal here)
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
			Err(_err) => {
				exit(-25);
			}
		};

		init_perm = init_perm & 0o777;

		// chars in strings cannot be directly accessed using indexes (multi-bytes)
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

		if sgn == 0 {
			if u == 1 {
				u = 8 - (mask / 100);
			}
			if g == 1 {
				g = 8 - ((mask / 10) % 10);
			}
			if o == 1 {
				o = 8 - (mask % 10);
			}
			mask = 10 * 10 * u + 10 * g + o;
		}

		let o_str_mask = format!("{}", mask);
		println!("Mask {}", mask);
		let o_mask = match u32::from_str_radix(&o_str_mask, 8) {
			Ok(ok) => ok,
			Err(_e) => exit(-25),
		};
		let perm = fs::Permissions::from_mode(o_mask & init_perm);
		match fs::set_permissions(fpath, perm) {
			Ok(_ok) => {
			}
			Err(_e) => {
				exit(-25);
			}
		}
		exit(0);
	}
}

// non-recursive rm
fn rm(arg_list: &[String])
{
	let mut idx = 2;
	let mut error = 0;

	if arg_list[2] == "-r" || arg_list[2] == "-R" || arg_list[2] == "--dir"
		|| arg_list[2] == "-d" {
		idx += 1;

        if arg_list.len() <= 3 {
            println!("Invalid command");
            exit(-1);
        }
	}

	if arg_list.len() >= 4 && (arg_list[3] == "-r" || arg_list[3] == "-R" ||
        arg_list[3] == "--dir" || arg_list[3] == "-d") {
		idx += 1;

        if arg_list.len() <= 4 {
            println!("Invalid command");
            exit(-1);
        }
	}

	for arg in arg_list.iter().skip(idx) {
		let fpath = &Path::new(&arg);
		if !fpath.exists() {
			exit(-70);
		}
		if arg_list[2] != "--dir" && arg_list[2] != "-d" &&
            !(arg_list.len() >= 4 &&
			(arg_list[3] == "--dir" && arg_list[3] == "-d")) {
			if fpath.is_dir() {
				// the directory is not removed (just ignored)
				error = 1;
			}

			match fs::remove_file(&arg) {
				Ok(_ok) => {}
				Err(_e) => {
					error = 1;
				}
			}
		} else {
			match fs::remove_dir(&arg) {
				Err(_e) => {
								error = 1;
				},
				Ok(_ok) => {},
			}
		}
	}

	if error == 1 {
		exit(-70);
	} else {
		exit(0);
	}
}

fn touch(arg_list: &[String])
{
	for arg in arg_list.iter().skip(2) {
		let fpath = &Path::new(&arg);
		if arg_list[2] == "-a" {
			exit(-100);
		}

		if arg_list[2] == "-c" {
			match OpenOptions::new().create(false).write(true).open(fpath) {
				Ok(_) => {},
				Err(_e) => {
					exit(-100);
				},
			}
		}

		if arg_list[2] == "-m" {
			exit(-100);
		}

		match OpenOptions::new().create(true).write(true).open(fpath) {
			Ok(_) => {},
			Err(_e) => {
				exit(-100);
			},
		}
	}

	exit(0);
}

fn ls(arg_list: &[String])
{
    let fpath = if arg_list.len() >= 3 {
        &arg_list[2]
    } else {
		"."
    };

	match fs::read_dir(Path::new(fpath)) {
		Err(_err) => exit(-80),
		Ok(dirs) => for dir in dirs {
			println!("> {:?}", dir.unwrap().path());
		},
	}
}

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
		if let Err(_err) = std::os::unix::fs::symlink(&arg_list[3], &arg_list[4]) {
			exit(-50);
		}

		exit(0);
	}

	if let Err(_err) = fs::hard_link(&arg_list[2], &arg_list[3]) {
		exit(-50);
	}

	exit(0);
}

fn main()
{
	let arg_list: Vec<String> = env::args().collect();

    if arg_list.len() <= 1 {
        println!("Invalid command");
        exit(-1);
    }

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

		_ => {
			println!("Invalid command");
			exit(-1);
		}
	}
}

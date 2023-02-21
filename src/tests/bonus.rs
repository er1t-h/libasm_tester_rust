mod atoi_base {
	use std::ffi::CString;

	use crate::ft_atoi_base;

	macro_rules! test {
		($name: ident, $str: expr, $base: expr, $result: expr) => {
			#[test]
			fn $name() {
				let str = CString::new($str).expect("Cannot create str");
				let base = CString::new($base).expect("Cannot create base");
				let ret_val = unsafe { ft_atoi_base(str.as_ptr(), base.as_ptr()) };
				assert_eq!(ret_val, $result);
			}
		};
	}

	test!(basic, "2147483647other_useless_thing", "0123456789", 2147483647);
	test!(binary, "oooxxxxoxxoooxox_yup", "ox", 7877);
	test!(b64, "    lasm+", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789:/", 9808678);
	test!(plus_in_base, "Super", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/", 0);
	test!(neg_in_base, "Super", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-/", 0);
	test!(double_in_base, "Super", "ABCDEFGHIJKLMNOPQRSTUVWXYZAbcdefghijklmnopqrstuvwxyz0123456789:/", 0);
	test!(nothing_in_base, "Super", "", 0);
	test!(nothing_in_str, "", "0123456789", 0);
}

mod list_push_front {
	use crate::{FtList, ft_list_push_front};
	use std::ffi::CString;

	macro_rules! test {
		($name: ident, $arg: expr) => {
			#[test]
			fn $name() {
				let mut list: *const FtList = std::ptr::null();
				let all_data: Vec<CString> = $arg.iter().map(|elt| CString::new(*elt).unwrap()).collect();
				for i in all_data.iter() {
					unsafe {
						ft_list_push_front(&mut list, i.as_ptr() as *const cty::c_void);
					}
				}
				let mut elt: &FtList = unsafe {&*list};
				for (index, i) in all_data.iter().rev().enumerate() {
					let content = unsafe {std::slice::from_raw_parts(elt.data as *mut u8, i.as_bytes().len())};
					println!("Index: {:3} ; Content: {}", index, String::from_utf8_lossy(&content));
					assert_eq!(content, i.as_bytes());
					let tmp: *mut FtList = unsafe { std::ptr::read(&((elt as *const FtList) as *mut FtList)) };
					elt = unsafe {&*elt.next};
					unsafe { libc::free(tmp as *mut cty::c_void); }
				}
			}
		};
	}

	test!(basic, ["Yes", "Nope", "Meh"]);
	test!(more_items, ["Yes", "Nope", "Meh", "I dunno", "Lorem Ipsum", "Riichi Ippatsu Tsumo", "Ron", "ye"]);
	test!(one_item, ["Yes"]);
}

mod list_size {
	use crate::{FtList, ft_list_size, ft_list_push_front};
	use std::ffi::CString;

	macro_rules! test {
		($name: ident, $arg: expr) => {
			#[test]
			fn $name() {
				let mut list: *const FtList = std::ptr::null();
				let all_data: Vec<CString> = $arg.iter().map(|elt| CString::new(*elt).unwrap()).collect();
				for i in all_data.iter() {
					unsafe {
						ft_list_push_front(&mut list, i.as_ptr() as *const cty::c_void);
					}
				}
				let size = unsafe { ft_list_size(list) };
				assert_eq!(size, $arg.len());
				while (!list.is_null()) {
					let tmp: *mut FtList = unsafe { std::ptr::read(&(list as *mut FtList)) };
					list = unsafe {&*(*list).next};
					unsafe { libc::free(tmp as *mut cty::c_void); }
				}
			}
		};
	}

	test!(basic, ["Yes", "Nope", "Meh"]);
	test!(more_items, ["Yes", "Nope", "Meh", "I dunno", "Lorem Ipsum", "Riichi Ippatsu Tsumo", "Ron", "ye"]);
	test!(one_item, ["Yes"]);
	test!(hundred, ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96", "97", "98", "99", "100"]);
}

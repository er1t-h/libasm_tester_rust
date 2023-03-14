use std::ffi::CString;

use crate::ft_strlen;

macro_rules! test {
	($name: ident, $to_test: expr) => {
		#[test]
		fn $name() {
			let test_str = CString::new($to_test).expect("Couldn't create string");
			let result = unsafe {
				ft_strlen(test_str.as_ptr())
			};
			assert_eq!(result, test_str.as_bytes().len());
		}
	};
}

crate::fork_test! {
	#[test]
	#[should_panic="signal: 11"]
	fn null() {
		let result = unsafe {
			ft_strlen(std::ptr::null())
		};
		assert_eq!(result, 0);
	}
}

test!(basic, "SuperTest");
test!(longer, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer ornare et ipsum et molestie. Sed fermentum metus ut sem imperdiet pretium. Etiam non dolor justo. Nullam dignissim malesuada dui, a malesuada ex facilisis ac. Nullam sit amet odio et neque vestibulum eleifend. Etiam malesuada ultrices orci. Sed quam ligula, pharetra at mattis vitae, mollis et urna. Proin a lobortis elit. Quisque gravida nec lorem ut auctor. In vitae tincidunt arcu. Cras ultricies augue augue, in mattis massa elementum vel.");
test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");

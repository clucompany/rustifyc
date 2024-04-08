
#[macro_export]
macro_rules! define_replace_or_skip {
	[
		[$($current:tt)*][$($end:tt)*]: [[$($n:tt)*][$($v:tt)*]] $($tt:tt)*
	] => {
		// TODO
		/*
			macro_rules! _h {
				[ [$($current)*]: [$($current)*] ]		=> {{ $($v)* }};
				[ [$($current)*]: [$($n)*] ]	=> {
					/* skip */
					$crate::define_replace_or_skip! {
						[$($current)*][$($end)*]: $($tt)*
					}
				};
			};
			
			_h! {
				[$($current)*]: [$($n)*]
			}
			*/
	};
	[ /*END*/
		[$($current:tt)*][$($end:tt)*]:
	] => {
		$($end)*
	};
}

/*
#[cfg(test)]
#[test]
fn __test_define() {
	const __TEST: &'static str = "__TEST";
	const __TEST2: &'static str = "__TEST2";
	
	let test0 = crate::define_replace_or_skip! {
		[__TEST][__TEST]:
		
		[[_V]["_"]]
		[[_T_T_][""]]
		[[__TEST]["test"]]
	};
	assert_eq!(test0, "test");
	
	let test1 = crate::define_replace_or_skip! {
		[__TEST][__TEST2]:
		
		[[_V]["_"]]
		[[_T_T_][""]]
		//[[__TEST]["test"]]
	};
	assert_eq!(test1, __TEST2);
}
 */
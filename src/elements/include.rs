
#[macro_export]
macro_rules! __continue_cparse_include {
	[
		#set_root: [$($set_root:tt)*];
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		<$t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt> $($tt:tt)*
	] => {
		$crate::include_tt! {
			$($econtinue)* {
				$($earg)*
				#include_and_fix_unknown_start_token!([
					$($set_root)* /*$t1 $t2 $t3 $t4*/ $t5 $t6 $t7
				])
				#break_search_macro;
				
				$($tt)*
			}
		}
	};
	[
		#set_root: [$($set_root:tt)*];
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		<$t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt> $($tt:tt)*
	] => {
		$crate::include_tt! {
			$($econtinue)* {
				$($earg)*
				#include_and_fix_unknown_start_token!([
					$($set_root)* /*$t1 $t2 $t3*/ $t4 $t5 $t6
				])
				#break_search_macro;
				
				$($tt)*
			}
		}
	};
	[
		#set_root: [$($set_root:tt)*];
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		<$t1:tt $t2:tt $t3:tt $t4:tt $t5:tt> $($tt:tt)*
	] => {
		$crate::include_tt! {
			$($econtinue)* {
				$($earg)*
				#include_and_fix_unknown_start_token!([
					$($set_root)* /*$t1 $t2*/ $t3 $t4 $t5
				])
				#break_search_macro;
				
				$($tt)*
			}
		}
	};
	[
		#set_root: [$($set_root:tt)*];
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		<$t1:tt $t2:tt $t3:tt $t4:tt> $($tt:tt)*
	] => {
		$crate::include_tt! {
			$($econtinue)* {
				$($earg)*
				#include_and_fix_unknown_start_token!([
					$($set_root)* /*$t1*/ $t2 $t3 $t4
				])
				#break_search_macro;
				
				$($tt)*
			}
		}
	};
	[
		#set_root: [$($set_root:tt)*];
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		<$t1:tt $t2:tt $t3:tt> $($tt:tt)*
	] => {
		$crate::include_tt! {
			$($econtinue)* {
				$($earg)*
				#include_and_fix_unknown_start_token!([
					$($set_root)* $t1 $t2 $t3
				])
				#break_search_macro;
				
				$($tt)*
			}
		}
	};
	[
		#set_root: [$($set_root:tt)*];
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		<$t1:tt $t2:tt> $($tt:tt)*
	] => {
		$crate::include_tt! {
			$($econtinue)* {
				$($earg)*
				#include_and_fix_unknown_start_token!([
					$($set_root)* $t1 $t2
				])
				#break_search_macro;
				
				$($tt)*
			}
		}
	};
	[
		#set_root: [$($set_root:tt)*];
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		<$t1:tt> $($tt:tt)*
	] => {
		$crate::include_tt! {
			$($econtinue)* {
				$($earg)*
				#include_and_fix_unknown_start_token!([
					$($set_root)* $t1
				])
				#break_search_macro;
				
				$($tt)*
			}
		}
	};
}
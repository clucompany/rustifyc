
#[macro_export]
macro_rules! __continue_cparse_ifndef {
	[ // END
		@run[
			[ $($rcontinue:tt)* ] // continue_code
			[ $($rarg:tt)* ] // continue_code_args
		]
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		[ $($ifcode:tt)* ] // c_ifcode
		
		# endif /* end */ $($all:tt)*
	] => {
		$($rcontinue)* {
			$($rarg)*
			$($ifcode)*
		}
		
		$($econtinue)* {
			$($earg)*
			
			$($all)*
		}
	};
	
	
	[ // new tree macros
		@run[
			[ $($rcontinue:tt)* ] // continue_code
			[ $($rarg:tt)* ] // continue_code_args
		]
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		[ $($ifcode:tt)* ] // c_ifcode
			
		# if 0 $($all:tt)*
	] => {
		/*$($rcontinue)* {
			$($rarg)*
			$($ifcode)*
		}*/
		
		$crate::__continue_cparse_ifndef! { //
			@run[
				[ $crate::skip_tt! ] // continue
				[/*empty*/] // continue_arg
			]
			@end[
				[$crate::__continue_cparse_ifndef!] // continue
				[
					@run[
						[ $($rcontinue)* ] // continue
						[ $($rarg)* ] // continue_arg
					]
					@end[
						[ $($econtinue)* ]
						[ $($earg)* ]
					]
					[]
					$($ifcode)*
				] // continue_arg
			]
			
			[]
			$($all)*
		}
	};
	
	[ // new tree macros
		@run[
			[ $($rcontinue:tt)* ] // continue_code
			[ $($rarg:tt)* ] // continue_code_args
		]
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		[ $($ifcode:tt)* ] // c_ifcode
		
		# ifndef $n: ident /* new tree macros */ $($all:tt)*
	] => {
		/*$($rcontinue)* {
			$($rarg)*
			$($ifcode)*
		}*/
		
		$crate::__continue_cparse_ifndef! { // TODO CONFIG SUPPORT
			@run[
				[ $($rcontinue)* ] // continue
				[ $($rarg)* ] // continue_arg
			]
			@end[
				[$crate::__continue_cparse_ifndef!] // continue
				[
					@run[
						[ $($rcontinue)* ] // continue
						[ $($rarg)* ] // continue_arg
					]
					@end[
						[ $($econtinue)* ]
						[ $($earg)* ]
					]
					[]
					$($ifcode)*
				] // continue_arg
			]
			
			[]
			$($all)*
		}
	};
	[
		@run[
			[ $($rcontinue:tt)* ] // continue_code
			[ $($rarg:tt)* ] // continue_code_args
		]
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		[ $($ifcode:tt)* ] // c_ifcode
		$tt:tt /* skip */ 
		
		$($all:tt)*
	] => {
		$crate::__continue_cparse_ifndef! {
			@run[
				[ $($rcontinue)* ] // continue
				[ $($rarg)* ] // continue_arg
			]
			@end[
				[ $($econtinue)* ] // continue_code
				[ $($earg)* ] // continue_code_args
			]
			
			[ $($ifcode)* $tt ] // c_ifcode
			
			$($all)*
		}
	};
	
	[ // end
		@run[
			[ $($rcontinue:tt)* ] // continue_code
			[ $($rarg:tt)* ] // continue_code_args
		]
		@end[
			[ $($econtinue:tt)* ] // continue_code
			[ $($earg:tt)* ] // continue_code_args
		]
		
		[ $($ifcode:tt)* ] // c_ifcode
		/* end */
	] => {
		compile_error!(
			stringify!($($ifcode)*)
		);
		
		$($rcontinue)* {
			$($rarg)*
			$($ifcode)*
		}
		
		$($econtinue)* {
			$($earg)*
		}
	};
	
	[ $($all:tt)+ ] => {
		compile_error!(
			stringify!($($all)*)
		);
	};
	
	[] => {}
}

#[macro_export]
macro_rules! __debug {
	[ $($all:tt)* ] => {
		compile_error!(
			stringify!($($all)*)
		);
	};
}

#[macro_export]
macro_rules! skip_tt {
	[$($all:tt)*] => []
}

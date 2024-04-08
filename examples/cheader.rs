#![recursion_limit = "2048"]

// todo

use rustifyc::include_tt;
use rustifyc::cparse;

#[macro_export]
macro_rules! cinclude {
	[
		#include($path: expr, $n: tt $p: tt $ext: tt);
		
		$($all:tt)*
	] => {
		//const DWC_AC_PINMUX_TOTAL: usize = 0;
		//const DWC_DFI_PINMUX_TOTAL: usize = 0;
		include_tt! {
			pub mod $n {
				const DWC_AC_PINMUX_TOTAL: usize = 0;
				const DWC_DFI_PINMUX_TOTAL: usize = 0;
				
				crate::cparse! {
					#set_include_root[ $path ];
					#include_and_fix_unknown_start_token!([ $path $n $p $ext ])
					
					//@define_debug@
				}
			}
		}
		
		$crate::cinclude! { $($all)* }
	};
	[] => {};
}

cinclude! {
	#include("./examples/arch-g12a/", timing.h);
}

fn main() {
	
}

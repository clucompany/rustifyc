
#[macro_export]
macro_rules! __cparse_transformctypes {
	/* auto array */
	[ $t0: tt $t1:tt $t2:tt[$($c:tt)*] ] =>	[ [$crate::__cparse_transformctypes!( $t0 $t1 $t2 ); $($c)*] ];
	[ $t0: tt $t1:tt [$($c:tt)*] ] =>		[ [$crate::__cparse_transformctypes!( $t0 $t1 ); $($c)*] ];
	[ $t0: tt [$($c:tt)*] ] =>			[ [$crate::__cparse_transformctypes!( $t0 ); $($c)*] ];
	
	/* types */
	[ unsigned char *] => 				[ *const core::ffi::c_uchar ];
	[ unsigned short *] => 				[ *const core::ffi::c_ushort ];
	[ unsigned long *] => 				[ *const core::ffi::c_ulong ];
	[ unsigned int *] => 				[ *const core::ffi::c_uint ];
	[ unsigned char ] => 				[ core::ffi::c_uchar ];
	[ unsigned short ] => 				[ core::ffi::c_ushort ];
	[ unsigned long ] => 				[ core::ffi::c_ulong ];
	[ unsigned int ] => 				[ core::ffi::c_uint ];
	
	[ $(signed)? char *] => 				[ *const core::ffi::c_schar ];
	[ $(signed)? short *] => 			[ *const core::ffi::c_short ];
	[ $(signed)? long *] => 				[ *const core::ffi::c_long ];
	[ $(signed)? int *] => 				[ *const core::ffi::c_int ];
	[ $(signed)? char ] => 				[ core::ffi::c_schar ];
	[ $(signed)? short ] => 				[ core::ffi::c_short ];
	[ $(signed)? long ] => 				[ core::ffi::c_long ];
	[ $(signed)? int ] => 				[ core::ffi::c_int ];
	
	[ uint8_t ] => 					[ core::ffi::c_uchar ];
	[ uint16_t ] => 					[ core::ffi::c_ushort ];
	[ uint32_t ] => 					[ core::ffi::c_uint ];
	
	[ int8_t ] => 						[ core::ffi::c_schar ];
	[ int16_t ] => 					[ core::ffi::c_short ];
	[ int32_t ] => 					[ core::ffi::c_sint ];
	
	[ $($unk:tt)+ ] => {
		/*compile_error!(
			stringify!( $($unk)* )
		)*/
		$($unk)+
	};
	[] => []
}

#[cfg(test)]
#[test]
fn test_ctypes() {
	use std::mem::{size_of_val, size_of};

	let a: __cparse_transformctypes!( signed char [12] ) = Default::default();
	assert_eq!(size_of_val(&a), size_of::<[core::ffi::c_schar; 12]>());
	
	let b: __cparse_transformctypes!( unsigned char [12] ) = Default::default();
	assert_eq!(size_of_val(&b), size_of::<[core::ffi::c_uchar; 12]>());
}

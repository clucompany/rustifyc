
#[macro_export]
macro_rules! __cparse_cstructs_dec_args {
	[
		{
			[ $($nstruct:tt)* ][$( $attr:tt )?]
			[ $($eoktypes:tt)* ]
		}
		
		#if 0
		$($tt:tt)*
	] => {
		$crate::__continue_cparse_ifndef! { // TODO CONFIG SUPPORT
			@run[
				//[ $crate::__debug! ]
				[ $crate::skip_tt! ]
				[ /*empty*/ ]
			]
			@end[
				//[ $crate::__debug! ] // continue
				[ $crate::__cparse_cstructs_dec_args! ]
				[
					{
						[ $($nstruct)* ][$( $attr )?]
						[ $($eoktypes)* ]
					}
				] // continue_arg
			]
			
			[]
			$($tt)*
		}
	};
	
	[
		{
			[ $($nstruct:tt)* ][$( $attr:tt )?]
			[ $($eoktypes:tt)* ]
		}
		
		#[$($meta:tt)*]
		$arg000: tt $arg1: tt $n: tt 
		$([ $($thearrtype: tt)* ])? ;
		
		$($unk:tt)*
	] => {
		$crate::__cparse_cstructs_dec_args! {
			{
				[ $($nstruct)* ][$( $attr )?]
				[
					/*enddata*/
					$($eoktypes)*
					[
						[
							/*#[$($meta)*]
							#[doc = concat!(
								"ctype: ",
								stringify!($arg000 $arg1 $n $([ $($thearrtype)* ])? ; )
							)]*/
						]
						[$n][
							$crate::__cparse_transformctypes! {
								$arg000 $arg1 $([ $($thearrtype)* ])?
							}
						]
					]
				]
			}
			
			$($unk)*
		}
	};
	[
		{
			[ $($nstruct:tt)* ] [ $( $attr:tt )? ]
			[ $($eoktypes:tt)* ]
		}
		
		#[$($meta:tt)*]
		$arg00: tt $n: tt $([ $($thearrtype: tt)* ])? ; 
		
		$($unk:tt)*
	] => {
		$crate::__cparse_cstructs_dec_args! {
			{
				[ $($nstruct)* ][$( $attr )?]
				[
					$($eoktypes)*
					[
						[
							/*#[$($meta)*]
							#[doc = concat!(
								"ctype: ",
								stringify!($arg00 $n $([ $($thearrtype)* ])? ; )
							)]*/
						]
						[$n][$crate::__cparse_transformctypes! {
							$arg00 $([ $($thearrtype)* ])?
						}]
					]
				]
			}
			
			$($unk)*
		}
	};
	
	[
		{
			[ $($nstruct:tt)* ] [$( $attr:tt )?]
			[ $($eoktypes:tt)* ]
		}
		
		//$( #[$($meta:tt)*] )?
		$arg00: tt $n: tt $([ $($thearrtype: tt)* ])? ; 
			
		$($unk:tt)*
	] => {
		$crate::__cparse_cstructs_dec_args! {
			{
				[ $($nstruct)* ][ $( $attr )? ]
				[
					$($eoktypes)*
					[
						[
							//$( #[$($meta)*] )*
							/*#[doc = concat!(
								"ctype: ",
								stringify!($arg00 $n $([ $($thearrtype)* ])? ; )
							)]*/
						]
						[$n] [$crate::__cparse_transformctypes! {
							$arg00 $([ $($thearrtype)* ])?
						}]
					]
				]
			}
			
			$($unk)*
		}
	};
	[
		{
			[ $($nstruct:tt)* ] [ $( $attr:tt )? ]
			[ $($eoktypes:tt)* ]
		}
		
		//$( #[$($meta:tt)*] )?
		$arg000: tt $arg1: tt $n: tt
		$([ $($thearrtype: tt)* ])? ;
		
		$($unk:tt)*
	] => {
		$crate::__cparse_cstructs_dec_args! {
			{
				[ $($nstruct)* ][$( $attr )?]
				[
					$($eoktypes)*
					[
						[
							//$( #[$($meta)*] )*
							/*#[doc = concat!(
								"ctype: ",
								stringify!($arg000 $arg1 $n $([ $($thearrtype)* ])? ; )
							)]*/
						]
						[$n][$crate::__cparse_transformctypes! {
							$arg000 $arg1 $([ $($thearrtype)* ])?
						}]
					]
				]
			}
			
			$($unk)* /*rawdata*/
		}
	};
	
	
	[ // end
		{
			[ $($nstruct:tt)* ] [ $( $attr:tt )? ]
			[
				$([
					[ $( $meta:tt )* ]
					[ $($n: tt)* ] [ $($ty: tt)* ]
				])*
			]
		}
		
		/* empty */
	] => {
		#[allow(non_camel_case_types)]
		#[repr(C)]
		$(#[repr($attr)])?
		$(#[doc = concat!(
			"____attribute____ ((",
			stringify!($attr),
			"))"
		)])?
		
		pub struct $($nstruct)* {
			$(
				//#[allow(non_snake_case)]
				//$( #[$($meta)*] )*
				pub $($n)*: $($ty)*
			),*
		}
		
		/*$crate::new_dlau! {
			$($attr)? struct $nstruct {
				$($n),*
			};
		}*/
	};
	[
		$($unk:tt)+
	] => {
		compile_error!(
			concat!(
				"Unsupported ctype, ",
				stringify!($( $unk )+)
			)
		);
	}
}

#[macro_export]
macro_rules! __cparse_create_cstructs {
	[
		$(@def[ $($define:tt)* ])? /*define*/
		
		struct [
			[ $($name:tt)* ] [ $($attr:tt)? ]
			[ $($struct_data:tt)* ]
		];
		$($all:tt)*
	] => {
		$crate::__cparse_cstructs_dec_args! {
			{
				[ $($name)* ][ $($attr)? ]
				[/*enddata*/]
			}
			
			$($struct_data)*
		}
		
		$crate::__cparse_create_cstructs! {
			$(@def[$($define)*])?
			
			$($all)*
		}
	};
	[
		$(@def[ $($define:tt)* ])? /*define*/
		
		typedef [
			[ $($n:tt)* ]
			[ $($v:tt)* ]
		];
		$($all:tt)*
	] => {
		$crate::define_replace_or_skip!{
			[$($v)*][pub type $($n)* = $crate::__cparse_transformctypes!($($v)*)]:
			
			$($($define)*)?
		}
		//$crate::__cparse_transformctypes!($($v)*);
		
		$crate::__cparse_create_cstructs! {
			$(@def[$($define)*])?
			
			$($all)*
		}
	};
	
	[
		$(@def[ $($define:tt)* ])? /*define*/
	] => []
}

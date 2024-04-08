
extern crate include_tt;
extern crate concat_idents;
pub use include_tt::include_tt as include_tt;
pub use concat_idents::concat_idents as concat_idents;

pub mod elements {
	pub mod include;
	pub mod r#struct;
	pub mod types;
	pub mod ifndef;
	pub mod define;
}

#[macro_export]
macro_rules! cparse {
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		#set_include_root[ $($cset_root:tt)* ];
		$($tt:tt)*
	] => {
		$crate::cparse! {
			$(@def[ $($define)* ])?
			$(@struct[ $($struct)* ])?
			@root[ $($cset_root)* '/' ]
			
			$($tt)*
		}
	};
	/*[
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		# if $t:tt
		
		$($tt:tt)*
	] => {
		$crate::__continue_cparse_ifndef! { // TODO CONFIG SUPPORT
			@[[$crate::cparse!]]
			
			[$($tt)*]
		}
	};*/
	
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		# include $($tt:tt)*
	] => {
		$crate::__continue_cparse_include! {
			#set_root: [$($($set_root)*)?];
			@end [
				[$crate::cparse!]
				[
					$(@def[ $($define)* ])? /*define*/
					$(@struct[ $($struct)* ])? /*struct*/
					$(@root[ $($set_root)* /*set_root*/ ])?
				]
			]
			
			$($tt)*
		}
	};
	
	[ // NEXT
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# if 0
		
		$($tt:tt)*
	] => {
		/*$crate::cparse! {
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}*/
		$crate::__continue_cparse_ifndef! { // TODO CONFIG SUPPORT
			@run[
				//[ $crate::__debug! ]
				[ $crate::skip_tt! ]
				[ /*empty*/ ]
			]
			@end[
				//[ $crate::__debug! ] // continue
				[ $crate::cparse! ]
				[
					$(@def[ $($define)* ])? /*define*/
					$(@struct[ $($struct)* ])? /*struct*/
					$(@root[ $($set_root)* /*set_root*/ ])?
				] // continue_arg
			]
			
			[]
			$($tt)*
		}
	};
	[ // NEXT
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# ifndef $n: ident
		
		$($tt:tt)*
	] => {
		/*$crate::cparse! {
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}*/
		$crate::__continue_cparse_ifndef! { // TODO CONFIG SUPPORT
			@run[
				//[ $crate::__debug! ]
				[ $crate::cparse! ]
				[ $(@root[ $($set_root)* ])? ]
			]
			@end[
				//[ $crate::__debug! ] // continue
				[ $crate::cparse! ]
				[
					$(@def[ $($define)* ])? /*define*/
					$(@struct[ $($struct)* ])? /*struct*/
					$(@root[ $($set_root)* /*set_root*/ ])?
				] // continue_arg
			]
			
			[]
			$($tt)*
		}
	};
	
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# define $n:ident ($($na:tt)*) ($($e:tt)*) $(|| ( $($e2:tt)* ))*
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			@def[
				$($($define)*)*
				[ [$n][ ($($na)*) ($($e)*) $(|| ( $($e2)* ))* ] ]
			]
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}
	};
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# define $n:ident ($($e:tt)*)
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			@def[
				$($($define)*)*
				[ [$n][ ($($e)*) ] ]
			]
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}
	};
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# define $n:ident $e:tt << $e2:tt
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			@def[
				$($($define)*)?
				[ [$n][ $e << $e2 ] ]
			]
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}
	};
	[ // TODO
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# define $n:ident # 
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			@def[
				$($($define)*)?
				[ [$n][ $n ] ]
			]
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* ])?
			
			#$($tt)*
		}
	};
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# define $n:ident $e:tt
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			@def[
				$($($define)*)?
				[ [$n][ $e ] ]
			]
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}
	};
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# define $n:ident
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			@def[
				$($($define)*)?
				[ [$n][ $n ] ]
			]
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}
	};
	
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# elif( $eliff: expr )
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# if( $iff: expr )
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	
	
	/*[ // SKIP
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# ifndef $n: ident
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			$(@root[ $($set_root)* ])?
			
			$($tt)*
		}
	};*/
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# else
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	
	[ // SKIP
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		# endif
		
		$($tt:tt)*
	] => {
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			$(@struct[ $($struct)* ])? /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		$(#[$($meta:tt)*])*
		typedef struct $name:ident $name2:ident;
		
		$($tt:tt)*
	] => {
		//#[allow(non_camel_case_types)]
		//pub type $name2 = $name;
		
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			//$(@struct[ $($struct)* ])? /*struct*/
			@struct[
				$($($struct)*)?
				
				typedef [
					[ $name2 ]
					[ $name ]
				];
			] /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		$(#[$($meta:tt)*])*
		typedef struct $tstruct: ident {
			$($nstruct_data :tt)*
		} __attribute__ (($attr:tt)) $nstruct: ident;
		
		$($tt:tt)*
	] => {
		//#[allow(non_camel_case_types)]
		//pub type $tstruct = $nstruct;
		
		/*$crate::__cparse_cstructs_dec_args! {
			{
				[ $nstruct ][ $attr ]
				[/*enddata*/]
			}
			
			$($struct_data)*
		}*/
		
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			//$(@struct[ $($struct)* ])? /*struct*/
			@struct[
				$($($struct)*)?
				
				struct [
					[ $nstruct ] [ $attr ]
					[ $($nstruct_data)* ]
				];
				typedef [
					[ $tstruct ]
					[ $nstruct ]
				];
			] /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		$(#[$($meta:tt)*])*
		typedef struct $tstruct: ident {
			$($nstruct_data :tt)*
		} $nstruct: ident;
		
		$($tt:tt)*
	] => {
		/*#[allow(non_camel_case_types)]
		pub type $tstruct = $nstruct;
		
		$crate::__cparse_cstructs_dec_args! {
			{
				[ $nstruct ][ ]
				[/*enddata*/]
			}
			
			$($nstruct_data)*
		}*/
		
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			//$(@struct[ $($struct)* ])? /*struct*/
			@struct[
				$($($struct)*)?
				
				struct [
					[ $nstruct ] [  ]
					[ $($nstruct_data)* ]
				];
				typedef [
					[ $tstruct ]
					[ $nstruct ]
				];
			] /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		$(#[$($meta:tt)*])*
		typedef $t1:tt $t2:tt;
		
		$($tt:tt)*
	] => {
		//#[allow(non_camel_case_types)]
		//pub type $t2 = $crate::__cparse_transformctypes!($t1);
		
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			//$(@struct[ $($struct)* ])? /*struct*/
			@struct[
				$($($struct)*)?
				
				typedef [
					[ $t2 ]
					[ $t1 ]
				];
			] /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		$(#[$($meta:tt)*])*
		typedef $t1:tt $t2:tt $t3:ident;
		
		$($tt:tt)*
	] => {
		//#[allow(non_camel_case_types)]
		//pub type $t3 = $crate::__cparse_transformctypes!($t1 $t2);
		
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			//$(@struct[ $($struct)* ])? /*struct*/
			@struct[
				$($($struct)*)?
				
				typedef [
					[ $t3 ]
					[ $t1 $t2 ]
				];
			] /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		$(#[$($meta:tt)*])*
		typedef $t1:tt $t2:tt $t3:tt $t4:ident;
		
		$($tt:tt)*
	] => {
		//#[allow(non_camel_case_types)]
		//pub type $t4 = $crate::__cparse_transformctypes!($t1 $t2 $t3);
		
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			//$(@struct[ $($struct)* ])? /*struct*/
			@struct[
				$($($struct)*)?
				
				typedef [
					[ $t4 ]
					[ $t1 $t2 $t3 ]
				];
			] /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	
	[
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
		
		$(#[$($meta:tt)*])*
		struct $nstruct: ident {
			$($struct_data :tt)*
		} $(__attribute__ (($attr:tt)) )? ;
		
		$($tt:tt)*
	] => {
		/*$crate::__cparse_cstructs_dec_args! {
			{
				[ $nstruct ][ $($attr)? ]
				[/*enddata*/]
			}
			
			$($struct_data)*
		}*/
		
		$crate::cparse! {
			$(@def[ $($define)* ])? /*define*/
			@struct[
				$($($struct)*)?
				
				struct [
					[ $($nstruct)* ] [ $($attr)? ]
					[ $($nstruct_data)* ]
				];
			] /*struct*/
			$(@root[ $($set_root)* /*set_root*/ ])?
			
			$($tt)*
		}
	};
	
	[ /* end */
		$(@def[ $($define:tt)* ])? /*define*/
		$(@struct[ $($struct:tt)* ])? /*struct*/
		$(@root[ $($set_root:tt)* /*set_root*/ ])?
	] => {
		$crate::__cparse_create_cstructs!{
			$(@def[ $($define)* ])?
			
			$($($struct)*)?
		}
	};
	[ /* end */
		$($unk:tt)+
	] => {
		compile_error!(
			stringify!($($unk)+)
		);
	};
	
	
	[] => []
}


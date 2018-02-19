#[macro_export]
macro_rules! unit {
	( $val:tt pt) => (
		$val.point()
	);
	( $val:tt %) => {
		$val.percent()
	};
	( $val:expr) => {
		$val
	};
}

#[macro_export]
macro_rules! flex_style {
	// Manually match on styles which require an OrderedFloat
	// This way the styles like
	//     Flex(1.0)
	// will be converted to:
	//     Flex(1.0.into())
	(AspectRatio($val:expr)) => (
		AspectRatio($val.into())
	);
	(BorderBottom($val:expr)) => (
		BorderBottom($val.into())
	);
	(BorderEnd($val:expr)) => (
		BorderEnd($val.into())
	);
	(BorderLeft($val:expr)) => (
		BorderLeft($val.into())
	);
	(BorderRight($val:expr)) => (
		BorderRight($val.into())
	);
	(BorderStart($val:expr)) => (
		BorderStart($val.into())
	);
	(BorderTop($val:expr)) => (
		BorderTop($val.into())
	);
	(Border($val:expr)) => (
		Border($val.into())
	);
	(Flex($val:expr)) => (
		Flex($val.into())
	);
	(FlexGrow($val:expr)) => (
		FlexGrow($val.into())
	);
	(FlexShrink($val:expr)) => (
		FlexShrink($val.into())
	);
	($s:ident($($unit:tt)*)) => (
		$s(unit!($($unit)*))
	);
}

#[macro_export]
macro_rules! style {
	( $x:expr, $($s:tt($($unit:tt)*)),* ) => {
		$x.apply_styles(&vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		))
	};
}

#[macro_export]
macro_rules! make_styles {
	( $($s:tt($($unit:tt)*)),* ) => {
		vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		)
	};
}

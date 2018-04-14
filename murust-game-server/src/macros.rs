macro_rules! closet {
  (@as_expr $e:expr) => { $e };

  ([$($var:ident),*] $cl:expr) => {{
    $(let $var = $var.clone();)*
      $cl
  }};
}

#[macro_export]
macro_rules! view {
    (($ty:expr; $($prop_id:ident={$prop:expr})* $({$($children:tt)*})?)) => ({
      let component = $ty;
      $(let component = component.$prop_id($prop);)*
      $($(let component = component.push(view!($children));)*)?
      component
    })
}

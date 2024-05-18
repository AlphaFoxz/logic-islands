// #[macro_export]
// macro_rules! set_gd_property {
//     ($obj:ty, $k:literal, $v:ty) => {
//         $godot::obj::Gd::<Island>::to_variant(&obj)
//         return $obj.set_meta(StringName::from($k), $v);
//     };
// }

#[macro_export]
macro_rules! prop_name {
    ($obj:expr, $k:tt) => {
        // godot::obj::Gd::<Island>::to_variant(&obj)
        $obj.get_meta(StringName::from($k))
    };
}

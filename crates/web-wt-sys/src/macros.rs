//! Macros.

/// Define a dictionary constructor.
#[macro_export]
macro_rules! dictionary_constructor {
    ($name:ident) => {
        impl $name {
            /// Create a new instance.
            pub fn new() -> Self {
                ::js_sys::Object::new().unchecked_into()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _dictionary_erase_type_option_jsvalue {
    (Option<JsValue>) => {
        JsValue
    };

    ($t:ty) => {
        $t
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _dictionary_type_non_wasm_bindgen_accessors {
    ($attr:ident, JsValue, $js_name:ident) => {
        ::paste::paste! {
            #[doc = "Set `" $js_name "` field to the given value."]
            pub fn [<set_ $attr>](&self, val: JsValue) {
                self.[<set_option_ $attr>](val)
            }

            #[doc = "Unset `" $js_name "` field."]
            pub fn [<unset_ $attr>](&self) {
                self.[<set_option_ $attr>](JsValue::NULL)
            }
        }
    };

    ($attr:ident, $value_type:ty, $js_name:ident) => {
        ::paste::paste! {
            #[doc = "Set `" $js_name "` field to the given value."]
            pub fn [<set_ $attr>](&self, val: $value_type) {
                self.[<set_option_ $attr>](Some(val))
            }

            #[doc = "Unset `" $js_name "` field."]
            pub fn [<unset_ $attr>](&self) {
                self.[<set_option_ $attr>](None)
            }
        }
    };
}

/// Define a dictionary type.
#[macro_export]
macro_rules! dictionary_type {
    (
        $( #[$attrs:meta] )*
        pub type $name:ident {
            $($attr:ident: $value_type:ty => $js_name:ident)*
        }
    ) => {
        ::paste::paste! {
            #[wasm_bindgen]
            extern "C" {
                #[doc = "[`" $name "`] dictionary type."]
                $(#[$attrs])*
                #[wasm_bindgen(extends = ::js_sys::Object, js_name = Object)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub type $name;

                $(
                    #[doc = "Get `" $js_name "` field value, if set."]
                    #[wasm_bindgen(method, getter = $js_name)]
                    pub fn [<get_ $attr>](this: &$name) -> $crate::_dictionary_erase_type_option_jsvalue!(Option<$value_type>);

                    #[doc = "Set `" $js_name "` field to the given value or unset it."]
                    #[wasm_bindgen(method, setter = $js_name)]
                    pub fn [<set_option_ $attr>](this: &$name, val: $crate::_dictionary_erase_type_option_jsvalue!(Option<$value_type>));
                )*
            }

            impl $name {
                $(
                    $crate::_dictionary_type_non_wasm_bindgen_accessors!($attr, $value_type, $js_name);
                )*
            }
        }

        $crate::dictionary_constructor!($name);
    };
}

/// Define a set_option_... accessors.
#[macro_export]
macro_rules! set_option_accessors {
    (
        $( #[$attrs:meta] )*
        $attr:ident: $value_type:ty
    ) => {
        ::paste::paste! {
            /// Set field to the given value.
            $( #[$attrs] )*
            pub fn [<set_ $attr>](&self, val: $value_type) {
                self.[<set_option_ $attr>](Some(val))
            }

            /// Unset field.
            $( #[$attrs] )*
            pub fn [<unset_ $attr>](&self) {
                self.[<set_option_ $attr>](None)
            }
        }
    };
}

/// Define a set_option_... accessors for a fallible assignment.
#[macro_export]
macro_rules! set_option_accessors_fallible {
    (
        $( #[$attrs:meta] )*
        $attr:ident: $value_type:ty => $error_type:ty
    ) => {
        ::paste::paste! {
            /// Set field to the given value.
            $( #[$attrs] )*
            pub fn [<set_ $attr>](&self, val: $value_type) -> Result<(), $error_type> {
                self.[<set_option_ $attr>](Some(val))
            }

            /// Unset field.
            $( #[$attrs] )*
            pub fn [<unset_ $attr>](&self) -> Result<(), $error_type> {
                self.[<set_option_ $attr>](None)
            }
        }
    };
}

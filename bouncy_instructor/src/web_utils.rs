#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! println {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! println {
    ( $( $t:tt )* ) => {
        println!( $( $t )* );
    }
}

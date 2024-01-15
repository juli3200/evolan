use cpp;
#[link(name = "live_view")]
extern {
    pub fn hello();
}

cpp!{{
    #include "src/live_view/live_view.h"
}}
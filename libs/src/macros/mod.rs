// libs/cubeos-common/src/macros/mod.rs

#[macro_export]
macro_rules! define_command_id {
    ($($cmd:ident),*) => {
        // 这里放 command_id! 宏内部的逻辑
        // ...
    };
}

// 原先的 app_macro!/service_macro! 有重复逻辑
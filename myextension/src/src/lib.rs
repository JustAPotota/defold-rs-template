const MODULE_NAME: &str = "myextension";

// Import Defold's SDK
use dmsdk::*;

// `#[no_mangle] extern "C"` is boilerplate to make sure Defold can run the function
#[no_mangle]
extern "C" fn reverse(l: lua::State) -> i32 {
    // Grab the given string from the Lua stack
    // Note that most functions using `lua::State` are unsafe
    let to_reverse = unsafe { lua::check_string(l, 1) };

    // Reverse the string
    let reversed: String = to_reverse.chars().rev().collect();

    // Push the newly reversed string back onto the Lua stack
    unsafe {
        lua::push_string(l, &reversed);
    }

    1
}

// This is an array of tuples where the first element is the function name,
// and the second is the Rust function it refers to
const EXTENSION_FUNCTIONS: lua::Reg = &[("reverse", reverse)];

fn lua_init(l: lua::State) {
    unsafe {
        let top = lua::get_top(l);

        // Register our new module
        lua::register(l, MODULE_NAME, EXTENSION_FUNCTIONS);

        lua::pop(l, 1);
        assert_eq!(top, lua::get_top(l));
    }
}

#[no_mangle]
extern "C" fn app_init(params: dmextension::AppParams) -> i32 {
    dmlog::info("MYEXTENSION", "app_init");
    dmextension::RESULT_OK
}

#[no_mangle]
extern "C" fn ext_init(params: dmextension::Params) -> i32 {
    unsafe {
        // Here we need to dereference `params` (which is a pointer), so we again use `unsafe`
        let lua_state = (*params).m_L;
        lua_init(lua_state);
    }
    dmlog::info("MYEXTENSION", "Registered my extension");
    dmextension::RESULT_OK
}

#[no_mangle]
extern "C" fn app_final(params: dmextension::AppParams) -> i32 {
    dmlog::info("MYEXTENSION", "app_final");
    dmextension::RESULT_OK
}

#[no_mangle]
extern "C" fn ext_final(params: dmextension::Params) -> i32 {
    dmlog::info("MYEXTENSION", "ext_final");
    dmextension::RESULT_OK
}

#[no_mangle]
extern "C" fn on_update(params: dmextension::Params) -> i32 {
    dmlog::info("MYEXTENSION", "on_update");
    dmextension::RESULT_OK
}

#[no_mangle]
extern "C" fn on_event(params: dmextension::Params, event: dmextension::Event) {
    let event_id = unsafe { (*event).m_Event };

    match event_id {
        0 => dmlog::info("MYEXTENSION", "on_event - EVENT_ID_ACTIVATE_APP"),
        1 => dmlog::info("MYEXTENSION", "on_event - EVENT_ID_DEACTIVATE_APP"),
        2 => dmlog::info("MYEXTENSION", "on_event - EVENT_ID_ICONFIY_APP"),
        3 => dmlog::info("MYEXTENSION", "on_event - EVENT_ID_DEICONIFY_APP"),
        _ => dmlog::warning("MYEXTENSION", "on_event - Unknown event ID"),
    };
}

// Defold's SDK uses a macro for setting up extension entry points:
//
// declare_extension!(symbol, app_init, app_final, init, update, on_event, final)
//
// The symbol (`MY_EXTENSION` in this example) must match the name in `ext.manifest`
declare_extension!(
    MY_EXTENSION,
    Some(app_init),
    Some(app_final),
    Some(ext_init),
    Some(ext_final),
    Some(on_update),
    Some(on_event)
);

const MODULE_NAME: &str = "myextension";

// Import Defold's SDK
use dmsdk::*;

// Also import this enum for later
use dmextension::Event;

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

// We're putting an underscore before `params` here to tell Rust that we're not using it.
// If we don't, the compiler will warn about unused variables.
fn app_init(_params: dmextension::AppParams) -> dmextension::Result {
    dmsdk::info!("app_init");
    dmextension::Result::Ok
}

fn app_final(_params: dmextension::AppParams) -> dmextension::Result {
    dmsdk::info!("app_final");
    dmextension::Result::Ok
}

fn ext_init(params: dmextension::Params) -> dmextension::Result {
    let lua_state = params.l;
    lua_init(lua_state);
    dmsdk::info!("Registered my extension");
    dmextension::Result::Ok
}

fn ext_final(_params: dmextension::Params) -> dmextension::Result {
    dmsdk::info!("ext_final");
    dmextension::Result::Ok
}

fn on_update(_params: dmextension::Params) -> dmextension::Result {
    dmsdk::info!("on_update");
    dmextension::Result::Ok
}

fn on_event(_params: dmextension::Params, event: dmextension::Event) {
    match event {
        // Since we put `use dmextension::Event;` at the beginning,
        // we only need to write `Event::ActivateApp` instead of `dmextension::Event::ActivateApp`
        Event::ActivateApp => dmsdk::info!("App activated!"),
        Event::DeactivateApp => dmsdk::info!("App deactivated!"),
        Event::IconifyApp => dmsdk::info!("App iconified!"),
        Event::DeiconifyApp => dmsdk::info!("App deiconified!"),
        Event::Unknown => dmsdk::warning!("Received unknown event!"),
    };
}

// Defold's SDK uses a macro for setting up extension entry points:
//
// declare_extension!(symbol, app_init, app_final, init, update, on_event, final)
//
// The symbol (`MY_EXTENSION` in this example) must match the name in `ext.manifest`.
declare_extension!(
    MY_EXTENSION,
    Some(app_init),
    Some(app_final),
    Some(ext_init),
    Some(ext_final),
    Some(on_update),
    Some(on_event)
);

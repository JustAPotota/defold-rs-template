// Import Defold's SDK
use dmsdk::*;

// Also import this enum for later
use dmextension::Event;

fn reverse(l: lua::State) -> i32 {
    // Grab the given string from the Lua stack
    let to_reverse = lua::check_string(l, 1);

    // Reverse the string
    let reversed: String = to_reverse.chars().rev().collect();

    // Push the newly reversed string back onto the Lua stack
    lua::push_string(l, &reversed);

    1
}

// This is the name of our module in Lua
const MODULE_NAME: &str = "myextension";

// This macro creates a constant named `EXTENSION_FUNCTIONS` that we can give to `lua::register()`
// The functions in Lua will be named the same as in Rust, so our `reverse` function will become `myextension.reverse()`
//
// Adding more functions is easy:
// declare_functions!(EXTENSION_FUNCTIONS, reverse, my_function, other_function);
declare_functions!(EXTENSION_FUNCTIONS, reverse);

// Tip: In VS Code, you can run the "Exapand macro recursively" command to see what the code looks like!

fn lua_init(l: lua::State) {
    let top = lua::get_top(l);

    // Register our new module
    lua::register(l, MODULE_NAME, EXTENSION_FUNCTIONS);

    lua::pop(l, 1);
    assert_eq!(top, lua::get_top(l));
}

// We're putting an underscore before `params` here to tell Rust that we're not using it
// If we don't, the compiler will warn about unused variables
fn app_init(_params: dmextension::AppParams) -> dmextension::Result {
    dmlog::info!("app_init");
    dmextension::Result::Ok
}

fn app_final(_params: dmextension::AppParams) -> dmextension::Result {
    dmlog::info!("app_final");
    dmextension::Result::Ok
}

fn ext_init(params: dmextension::Params) -> dmextension::Result {
    let lua_state = params.l;
    lua_init(lua_state);
    dmlog::info!("Registered my extension");
    dmextension::Result::Ok
}

fn ext_final(_params: dmextension::Params) -> dmextension::Result {
    dmlog::info!("ext_final");
    dmextension::Result::Ok
}

fn on_update(_params: dmextension::Params) -> dmextension::Result {
    dmlog::info!("on_update");
    dmextension::Result::Ok
}

fn on_event(_params: dmextension::Params, event: dmextension::Event) {
    match event {
        // Since we put `use dmextension::Event;` at the beginning,
        // we only need to write `Event::ActivateApp` instead of `dmextension::Event::ActivateApp`
        Event::ActivateApp => dmlog::info!("App activated!"),
        Event::DeactivateApp => dmlog::info!("App deactivated!"),
        Event::IconifyApp => dmlog::info!("App iconified!"),
        Event::DeiconifyApp => dmlog::info!("App deiconified!"),
        Event::Unknown => dmlog::warning!("Received unknown event!"),
    };
}

// Defold's SDK includes a macro for setting up extension entry points:
//
// declare_extension!(symbol, app_init, app_final, init, update, on_event, final)
//
// The symbol (`MY_EXTENSION` in this case) must match the name in `ext.manifest`
declare_extension!(
    MY_EXTENSION,
    Some(app_init),
    Some(app_final),
    Some(ext_init),
    Some(ext_final),
    Some(on_update),
    Some(on_event)
);

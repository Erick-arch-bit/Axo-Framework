use mlua::prelude::*;

pub fn register_functions(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    globals.set("log", lua.create_function(|_, msg: String| {
        println!("[Lua] {}", msg);
        Ok(())
    })?)?;

    globals.set("print", lua.create_function(|_, msg: String| {
        println!("[Lua] {}", msg);
        Ok(())
    })?)?;

    Ok(())
}

/// This example is fully copied from the mlua documentation
/// published under the MIT License
/// 
/// https://github.com/mlua-rs/mlua/tree/3ca7b4942ed6ccad816883a2a88582705916c917
use mlua::prelude::*;

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    lua.globals().set("map_table", map_table)?;

    lua.load("for k,v in pairs(map_table) do print(k,v) end").exec()?;

    Ok(())
}
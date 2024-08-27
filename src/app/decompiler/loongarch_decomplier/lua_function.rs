use td_rlua::Lua;

pub fn load_lua_functions(lua: &mut Lua) {
    lua.set("strlen", td_rlua::function1(lua_strlen));    
    lua.set("strat", td_rlua::function2(lua_strat));
}

fn lua_strlen(s: String) -> usize {
    s.len()
}

fn lua_strat(s: String, i: usize) -> u8 {
    s.as_bytes()[i]
} 


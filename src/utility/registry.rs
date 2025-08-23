use std::collections::HashMap;

pub fn get_registry() -> HashMap<&'static str, &'static str> {
    let mut registry = HashMap::new();

    registry.insert("fmt", "https://github.com/fmtlib/fmt.git");
    registry.insert("catch2", "https://github.com/catchorg/Catch2.git");
    registry.insert("spdlog", "https://github.com/gabime/spdlog.git");
    registry.insert("googletest", "https://github.com/google/googletest.git");

    registry
}

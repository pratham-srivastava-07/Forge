#include "lib.hpp"

namespace {{project_name}} {

std::string hello() {
    return "Hello from {{project_name}} library!";
}

std::string greet(const std::string& name) {
    return "Hello, " + name + " from {{project_name}} library!";
}

} // namespace {{project_name}}

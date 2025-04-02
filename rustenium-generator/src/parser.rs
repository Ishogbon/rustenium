
use regex::Regex;

pub fn parse_bs_file(content: &str) {
    // Regex to find module declarations (unchanged)
    let re_module = Regex::new(r"##\s+(?:The\s+)?(\w+)\s+Module\s+##\s+\{#module-([^}]+)\}").unwrap();

    // New regex to find ALL type definitions under a module
    let re_type = Regex::new(r"####\s+The\s+([^\s.]+)\.([^\s]+)\s+Type\s+####\s+\{#type-([^}]+)-([^}]+)\}").unwrap();

    for caps in re_module.captures_iter(content) {
        let module_keyword = caps.get(1).unwrap().as_str();
        println!("Found module: '{}'", module_keyword);

        // Now find all types belonging to this module
        for type_caps in re_type.captures_iter(content) {
            let type_module = type_caps.get(1).unwrap().as_str();
            let type_name = type_caps.get(2).unwrap().as_str();

            if type_module == module_keyword {
                println!("Found type: '{}.{}'", module_keyword, type_name);
            }
        }
    }
}
pub mod parser_utils {
    use std::collections::HashMap;


    pub fn check_fields(map: HashMap<String, String>, mandatory_field: Vec<&str>) -> bool {

        let mut has_all_fields = true;
        for field in mandatory_field {
            if map.contains_key(field) == false  {
                has_all_fields = false;
            }
        }

        has_all_fields
    }
}

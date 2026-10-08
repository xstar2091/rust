pub(crate) struct CommonUtils {}

impl CommonUtils {
    pub(crate) fn process_database_table_name(table_name: String) -> String {
        table_name
            .split('_')
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<String>()
    }

    pub(crate) fn generate_row_class_name(table_name: String) -> String {
        Self::process_database_table_name(table_name) + "Row"
    }

    pub(crate) fn generate_table_class_name(table_name: String) -> String {
        Self::process_database_table_name(table_name) + "Table"
    }
}

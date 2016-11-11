use nautilus_extension::{Column, ColumnProvider};

pub struct TmsuColumnProvider {

}

impl ColumnProvider for TmsuColumnProvider {
    fn get_columns(&self) -> Vec<Column> {
        vec![
            Column {
                name: "TmsuNautilusExtension::tmsu_tags_column".to_string(),
                attribute: "tmsu_tags".to_string(),
                label: "TMSU tags".to_string(),
                description: "List of TMSU tags".to_string(),
            }
        ]
    }
}

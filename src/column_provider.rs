use nautilus_extension::{Column, ColumnProvider};

pub struct TmsuColumnProvider {}

impl ColumnProvider for TmsuColumnProvider {
    fn get_columns(&self) -> Vec<Column> {
        vec![Column::new(
            "TmsuNautilusExtension::tmsu_tags_column",
            "tmsu_tags",
            "TMSU tags",
            "List of TMSU tags",
        )]
    }
}

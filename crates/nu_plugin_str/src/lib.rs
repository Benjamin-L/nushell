mod nu;
mod strutils;

pub use strutils::Str;

#[cfg(test)]
mod tests {
    use super::Str;
    use crate::strutils::Action;
    use nu_protocol::Value;
    use nu_value_ext::ValueExt;

    impl Str {
        pub fn expect_action(&self, action: Action) {
            match &self.action {
                Some(set) if set == &action => {}
                Some(other) => panic!(format!("\nExpected {:#?}\n\ngot {:#?}", action, other)),
                None => panic!(format!("\nAction {:#?} not found.", action)),
            }
        }

        pub fn expect_field(&self, field: Value) {
            let field = match field.as_column_path() {
                Ok(column_path) => column_path,
                Err(reason) => panic!(format!(
                    "\nExpected {:#?} to be a ColumnPath, \n\ngot {:#?}",
                    field, reason
                )),
            };

            match &self.field {
                Some(column_path) if column_path == &field => {}
                Some(other) => panic!(format!("\nExpected {:#?} \n\ngot {:#?}", field, other)),
                None => panic!(format!("\nField {:#?} not found.", field)),
            }
        }
    }
}

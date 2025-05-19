use crate::orm::entity::Entity;

pub struct Evento {
    pub id: i32,
    pub titulo: String,
    pub data: String, // poderia ser datetime com outra lib
}

impl Entity for Evento {
    fn table_name() -> &'static str {
        "meu_keyspace.eventos"
    }

    fn fields() -> Vec<&'static str> {
        vec!["id", "titulo", "data"]
    }

    fn values(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            format!("'{}'", self.titulo),
            format!("'{}'", self.data),
        ]
    }

    fn update_values(&self) -> Vec<(String, String)> {
        vec![
            ("titulo".to_string(), format!("'{}'", self.titulo)),
            ("data".to_string(), format!("'{}'", self.data)),
        ]
    }

    fn field_definitions() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "int"),
            ("titulo", "text"),
            ("data", "text"),
        ]
    }

    fn primary_key() -> &'static str {
        "id"
    }
}

use crate::orm::entity::Entity;
pub struct Produto {
    pub id: i32,
    pub nome: String,
    pub preco: f64,
}

impl Entity for Produto {
    fn table_name() -> &'static str {
        "meu_keyspace.produtos"
    }

    fn fields() -> Vec<&'static str> {
        vec!["id", "nome", "preco"]
    }

    fn values(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            format!("'{}'", self.nome),
            self.preco.to_string(),
        ]
    }

    fn update_values(&self) -> Vec<(String, String)> {
        vec![
            ("nome".to_string(), format!("'{}'", self.nome)),
            ("preco".to_string(), self.preco.to_string()),
        ]
    }

    fn field_definitions() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "int"),
            ("nome", "text"),
            ("preco", "double"),
        ]
    }

    fn primary_key() -> &'static str {
        "id"
    }
}

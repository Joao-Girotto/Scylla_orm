use crate::orm::entity::Entity;
use crate::orm::entity::Listable;

pub struct Cliente {
    pub id: i32,
    pub nome: String,
    pub email: String,
}

impl Entity for Cliente {
    fn table_name() -> &'static str {
        "meu_keyspace.clientes"
    }

    fn fields() -> Vec<&'static str> {
        vec!["id", "nome", "email"]
    }

    fn values(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            format!("'{}'", self.nome),
            format!("'{}'", self.email),
        ]
    }

    fn update_values(&self) -> Vec<(String, String)> {
        vec![
            ("nome".to_string(), format!("'{}'", self.nome)),
            ("email".to_string(), format!("'{}'", self.email)),
            ]
    }

    fn field_definitions() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "int"),
            ("nome", "text"),
            ("email", "text"),
        ]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    
}

impl Listable for Cliente {
    type Row = (i32, String, String);

    fn format_row((id, nome, email): Self::Row) {
        println!("Cliente ID: {}, Nome: {}, Email: {}", id, nome, email);
    }
}

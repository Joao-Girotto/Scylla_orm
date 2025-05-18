use crate::orm::entity::Entity;


pub fn create_table<T: Entity>() -> String {
    let table = T::table_name();
    let fields: Vec<String> = T::field_definitions()
        .into_iter()
        .map(|(nome, tipo)| format!("{} {}", nome, tipo))
        .collect();
    let primary_key = T::primary_key();
    format!(
        "CREATE TABLE IF NOT EXISTS {} ({}, PRIMARY KEY ({}));",
        table,
        fields.join(", "),
        primary_key
    )
}

pub fn insert<T: Entity>(entity: &T) -> String {
    let table = T::table_name();
    let fields = T::fields().join(", ");
    let values = entity.values().join(", ");
    format!("INSERT INTO {} ({}) VALUES ({});", table, fields, values)
}


pub fn update_by_id<T: Entity>(entity: &T, id: i32) -> String {
    let updates: Vec<String> = entity
        .update_values()
        .into_iter()
        .map(|(campo, valor)| format!("{} = {}", campo, valor))
        .collect();

    format!(
        "UPDATE {} SET {} WHERE {} = {};",
        T::table_name(),
        updates.join(", "),
        T::primary_key(),
        id
    )
}


pub fn delete_by_id<T: Entity>(id: i32) -> String {
    let table = T::table_name();
    format!("DELETE FROM {} WHERE id = {};", table, id)
}

pub fn select_by_id<T: Entity>(id: i32) -> String {
    let table = T::table_name();
    format!("SELECT * FROM {} WHERE id = {};", table, id)
}

pub fn select_all<T: Entity>() -> String {
    format!("SELECT * FROM {};", T::table_name())
}

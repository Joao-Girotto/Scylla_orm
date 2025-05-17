pub trait Entity {
    fn table_name() -> &'static str;
    fn fields() -> Vec<&'static str>;
    fn values(&self) -> Vec<String>;

    /// Retorna pares (nome, tipo) das colunas para CREATE TABLE
    fn field_definitions() -> Vec<(&'static str, &'static str)>;

    /// Nome da coluna que é chave primária
    fn primary_key() -> &'static str;
}

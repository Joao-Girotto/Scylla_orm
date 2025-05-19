use scylla::{Session, SessionBuilder};

pub async fn connect(uri: &str) -> Session {
    SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .expect("Erro ao conectar ao ScyllaDB")
}

pub async fn executar_query(session: &Session, query: &str) {
    if let Err(e) = session.query(query, &[]).await {
        eprintln!("Erro ao executar query: {}", e);
    }
}


pub async fn init_keyspace(session: &Session, keyspace: &str) {
    let query = format!(
        "CREATE KEYSPACE IF NOT EXISTS {} \
         WITH replication = {{ 'class': 'SimpleStrategy', 'replication_factor': 1 }};",
        keyspace
    );

    if let Err(e) = session.query(query, &[]).await {
        eprintln!("Erro ao criar keyspace {}: {}", keyspace, e);
    }
}
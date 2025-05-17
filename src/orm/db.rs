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

use scylla_orm::orm::db;
use scylla_orm::orm::query_builder;
use scylla_orm::models::cliente::Cliente;
use scylla::IntoTypedRows;


async fn listar_clientes(session: &scylla::Session) {
    let query = query_builder::select_all::<Cliente>();
    match session.query(query, &[]).await {
        Ok(result) => {
            if let Some(rows) = result.rows {
                for row in rows.into_typed::<(i32, String, String)>() {
                    match row {
                        Ok((id, nome, email)) => {
                            println!("Cliente ID: {}, Nome: {}, Email: {}", id, nome, email);
                        }
                        Err(e) => {
                            println!("Erro ao converter linha: {}", e);
                        }
                    }
                }
            } else {
                println!("Nenhum dado encontrado.");
            }
        }
        Err(e) => {
            println!("Erro ao executar SELECT: {}", e);
        }
    }
}



#[tokio::main]
async fn main() {
    let session = db::connect("127.0.0.1:9042").await;

    // Criar keyspace se ainda não existir
    let create_keyspace = "
        CREATE KEYSPACE IF NOT EXISTS meu_keyspace
        WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};
    ";
    db::executar_query(&session, create_keyspace).await;

    // Não precisa usar `USE`, pois o keyspace está embutido nas queries

    let create = query_builder::create_table::<Cliente>();
    db::executar_query(&session, &create).await;

    let cliente = Cliente {
        id: 1,
        nome: "João".to_string(),
        email: "joao@email.com".to_string(),
    };
    let insert = query_builder::insert(&cliente);
    db::executar_query(&session, &insert).await;
    listar_clientes(&session).await;
}

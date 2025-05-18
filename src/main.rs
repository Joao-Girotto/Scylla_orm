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
                            println!("Cliente ID: {}, Nome: {}, Email: {}", id, email, nome);
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


    let cliente_one = Cliente {
        id: 2,
        nome: "Marcos".to_string(),
        email: "marcos@email.com".to_string(),
    };

    let cliente_atualizado = Cliente {
        id: 2,
        nome: "Rodrigo Hubner".to_string(),
        email: "rhubner@email.com".to_string(),
    };
    let update = query_builder::update_by_id(&cliente_atualizado, cliente_atualizado.id);
    db::executar_query(&session, &update).await;

    // Deletar cliente
    let delete = query_builder::delete_by_id::<Cliente>(1);
    db::executar_query(&session, &delete).await;
    let insert = query_builder::insert(&cliente);
    db::executar_query(&session, &insert).await;
    listar_clientes(&session).await;
}

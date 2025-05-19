// IMPORTS
use scylla_orm::orm::db;
use scylla_orm::orm::query_builder;
use scylla::Session;
use scylla_orm::models::cliente::Cliente;
use scylla_orm::models::evento::Evento;
use scylla_orm::models::produto::Produto;
use scylla_orm::orm::entity::Listable;
use scylla::IntoTypedRows;

//Função para listar entidades
// Essa função é genérica e pode ser usada para qualquer entidade que implemente o trait Listable
pub async fn listar_entidades<T: Listable>(session: &Session) {
    let query = query_builder::select_all::<T>();

    match session.query(query, &[]).await {
        Ok(result) => {
            if let Some(rows) = result.rows {
                for row in rows.into_typed::<T::Row>() {
                    match row {
                        Ok(data) => T::format_row(data),
                        Err(e) => println!("Erro ao converter linha: {}", e),
                    }
                }
            } else {
                println!("Nenhum registro encontrado para a tabela {}", T::table_name());
            }
        }
        Err(e) => println!("Erro ao listar {}: {}", T::table_name(), e),
    }
}

#[tokio::main]
async fn main() {
    // Conexão e inicialização
    let session = db::connect("127.0.0.1:9042").await;
    db::init_keyspace(&session, "meu_keyspace").await;

    // Criação das tabelas
    db::executar_query(&session, &query_builder::create_table::<Cliente>()).await;
    db::executar_query(&session, &query_builder::create_table::<Produto>()).await;
    db::executar_query(&session, &query_builder::create_table::<Evento>()).await;

    // --- TESTES DE CRUD: CLIENTE ---
    let cliente = Cliente {
        id: 1,
        nome: "João".to_string(),
        email: "joao@email.com".to_string(),
    };
    let cliente_para_update = Cliente {
        id: 1,
        nome: "Marcio".to_string(),
        email: "marcio@email.com".to_string(),
    };
     let cliente_extra = Cliente {
        id: 3,
        nome: "Teste".to_string(),
        email: "teste@email.com".to_string(),
    };


    db::executar_query(&session, &query_builder::insert(&cliente)).await;         // INSERT
    db::executar_query(&session, &query_builder::insert(&cliente_extra)).await;         // INSERT
    println!("LISTAGEM clientes inseridos");
    listar_entidades::<Cliente>(&session).await;
    println!("\n");

    db::executar_query(&session, &query_builder::update_by_id(&cliente_para_update, 1)).await; // UPDATE
    db::executar_query(&session, &query_builder::delete_by_id::<Cliente>(3)).await;     // DELETE
    println!("LISTAGEM de clientes atualizados e deletado");
    listar_entidades::<Cliente>(&session).await;
    println!("\n");


    // --- TESTES DE CRUD: PRODUTO ---
    let produto = Produto {
        id: 1,
        nome: "Notebook".to_string(),
        preco: 4999.90,
    };
    let produto_extra = Produto {
        id: 2,
        nome: "Monitor".to_string(),
        preco: 999.99,
    };
    let produto_atualizado = Produto {
        id: 2,
        nome: "Monitor Gamer".to_string(),
        preco: 1299.99,
    };

    db::executar_query(&session, &query_builder::insert(&produto_extra)).await;
    db::executar_query(&session, &query_builder::insert(&produto)).await;
    println!("LISTAGEM de produtos inseridos");
    listar_entidades::<Produto>(&session).await;
    println!("\n");


    db::executar_query(&session, &query_builder::delete_by_id::<Produto>(1)).await;
    db::executar_query(&session, &query_builder::update_by_id(&produto_atualizado, 2)).await;
    println!("LISTAGEM de produtos atualizados e deletado");
    listar_entidades::<Produto>(&session).await;
    println!("\n");

    
    // --- TESTES DE CRUD: EVENTO ---
    let evento = Evento {
        id: 1,
        titulo: "Aniversário".to_string(),
        data: "2025-05-10".to_string(),
    };
    let evento_extra = Evento {
        id: 2,
        titulo: "Reunião".to_string(),
        data: "2025-06-15".to_string(),
    };
    let evento_atualizado = Evento {
        id: 2,
        titulo: "Reunião Final".to_string(),
        data: "2025-06-30".to_string(),
    };
 

    db::executar_query(&session, &query_builder::insert(&evento_extra)).await;
    db::executar_query(&session, &query_builder::insert(&evento)).await;
    println!("LISTAGEM de eventos inseridos");
    listar_entidades::<Evento>(&session).await;
    println!("\n");

    db::executar_query(&session, &query_builder::delete_by_id::<Evento>(1)).await;
    db::executar_query(&session, &query_builder::update_by_id(&evento_atualizado, 2)).await;
    println!("LISTAGEM de eventos atualizado e deletado");
    listar_entidades::<Evento>(&session).await;
}

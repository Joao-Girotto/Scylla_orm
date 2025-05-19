# Scylla ORM
**scylla_orm** é um mini framework ORM (Object-Relational Mapper) desenvolvido em Rust para o banco de dados **ScyllaDB**, com suporte completo a operações CRUD. Foi criado para o trabalho prático da disciplina **Aspectos de Linguagens de Programação** do curso de Ciência da Computação da UTFPR — Campo Mourão.

---

##  Funcionalidades

- Mapeamento de structs Rust para tabelas do ScyllaDB
- Criação de tabelas (`CREATE TABLE`)
- Inserção de dados (`INSERT`)
- Leitura de registros (`SELECT *`)
- Atualização de dados (`UPDATE`)
- Remoção de dados (`DELETE`)
- Modularização com suporte a múltiplas entidades

---

## Estrutura do Projeto

```
scylla_orm/
├── src/
│ ├── main.rs # Exemplo de uso do framework
│ ├── lib.rs # Integração dos módulos
│ ├── orm/
│ │ ├── entity.rs # Trait Entity
│ │ ├── db.rs # Conexão com Scylla e execução de queries
│ │ ├── query_builder.rs # Geração de queries SQL/CQL
│ │ └── mod.rs
│ ├── models/
│ │ ├── cliente.rs # Exemplo de entidade Cliente
│ │ └── mod.rs
├── docker-compose.yml # Container ScyllaDB
├── Cargo.toml
```
---

## Como Executar

### 1. Suba o ScyllaDB com Docker

```bash
docker-compose up -d
```
#### 2. Compile e execute

```bash
cargo build
cargo run
```
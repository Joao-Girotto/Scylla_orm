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
---

## Comparações

### Diesel
O Diesel é um dos ORMs mais maduros do ecossistema Rust, focado em bancos de dados relacionais como PostgreSQL, MySQL e SQLite. Ele se destaca por sua forte verificação de tipos em tempo de compilação, onde até mesmo as queries SQL são validadas pelo compilador Rust via macros e código gerado automaticamente. No entanto, essa abordagem introduz uma curva de aprendizado mais alta e uma dependência significativa de macros (#[derive(Queryable], table!, etc.), o que o torna menos transparente para estudantes ou iniciantes. Em contraste, o scylla_orm adota uma abordagem leve e didática, dispensando macros e focando no uso explícito de traits e generics, o que favorece a compreensão do funcionamento interno do mapeamento objeto-banco. Além disso, o scylla_orm foi projetado especificamente para ScyllaDB, um banco NoSQL columnar que não é suportado pelo Diesel.

### SeaORM

O SeaORM é um ORM moderno e assíncrono para Rust que visa produtividade e facilidade de uso, com suporte a vários bancos relacionais. Ele adota uma abordagem parecida com ORMs de outras linguagens, gerando código com base no schema do banco e permitindo consultas encadeadas com APIs fluentes. Embora seja poderoso e mais flexível que o Diesel em alguns aspectos, o SeaORM ainda depende fortemente de código gerado automaticamente (via sea-orm-cli) e se destina a cenários mais completos de aplicações web ou microserviços. Por outro lado, o scylla_orm é propositalmente minimalista: ele oferece apenas o essencial para operações CRUD em ScyllaDB, focando em clareza, controle total do código e aprendizado dos fundamentos da linguagem Rust. Ao evitar geração automática e abstrações pesadas, ele permite ao desenvolvedor entender exatamente o que está sendo executado no banco.

### Scylla Orm
Ao contrário de frameworks como Diesel e SeaORM, que priorizam automação e abstração via macros, geração de código e ferramentas externas, o scylla_orm adota uma abordagem manual, explícita e leve, voltada à compreensão profunda dos mecanismos de mapeamento objeto-banco. Enquanto o Diesel valida queries SQL em tempo de compilação e o SeaORM oferece APIs fluentes com suporte a múltiplos bancos relacionais, ambos se baseiam em bancos SQL tradicionais e têm pouca ou nenhuma compatibilidade com bancos NoSQL columnar como o ScyllaDB. Já o scylla_orm foi projetado especificamente para funcionar com ScyllaDB, e usa traits e generics para definir comportamento comum entre entidades, sem esconder a lógica de persistência do desenvolvedor. Essa escolha torna o framework altamente educacional, além de manter o código enxuto, transparente e fácil de estender, mesmo que sacrifique parte da conveniência presente em ORMs mais robustos.

## Contributors

João Vitor Girotto  
Lucas dos Santos Vaz
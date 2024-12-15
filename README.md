# RustAPI




Estrutura de pasta
.
├── Cargo.toml                  # Configuração do projeto e dependências
├── src/
│   ├── main.rs                 # Ponto de entrada da aplicação
│   ├── routes/                 # Define as rotas da aplicação
│   │   ├── mod.rs              # Organiza as rotas
│   │   ├── health.rs           # Rota de health check
│   │   └── users.rs            # Rotas relacionadas a usuários
│   ├── services/               # Lógica de negócio
│   │   ├── mod.rs              # Organiza os serviços
│   │   └── user_service.rs     # Serviços relacionados a usuários
│   ├── infrastructure/         # Integrações externas e configurações de infraestrutura
│   │   ├── mod.rs              # Organiza a infraestrutura
│   │   ├── environment.rs      # Setup das variaveis ambientes
│   │   ├── logger.rs           # Setup do logger da aplicação
│   │   └── postgres.rs         # Configuração e operações do banco de dados PostgreSQL
│   ├── middlewares/            # Middlewares para manipulação de requisições/respostas
│   │   ├── mod.rs              # Organiza os middlewares
│   │   ├── logger.rs           # Middleware para log de requisições
│   │   ├── validation.rs       # Middleware para validar body da requisição
│   │   ├── permission.rs       # Middleware para log de permissão
|   |   └── authentication.rs   # Middleware para validar authentificação
│   └── errors/                 # Gerenciamento de erros personalizados
│       └── mod.rs              # Erros da aplicação
# RustAPI

Estrutura Proposta para a Aplicação

Handler (Controller):
Recebe as solicitações HTTP, valida os dados de entrada, e delega as operações para o serviço correspondente.

Service:
Contém a lógica de negócios. Este é o núcleo onde todas as regras do domínio são aplicadas.

Repository:
Realiza operações no banco de dados ou em qualquer outra camada de persistência.

Models:
Representam os dados do domínio, tanto as entradas (DTOs) quanto as saídas.


<!-- Estrutura de pasta
.
├── Cargo.toml                  # Configuração do projeto e dependências
├── src/
│   ├── main.rs                 # Ponto de entrada da aplicação
│   ├── docs                    # Pasta onde fica juntado as docs de todos os recursos
│   │   ├── mod.rs              
│   │   └── open_api.rs         # Rotas relacionadas a usuários
│   │── errors/                 # Gerenciamento de erros personalizados
│   │   └── mod.rs              # Erros da aplicação
│   ├── infrastructure/         # Integrações externas e configurações de infraestrutura
│   │   ├── grpc                # Organiza todos os serviços de GRPC
│   │   |   |── mod.rs
│   │   |   └── clients         # servidor GRPC clients ( todos files necessarios abaixo dessa pasta)
│   │   ├── mod.rs              # Organiza a infraestrutura
│   │   ├── environment.rs      # Setup das variaveis ambientes
│   │   ├── logger.rs           # Setup do logger da aplicação
│   │   └── postgres.rs         # Configuração e operações do banco de dados PostgreSQL
│   ├── resources               # Todas as 'rotas' aqui em baixo dessa pasta ( recursos da API )
│   │   ├── mod.rs              
│   │   ├── users               # Organiza o recurso de usuarios
│   │   │   ├── controllers.rs  # Controladores da rota de usuario
│   │   │   ├── docs.rs         # Documentação do OpenAPI para o recurso de usuario
│   │   │   ├── mod.rs          
│   │   │   ├── models.rs       # Data Models utilizados na APi, aqui deve conter a request e response (usando a macro validation da lib validator)
│   │   │   ├── repository.rs   # Consulta de Repositorios utiliando sql (QUERYS ESCRITAS NA MÃO) OU Consulta da infrastructure/grpc/users
│   │   │   ├── services.rs     # Regras de negocio que são chamadas pelo controlador da rota ( camada de negocio )
│   │   │   └── routes.rs       # Rotas dos usuarios são chamadas aqui.
│   │   └── users.rs            # Rotas relacionadas a usuários
│   ├── middlewares/            # Middlewares para manipulação de requisições/respostas
│   │   ├── mod.rs              
│   │   ├── permission.rs       # Middleware para log de permissão ( no caso da hedro o ruskit faz isso )
|   |   └── authentication.rs   # Middleware para validar authentificação ( no caso da hedro o ruskit faz isso) -->


Estrutura de pasta
.
├── Cargo.toml                  # Configuração do projeto e dependências
├── src/
│   ├── main.rs                 # Ponto de entrada da aplicação
│   ├── docs                    # Pasta onde fica juntado as docs de todos os recursos
│   │   ├── mod.rs              
│   │   └── open_api.rs         # Rotas relacionadas a usuários
│   │── errors/                 # Gerenciamento de erros personalizados
│   │   └── mod.rs              # Erros da aplicação
│   ├── infrastructure/         # Integrações externas e configurações de infraestrutura
│   │   ├── grpc                # Organiza todos os serviços de GRPC
│   │   |   |── mod.rs
│   │   |   └── clients         # servidor GRPC clients ( todos files necessarios abaixo dessa pasta)
│   │   ├── mod.rs              # Organiza a infraestrutura
│   │   ├── environment.rs      # Setup das variaveis ambientes
│   │   ├── logger.rs           # Setup do logger da aplicação
│   │   └── postgres.rs         # Configuração e operações do banco de dados PostgreSQL
│   ├── resources               # Todas as 'rotas' aqui em baixo dessa pasta ( recursos da API )
│   │   ├── mod.rs              
│   │   ├── users               # Organiza o recurso de usuarios
│   │   │   ├── controllers.rs  # Controladores da rota de usuario
│   │   │   ├── docs.rs         # Documentação do OpenAPI para o recurso de usuario
│   │   │   ├── mod.rs          
│   │   │   ├── models.rs       # Data Models utilizados na APi, aqui deve conter a request e response (usando a macro validation da lib validator)
│   │   │   ├── repository.rs   # Consulta de Repositorios utiliando sql (QUERYS ESCRITAS NA MÃO) OU Consulta da infrastructure/grpc/users
│   │   │   ├── services.rs     # Regras de negocio que são chamadas pelo controlador da rota ( camada de negocio )
│   │   │   └── routes.rs       # Rotas dos usuarios são chamadas aqui.
│   │   └── users.rs            # Rotas relacionadas a usuários
│   ├── middlewares/            # Middlewares para manipulação de requisições/respostas
│   │   ├── mod.rs              
│   │   ├── permission.rs       # Middleware para log de permissão ( no caso da hedro o ruskit faz isso )
|   |   └── authentication.rs   # Middleware para validar authentificação ( no caso da hedro o ruskit faz isso)
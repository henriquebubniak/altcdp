# Site alternativo para o clube de programação

## Prerequisitos

1. Docker
2. Rust

## Como executar

1. Execute o comando em docker_command.txt para criar o container com o banco de dados.
2. Conecte-se ao banco de dados como preferir, e execute os scripts create.sql e populate.sql para criar as tabelas e popular o banco de dados.
3. Inicie o servidor com o comando "cargo run"
4. Acesse o site em localhost:8081

## To do

- Permitir usuários editares suas oficinas
- Permitir criação de calendário de oficinas
- Adicionar busca à pagina de oficinas
- Melhorar o layout dos forms
- Melhorar segurança das sessões
- Melhorar estrutura do código
- Permitir usuários se voluntariarem
- Permitir administrador editar oficinas

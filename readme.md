# Fuzzing Smart Contracts

Uma ferramenta simples para realizar fuzzing em smart contracts na rede Ethereum, escrita em Rust. Esta ferramenta gera inputs aleatórios para testar a robustez de funções de smart contracts.

## Pré-requisitos

Antes de executar o projeto, você precisará ter:

- [Rust](https://www.rust-lang.org/tools/install) instalado na sua máquina.
- Uma conta no [Infura](https://infura.io/) ou outro provedor de nó Ethereum para acessar a rede.

## Instalação

Clone este repositório e navegue até o diretório do projeto:

```bash
git clone https://github.com/seuusuario/fuzzing_smart_contract.git
cd fuzzing_smart_contract
```

Em seguida, instale as dependências:

```bash
cargo build
```

## Uso

Para usar a ferramenta, você precisará editar o arquivo `src/main.rs` e substituir `YOUR_INFURA_PROJECT_ID`, `0xYourSmartContractAddress` e `yourFunctionSignature` pelos valores apropriados.

Depois de fazer as alterações necessárias, você pode executar o projeto com:

```bash
cargo run
```

A ferramenta irá gerar inputs aleatórios e tentar invocar a função especificada no smart contract. Você verá a saída no terminal, mostrando se a transação foi bem-sucedida ou se falhou.

## Contribuindo

Se você deseja contribuir para este projeto, sinta-se à vontade para abrir issues ou pull requests. Estamos sempre abertos a melhorias e sugestões!

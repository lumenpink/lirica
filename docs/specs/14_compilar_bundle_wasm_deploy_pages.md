# Especificação Técnica: Compilação de Bundle WASM e Deploy no Pages

- **Ticket**: #14
- **Escopo**: Adição do passo de compilação WebAssembly via `wasm-pack` nas esteiras de CI e garantia de inclusão dos artefatos `site/pkg` no deploy do Cloudflare Pages.

## 1. Contexto e Motivação
O laboratório métrico Lírica opera exclusivamente client-side via WebAssembly e consome o motor `joaju-lirica`. O frontend estático em `site/` depende diretamente de `site/pkg/lirica.js` e `site/pkg/lirica_bg.wasm`.

Como a pasta `site/pkg/` é gerada durante o build e contém um `.gitignore` automático emitido pelo `wasm-pack`, os artefatos compilados não são commitados no repositório. Nas esteiras de CI (`.forgejo/workflows/ci.yaml` e `.github/workflows/ci.yaml`), o passo de compilação do bundle WASM estava ausente, fazendo com que o deploy via `npx wrangler pages deploy site` publicasse a aplicação sem a pasta `pkg/`. Isso gerava falha crítica na inicialização do JavaScript, impedindo o carregamento da poesia padrão e bloqueando todas as ações na interface.

## 2. Requisitos Técnicos
1. **Esteira de CI Forgejo (`.forgejo/workflows/ci.yaml`)**:
   - Incluir a compilação do pacote WebAssembly utilizando `wasm-pack build --target web --out-dir site/pkg --release`.
   - Executar a remoção preventiva de `site/pkg/.gitignore` para garantir que o Wrangler não ignore os binários compilados durante a publicação do diretório `site`.
2. **Esteira de CI GitHub (`.github/workflows/ci.yaml`)**:
   - Sincronizar o mesmo comportamento e ordem de passos para assegurar paridade entre as plataformas.
   - Instalar `wasm-pack` ou certificar presença no ambiente.
3. **Validação e Testes (`tests/test_ci_wasm_packaging.sh`)**:
   - Teste de integração automatizado verificando a conformidade dos workflows com a invocação do `wasm-pack` e o tratamento de `site/pkg/.gitignore`.
   - Verificação da integridade da compilação de `site/pkg/lirica.js` e `site/pkg/lirica_bg.wasm`.

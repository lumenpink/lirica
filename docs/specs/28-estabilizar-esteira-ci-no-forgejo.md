# Especificação: Estabilizar Esteira de CI no Forgejo (Ticket #28)

## 1. Diagnóstico do Defeito no Pipeline
No run 2051 da esteira de CI do Forgejo Actions, a execução falhou na fase `Run Tests` com a seguinte mensagem de erro:
```text
sccache: encountered fatal error
sccache: error: No such file or directory (os error 2)
sccache: caused by: No such file or directory (os error 2)
error: could not compile `unicode-ident` (lib)
```
Adicionalmente, a etapa anterior `Cache Cargo Dependencies` gerou um atraso de quase 1 minuto e falhou com:
```text
::warning::Failed to restore: getCacheEntry failed: tunneling socket could not be established, statusCode=502
```

## 2. Decisão Arquitetural (Árvore de Decisões para a Natasha)
- **Causa-Raiz:**
  1. A imagem base `gost-ci-wasm:latest` injeta por padrão `ENV RUSTC_WRAPPER=/usr/local/bin/sccache` e `ENV SCCACHE_DIR=/root/.cache/sccache`.
  2. Em containers efêmeros do Forgejo runner sem volume persistente montado em `/root/.cache/sccache`, o wrapper falha com `os error 2`.
  3. O serviço de cache de actions (`actions/cache@v4`) não está habilitado no servidor local do Forgejo, retornando 502 Bad Gateway.
- **Alternativa A:** Tentar criar pastas e subir o daemon `sccache` manualmente no workflow.
  *Descarte:* Introduz acoplamento desnecessário e pontos únicos de falha. `lirica` é uma biblioteca enxuta que compila em ~3s do zero.
- **Solução Adotada (Alinhamento Canônico com `navalha` e `kokorosim`):**
  1. Remover o step obsoleto `actions/cache@v4` de `.forgejo/workflows/ci.yaml`.
  2. Definir explicitamente `RUSTC_WRAPPER: ""` no ambiente do job, garantindo compilação nativa e direta com `rustc` sem depender de daemon de cache local.

## 3. Critérios de Aceitação
- `.forgejo/workflows/ci.yaml` não deve invocar `actions/cache@v4`.
- O job `test-and-deploy` deve declarar `RUSTC_WRAPPER: ""`.
- Teste de contrato em `tests/wasm_api.rs` verifica a conformidade da especificação de CI.

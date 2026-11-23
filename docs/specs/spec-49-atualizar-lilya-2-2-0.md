# Spec #49: Atualizar Perfil Lilya para Versão 2.2.0

## 1. Contexto e Motivação
Com o lançamento do GOST Lilya 2.2.0 e Kollontai 2.2.0, o compilador Rust de referência foi atualizado para rustc >= 1.100.0 e as rotas canônicas de regras foram unificadas. O repositório lirica precisa atualizar suas definições de governança (.gost/GOST.md e .gost/rules.toml) para herdar as regras e padrões da versão 2.2.0.

## 2. Escopo
- Atualizar referências de `2.1.0` para `2.2.0` em `.gost/GOST.md` e `.gost/rules.toml`.
- Atualizar requisito de compilador Rust para `rustc >= 1.100.0` em `.gost/GOST.md`.
- Validar conformidade estrita com `cccp check --strict`.
- Validar suíte de testes do repositório (`cargo test`).

## 3. Plano de Implementação (GOST DD-TDD)
1. Fase 3: Especificação técnica (este documento).
2. Fase 4: Teste Red (N/A - atualização de governança).
3. Fase 5: Atualização de `.gost/GOST.md` e `.gost/rules.toml`.
4. Fase 6-7: Refatoração e Mutação (N/A).
5. Fase 8: Quality Gate (`cargo test` e `cccp check --strict`).
6. Fase 9: Fechamento do ticket e integração.

## 4. Árvore de Decisão e Guardiã da Natasha
- **Por que atualizar para Lilya 2.2.0?** O repositório lirica deve acompanhar a versão canônica do compilador e a taxonomia atualizada de regras, prevenindo defasagem na toolchain e no CI.
- **Alternativa descartada:** Manter 2.1.0 geraria alertas em pipelines automatizadas e descontinuidade com os artefatos gerados pelo ecossistema.

## 5. Resultados e Validação
- cccp check: 24/24 PASS (100% de conformidade com Lilya 2.2.0).
- cargo test: 42 testes (unittests, wasm_api e testes de sincronização de gabarito) passaram com 100% de sucesso.

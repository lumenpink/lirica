# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Substituição de campos técnicos e jargões no box de rimas das estrofes por confronto morfológico em português e explicação didática em prosa acessível (closes #30).

### Fixed
- Correção de ancoragem de rolagem (.export-preview-wrapper com align-items: flex-start), viabilizando navegação vertical completa no preview do modal de exportação (closes #39).
- Correção de callback do modal de legenda em site/app.js e modularização dos ouvintes de eventos, eliminando ReferenceError e restaurando o modo texto livre e o workspace alinhado (closes #29).
- Estabilização da esteira de CI no Forgejo (.forgejo/workflows/ci.yaml), desativando actions/cache inexistente e desabilitando RUSTC_WRAPPER para compilar diretamente sem falhas de sccache em container efêmero (closes #28).
- Isolamento dos versos poéticos na escansão para descartar cabeçalhos Markdown (# Título e ## Autor), eliminando estrofe espúria e restaurando a avaliação de rimas no topo das estrofes (closes #25).
- Condicionamento de persistência no localStorage a edições reais via dirty state, evitando bloqueio do sorteio aleatório ao inspecionar exemplos (closes #23).
- Correção de precedência de rascunho recuperado no bootstrap para evitar sobrescrita pela biblioteca de exemplos e sorteio aleatório de poema inicial na ausência de rascunho (closes #21).
- Configuração de wasm-opt = false para compatibilidade com bulk memory em builds WebAssembly (closes #15).
- Compilação WebAssembly via wasm-pack no CI e empacotamento completo de site/pkg para deploy no Cloudflare Pages (closes #14).
- Configuração de `[profile.release]` com LTO fat e strip, e linking estático `+crt-static` para musl libc (closes #10).

### Added
- Enriquecimento do card poético com transparência metodológica (critérios métricos aplicados), mini-legenda contextual dos ícones métricos utilizados e incorporação do laudo e análise de rimas (closes #39).
- Gerador de card poético vetorial em SVG nativo e exportação em PNG de alta resolução e documento Typst editorial (.typ), com 3 temas visuais (Manuscrito, Noturno e Minimalista) e modal de compartilhamento (closes #35).
- Incorporação de formas populares brasileiras (Pavão Misterioso em sextilhas de Redondilha Maior, Pajada Gaúcha em décimas espinelianas, Galope à Beira-Mar com refrão canônico e reconhecimento de folhetos de cordel com marco de 32 estrofes in-octavo) (closes #36).
- Identificação determinística de cesuras clássicas em versos longos (decassílabos e alexandrinos) com proteção contra corte intraverbal e destaque com glifo canônico ‖ (closes #37).
- Resolução conservadora de lookback sintático ampliado (janela de 3 tokens) e convergência nominal para zonas cinzentas de substantivos e adjetivos sem falsos positivos (closes #33).
- Guia canônico de avaliação de rimas (`site/docs/rimas.html`) com fundamentação na tradição lusófona (Castilho, Said Ali, Bilac, Spina), árvore de decisão didática, graus epistêmicos e detector de clichês (closes #32).
- Modal contextual dedicado para metro e ritmo ao clicar no badge do verso (sem duplicar escansão) e unificação do badge de rima com o diagnóstico da estrofe (closes #31).
- Legenda estática do rodapé (site/index.html) expandida com todos os 12 ícones de ritmos e metros clássicos e os 5 ícones de auditoria de rimas, com botão de atalho para o guia interativo completo (closes #27).
- Modal interativo de legenda métrica e rímica (#modal-legenda), permitindo consulta acessível por clique/toque em badges (.meter-badge e .rhyme-badge) com suporte a teclado Escape (closes #26).
- Campos de título e autor no topo do laboratório sincronizados com cabeçalhos Markdown (# e ##) no modo livre e persistência unificada (closes #24).
- Corpus completo dos poemas de exemplo em WebAssembly (Gonçalves Dias, Cruz e Sousa, Fernando Pessoa e Olavo Bilac) (closes #22).
- Persistência automática do poema em edição no localStorage com debounce de 400ms, restauração transparente no bootstrap e limpeza no reset (closes #20).
- Laboratório interativo de escansão poética e análise métrica em WebAssembly.
- Interface visual dividida (split-screen) com atualização em tempo real.
- Suporte a múltiplos modos de saída: Painel visual, Typst, JSON estruturado e Texto puro.
- Painel de controle métrico: contagem de Castilho, cesura na pontuação e diérese crescente.
- Catálogo de sonetos e estrofes clássicas da língua portuguesa pré-carregados.

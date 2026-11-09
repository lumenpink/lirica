# Especificação Técnica: Integração de Graus Epistêmicos de Rima e Detector de Clichês

**Ticket:** #19  
**Branch:** `feat/19-integracao-de-graus-epistemicos-de-rima-e-detector-de-cliches`  
**Escopo:** `src/lib.rs` (camada WASM), `site/app.js`, `site/style.css`, `site/index.html` e testes de serialização

---

## 1. Visão Geral e Princípios Touch-First

Este ciclo integra o novo modelo de classificação morfológica e epistêmica de rimas do `joaju-lirica` (Ticket #20 upstream) ao laboratório web `lirica`:

1. **Graus Epistêmicos de Certeza:**
   - Exibição de `Rica`, `Pobre`, `Preciosa`, `Rara`, `ProvavelPobre`, `ProvavelRica` e `Indeterminada`.
2. **Detector de Rimas Clichê:**
   - Identificação imediata e pedagógica de pares hiper-saturados (`eh_cliche: bool`, `motivo_cliche: Option<String>`).
3. **Ergonomia Tátil de Primeira Classe (Touch-First):**
   - Eliminação de qualquer dependência de *hover* ou cursores de mouse.
   - **Badges autoexplicativos** estampados diretamente no cabeçalho de cada estrofe (`✨ Rica • arde × tarde`, `🔥 Clichê • amor × dor`).
   - **Interação por Toque Único (Tap-to-Pin):** tocar em um par seleciona e ilumina os dois versos e palavras correspondentes no editor.
   - **Micro-Cartão Expansível (Inline Drawer):** tocar no badge abre um cartão pedagógico com as classes gramaticais, regras de retroanálise e contexto filológico.
   - **Etiqueta Tátil no Verso:** indicador discreto `[ A • 🔥 ]` ao lado do esquema rimante do verso com acionamento ao toque.

---

## 2. Tipagem e Payload WebAssembly (`src/lib.rs`)

A estrutura `AnaliseWeb` retornada pela função WASM `escandir()` é enriquecida com o relatório morfológico estrófico:

```rust
use joaju_lirica::morphology::{ParRimaAvaliado, QualidadeRima, RelatorioRimasPoema};

#[derive(Serialize, Deserialize)]
pub struct ParRimaWeb {
    pub versos: (usize, usize),
    pub tokens: (String, String),
    pub classificacao: String,
    pub detalhe_classificacao: Option<String>,
    pub justificativa: String,
    pub eh_cliche: bool,
    pub motivo_cliche: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EstrofeRimasWeb {
    pub estrofe_indice: usize,
    pub pares: Vec<ParRimaWeb>,
}

#[derive(Serialize, Deserialize)]
pub struct AnaliseWeb {
    pub versos: Vec<VersoFormatado>,
    pub estrofes: Vec<EstrofeWeb>,
    pub esquema_rima: Option<String>,
    pub forma_poetica: Option<DiagnosticoForma>,
    pub avaliacao_rima: Option<InfoAvaliacaoRima>,
    pub rimas_morfologicas: Option<Vec<EstrofeRimasWeb>>,
}
```

---

## 3. Componentes de Interface (`site/`)

1. **Faixa de Pares por Estrofe (`.stanza-rhyme-strip`):**
   - Alvos de toque acessíveis ($\ge 44\text{px}$ de altura interativa).
   - Cores e semântica:
     - `Rica`: violeta elegante com ícone `✨`.
     - `Pobre`: grafite neutro.
     - `Preciosa`: turquesa lapidado com ícone `💎`.
     - `Rara`: esmeralda com ícone `✦`.
     - `Provável`: borda tracejada fina e ícone `⚡`.
     - `Indeterminada`: borda âmbar e ícone `?`.
     - `Clichê`: tag complementar de fogo `🔥`.
2. **Micro-Cartão de Auditoria (`.rhyme-audit-card`):**
   - Renderizado dinamicamente abaixo da faixa da estrofe quando um par é tocado.
   - Botão de fechar `✕` e fechamento ao retocar o badge.
3. **Destaque Visual Conectado:**
   - Classe CSS `.verse-highlight-rhyme` aplicada aos versos do par ativo.

---

## 4. Critérios de Aceite

1. Todos os pares de rima de estrofes canônicas (ABBA, ABAB, AABB, etc.) recebem classificação morfológica em `AnaliseWeb`.
2. Clichês canônicos (*amor* + *dor*, *mar* + *amar*) geram flag `eh_cliche: true` e motivo correspondente.
3. Testes unitários em `tests/` validam 100% da serialização e enriquecimento do payload.
4. O pacote WebAssembly compila com sucesso (`wasm-pack build --target web`).
5. A interface funciona com 100% de paridade por toque (sem hover).
6. Linters `cargo fmt` e `cargo clippy --all-targets -- -D warnings` sem qualquer aviso.

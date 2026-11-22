# Lírica

> Laboratório métrico e escansão poética determinística em WebAssembly para versos em língua portuguesa.

[![GOST: v3.0.0](https://img.shields.io/badge/GOST-v3.0.0-blue)](.gost/GOST.md)
[![Profile: Lilya 2.1](https://img.shields.io/badge/Profile-Lilya--2.1-purple.svg)](.gost/GOST.md)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

## Sobre

O **Lírica** é um playground interativo client-side para análise rítmica, divisão silábica poética e classificação métrica de versos em português. Ele consome o motor [joaju-lirica](https://gambi.2lp.in/lumen/joaju-lirica) através do registro autônomo `lumen` e compila para WebAssembly de alto desempenho. Espelhos públicos sincronizados no [Codeberg](https://codeberg.org/lumenpink/lirica) e no [GitHub](https://github.com/lumenpink/lirica).

## Funcionalidades

- **Alinhamento Verso a Verso**: Cada verso editável fica exatamente alinhado ao lado da sua análise métrica e fonotática irmã.
- **Análise Automática de Rimas**: Identificação instantânea do esquema rímico (ABBA, ABCB, AABB) e das terminações fonéticas.
- **Métrica Determinística (WASM)**: Execução client-side via WebAssembly com latência nula.
- **Identificação de Tônicas e Sinalefas**: Destaque visual das sílabas tônicas, elisões poéticas (`‿`) e sílabas extramétricas (`·`).
- **Classificação de Ritmo**: Reconhecimento automático de decassílabos (heroicos, sáficos), redondilhas (maior, menor), alexandrinos e formas livres.
- **Notação Poética e Sinais**: Suporte a `/` (forçar hiato), `_` (forçar sinalefa), cesura de pontuação e regra de Castilho.
- **Formatos Múltiplos**: Visão alinhada, painel estruturado, Typst markup, JSON (AST) e texto puro.

## Compilação Local

```bash
# Compilar bundle WebAssembly otimizado
wasm-pack build --target web --out-dir site/pkg --release

# Compilação estática universal (static-pie) contra musl libc
cargo build --release --target x86_64-unknown-linux-musl

# Servir localmente
cd site && python3 -m http.server 8080
```

---

Feito com amor travesti por [Lumen Pink](https://lumen.pink).

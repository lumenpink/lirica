# Lírica

> Laboratório métrico e escansão poética determinística em WebAssembly para versos em língua portuguesa.

[![GOST Compliance](https://img.shields.io/badge/GOST-1.5.0-blue)](https://gostlumen.2lp.in/profile.md)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

## Sobre

O **Lírica** é um playground interativo client-side para análise rítmica, divisão silábica poética e classificação métrica de versos em português. Ele consome o motor [joaju-lirica](https://codeberg.org/lumenpink/joaju-lirica) através do registro soberano `lumen` e compila para WebAssembly de alto desempenho.

## Funcionalidades

- **Métrica Instantânea (WASM)**: Execução local, determinística e com latência nula.
- **Divisão em Painéis**: Entrada editável à esquerda e saída poética detalhada à direita.
- **Identificação de Tônicas e Sinalefas**: Marcação visual exata das sílabas tônicas e elisões poéticas.
- **Classificação de Ritmo**: Reconhecimento automático de decassílabos (heroicos, sáficos), redondilhas (maior, menor), alexandrinos e formas livres.
- **Formatos Múltiplos**: Visualização em painel, código Typst pronto para diagramação acadêmica, JSON e texto puro.
- **Controles Poéticos**: Ajustes finos de cesura na pontuação, regra de Castilho e diérese crescente.

## Compilação Local

```bash
# Compilar bundle WebAssembly
wasm-pack build --target web --out-dir site/pkg --release

# Servir localmente
cd site && python3 -m http.server 8080
```

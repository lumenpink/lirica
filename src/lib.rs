//! # lirica
//!
//! WebAssembly interface for Portuguese poetic scansion, rhythm analysis, and rhyme schemes.

use joaju_lirica::{
    descobrir_tonica, escandir_verso_com_opcoes, parse_lirica_block, render_painel, render_typst,
    separar_silabas_com_spans, Poema, PoemaConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Pre-loaded classical poem example.
#[derive(Serialize, Deserialize)]
pub struct ExemploPoema {
    /// Identifier slug.
    pub id: String,
    /// Human readable title.
    pub titulo: String,
    /// Poet / Author.
    pub autor: String,
    /// Poetic form / expected meter description.
    pub descricao: String,
    /// Raw text.
    pub texto: String,
}

/// Structured verse scansion output for rich web rendering.
#[derive(Serialize, Deserialize)]
pub struct VersoFormatado {
    /// Original raw line.
    pub original: String,
    /// Scansion textual notation (e.g. "1:A - 2:mor - 3:é...").
    pub escansao: String,
    /// Rhythm classification (e.g. "Decassílabo Heroico").
    pub classificacao: String,
    /// Metric syllable count.
    pub total_metricas: usize,
    /// Phonetic syllable count.
    pub total_foneticas: usize,
    /// Tonic stress positions (1-indexed).
    pub tonicas: Vec<usize>,
    /// Syllable details.
    pub silabas: Vec<SilabaDetalhe>,
    /// Rhyme scheme identifier (e.g. "A", "B", "C").
    pub rima: Option<String>,
    /// Rhyme termination phonemes (e.g. "-er", "-ente").
    pub rima_terminacao: Option<String>,
}

/// Syllable details for visual highlighting.
#[derive(Serialize, Deserialize)]
pub struct SilabaDetalhe {
    /// Syllable text.
    pub texto: String,
    /// Whether this syllable carries tonic stress.
    pub tonica: bool,
    /// Metric index if counted (ends at final tonic under Castilho rule).
    pub numero_metrico: Option<usize>,
    /// Whether this syllable participates in an elision/sinalefa.
    pub elidida: bool,
}

/// Scans a text block of verses and returns the rendered result according to options.
#[wasm_bindgen]
pub fn escandir(
    texto: &str,
    modo_saida: &str,
    elidir_pontuacao: bool,
    dierese_crescente: bool,
    contagem_castilho: bool,
    analisar_rima: bool,
) -> String {
    if texto.trim().is_empty() {
        return String::new();
    }

    // Try parsing directives first, or create custom config
    let (mut config, poema) = match parse_lirica_block(texto) {
        Ok((cfg, p)) => (cfg, p),
        Err(_) => {
            let cfg = PoemaConfig {
                elidir_pontuacao,
                dierese_crescente,
                contagem_castilho,
                analisar_rima,
                ..Default::default()
            };

            let mut versos = Vec::new();
            let mut linhas = Vec::new();
            for linha in texto.lines() {
                let trimmed = linha.trim();
                if !trimmed.is_empty() && !trimmed.starts_with("%%") {
                    let esc = escandir_verso_com_opcoes(
                        linha,
                        elidir_pontuacao,
                        dierese_crescente,
                        contagem_castilho,
                    );
                    versos.push(esc);
                    linhas.push(linha.to_string());
                }
            }
            (
                cfg,
                Poema {
                    versos,
                    linhas_originais: linhas,
                },
            )
        }
    };

    // Apply explicit argument overrides if specified
    config.elidir_pontuacao = elidir_pontuacao;
    config.dierese_crescente = dierese_crescente;
    config.contagem_castilho = contagem_castilho;
    config.analisar_rima = analisar_rima;

    match modo_saida.to_lowercase().as_str() {
        "typst" => render_typst(&config, &poema),
        "json" => match serde_json::to_string_pretty(&poema) {
            Ok(j) => j,
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        },
        "texto" => poema.linhas_originais.join("\n"),
        "visual" | "html" => render_visual_json(&poema, analisar_rima),
        _ => render_painel(&poema),
    }
}

/// Extracts the phonetic rhyme suffix (from the final tonic vowel onward) for a verse.
fn extrair_rima_verso(linha: &str) -> Option<String> {
    let clean = linha
        .trim()
        .trim_end_matches(|c: char| !c.is_alphabetic() && c != '/');
    let last_word = clean.split_whitespace().last()?;
    let palavra_limpa: String = last_word.chars().filter(|&c| c.is_alphabetic()).collect();
    if palavra_limpa.is_empty() {
        return None;
    }

    let sils = separar_silabas_com_spans(&palavra_limpa, 0, false);
    if sils.is_empty() {
        return None;
    }

    let idx_tonica = descobrir_tonica(&sils, &palavra_limpa).unwrap_or(sils.len() - 1);

    let mut term = String::new();
    for (i, (s, _)) in sils.iter().enumerate().skip(idx_tonica) {
        if i == idx_tonica {
            let chars: Vec<char> = s.chars().collect();
            let mut v_idx = 0;
            for (ci, &c) in chars.iter().enumerate() {
                if "aeiouáéíóúâêôãõàüAEIOUÁÉÍÓÚÂÊÔÃÕÀÜ".contains(c) {
                    v_idx = ci;
                    break;
                }
            }
            term.push_str(&chars[v_idx..].iter().collect::<String>());
        } else {
            term.push_str(s);
        }
    }

    let normalizada = term
        .to_lowercase()
        .replace(['á', 'â', 'à', 'ã'], "a")
        .replace(['é', 'ê'], "e")
        .replace('í', "i")
        .replace(['ó', 'ô', 'õ'], "o")
        .replace(['ú', 'ü'], "u");

    Some(normalizada)
}

fn render_visual_json(poema: &Poema, analisar_rima: bool) -> String {
    let mut resultado = Vec::new();

    // Map rhymes across lines if requested
    let mut mapa_rimas: HashMap<String, char> = HashMap::new();
    let mut letra_atual = 'A';
    let mut rimas_info = Vec::new();

    for orig in &poema.linhas_originais {
        if analisar_rima {
            if let Some(term) = extrair_rima_verso(orig) {
                let letra = *mapa_rimas.entry(term.clone()).or_insert_with(|| {
                    let l = letra_atual;
                    if letra_atual < 'Z' {
                        letra_atual = (letra_atual as u8 + 1) as char;
                    }
                    l
                });
                rimas_info.push(Some((letra.to_string(), format!("-{}", term))));
            } else {
                rimas_info.push(None);
            }
        } else {
            rimas_info.push(None);
        }
    }

    for (i, (orig, esc)) in poema
        .linhas_originais
        .iter()
        .zip(poema.versos.iter())
        .enumerate()
    {
        let partes: Vec<String> = esc
            .silabas
            .iter()
            .map(|s| {
                let txt = if s.tonica {
                    format!("[{}]", s.texto)
                } else {
                    s.texto.clone()
                };
                if let Some(num) = s.numero_metrico {
                    format!("{}:{}", num, txt)
                } else {
                    format!("·{}", txt)
                }
            })
            .collect();

        let silabas: Vec<SilabaDetalhe> = esc
            .silabas
            .iter()
            .map(|s| SilabaDetalhe {
                texto: s.texto.clone(),
                tonica: s.tonica,
                numero_metrico: s.numero_metrico,
                elidida: s.fusao,
            })
            .collect();

        let (rima, rima_terminacao) = match rimas_info.get(i).and_then(|r| r.as_ref()) {
            Some((l, t)) => (Some(l.clone()), Some(t.clone())),
            None => (None, None),
        };

        resultado.push(VersoFormatado {
            original: orig.clone(),
            escansao: partes.join(" - "),
            classificacao: esc
                .classificacao
                .clone()
                .unwrap_or_else(|| "Forma Livre".to_string()),
            total_metricas: esc.total_metricas,
            total_foneticas: esc.silabas.len(),
            tonicas: esc.esquema_tonico.clone(),
            silabas,
            rima,
            rima_terminacao,
        });
    }

    serde_json::to_string(&resultado).unwrap_or_else(|_| "[]".to_string())
}

/// Returns a JSON list of pre-configured classical poetic examples without `%%` directives.
#[wasm_bindgen]
pub fn carregar_exemplos() -> String {
    let exemplos = vec![
        ExemploPoema {
            id: "camoes".to_string(),
            titulo: "Amor é fogo que arde sem se ver".to_string(),
            autor: "Luís de Camões".to_string(),
            descricao: "Decassílabo Heroico (10 sílabas métricas, rimas ABBA)".to_string(),
            texto: "Amor é fogo que arde sem se ver,\nÉ ferida que dói e não se sente;\nÉ um contentamento descontente,\nÉ dor que desatina sem doer.".to_string(),
        },
        ExemploPoema {
            id: "goncalves_dias".to_string(),
            titulo: "Canção do Exílio".to_string(),
            autor: "Gonçalves Dias".to_string(),
            descricao: "Redondilha Maior / Heptassílabo (7 sílabas métricas, rimas ABCB)".to_string(),
            texto: "Minha terra tem palmeiras,\nOnde canta o Sabiá;\nAs aves que aqui gorjeiam,\nNão gorjeiam como lá.".to_string(),
        },
        ExemploPoema {
            id: "cruz_e_sousa".to_string(),
            titulo: "Sinfonias do Ocaso".to_string(),
            autor: "Cruz e Sousa".to_string(),
            descricao: "Decassílabo Simbolista com alternância e aliterações (ABBA)".to_string(),
            texto: "Mórbidos sons de flautas e de violas\nVagam nas brumas lívidas da tarde,\nEntre os lamentos vagos das corolas\nOnde a chama da vida ainda arde.".to_string(),
        },
        ExemploPoema {
            id: "pessoa".to_string(),
            titulo: "Autopsicografia".to_string(),
            autor: "Fernando Pessoa".to_string(),
            descricao: "Redondilha Maior com alternância rítmica lírica (ABAB)".to_string(),
            texto: "O poeta é um fingidor.\nFinge tão completamente\nQue chega a fingir que é dor\nA dor que deveras sente.".to_string(),
        },
        ExemploPoema {
            id: "bilac".to_string(),
            titulo: "Ouvir Estrelas".to_string(),
            autor: "Olavo Bilac".to_string(),
            descricao: "Decassílabos Parnasianos perfeitos (ABAB)".to_string(),
            texto: "\"Ora (direis) ouvir estrelas! Certo\nPerdeste o senso!\" E eu vos direi, no entanto,\nQue, para ouvi-las, muita vez desperto\nE abro as janelas, pálido de espanto...".to_string(),
        },
    ];

    serde_json::to_string(&exemplos).unwrap_or_else(|_| "[]".to_string())
}

/// Returns the engine version.
#[wasm_bindgen]
pub fn versao() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

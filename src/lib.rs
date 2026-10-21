//! # lirica
//!
//! WebAssembly presentation layer for the `joaju-lirica` poetic engine.

use joaju_lirica::{
    escandir_poema, icone_metro, parse_lirica_block, render_painel, render_typst, Poema,
    PoemaConfig,
};
use serde::{Deserialize, Serialize};
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
    /// Distinctive Unicode icon representing the metric classification.
    pub icone: String,
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

/// Complete poem analysis output for the web interface.
#[derive(Serialize, Deserialize)]
pub struct AnaliseWeb {
    /// Verses formatted with metric and rhyme data.
    pub versos: Vec<VersoFormatado>,
    /// Overall poem rhyme scheme (e.g. "ABBA", "ABCB").
    pub esquema_rima: Option<String>,
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

    // Try parsing directives first, or use pure poem scanner from joaju-lirica
    let (config, poema) = match parse_lirica_block(texto) {
        Ok((mut cfg, p)) => {
            cfg.elidir_pontuacao = elidir_pontuacao;
            cfg.dierese_crescente = dierese_crescente;
            cfg.contagem_castilho = contagem_castilho;
            cfg.analisar_rima = analisar_rima;
            (cfg, p)
        }
        Err(_) => {
            let p = escandir_poema(
                texto,
                elidir_pontuacao,
                dierese_crescente,
                contagem_castilho,
                analisar_rima,
            );
            (
                PoemaConfig {
                    elidir_pontuacao,
                    dierese_crescente,
                    contagem_castilho,
                    analisar_rima,
                    ..Default::default()
                },
                p,
            )
        }
    };

    match modo_saida.to_lowercase().as_str() {
        "typst" => render_typst(&config, &poema),
        "json" => match serde_json::to_string_pretty(&poema) {
            Ok(j) => j,
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        },
        "texto" => poema.linhas_originais.join("\n"),
        "visual" | "html" => render_visual_json(&poema),
        _ => render_painel(&poema),
    }
}

fn render_visual_json(poema: &Poema) -> String {
    let mut resultado = Vec::new();

    for (orig, esc) in poema.linhas_originais.iter().zip(poema.versos.iter()) {
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

        let class_name = esc
            .classificacao
            .clone()
            .unwrap_or_else(|| "Forma Livre".to_string());
        let icone = icone_metro(&class_name).to_string();

        resultado.push(VersoFormatado {
            original: orig.clone(),
            escansao: partes.join(" - "),
            classificacao: class_name,
            icone,
            total_metricas: esc.total_metricas,
            total_foneticas: esc.silabas.len(),
            tonicas: esc.esquema_tonico.clone(),
            silabas,
            rima: esc.rima.clone(),
            rima_terminacao: esc.rima_terminacao.clone(),
        });
    }

    let web_out = AnaliseWeb {
        versos: resultado,
        esquema_rima: poema.esquema_rima.clone(),
    };

    serde_json::to_string(&web_out).unwrap_or_else(|_| "{}".to_string())
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

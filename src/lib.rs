//! # lirica
//!
//! WebAssembly presentation layer for the `joaju-lirica` poetic engine.

use joaju_lirica::{
    avaliar_forma_poetica, avaliar_qualidade_rimas, escandir_poema, icone_metro,
    parse_lirica_block, render_painel, render_typst, DiagnosticoForma, InfoAvaliacaoRima, Poema,
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

/// Layout information for a stanza.
#[derive(Serialize, Deserialize)]
pub struct EstrofeWeb {
    /// Zero-based index of the stanza.
    pub indice: usize,
    /// Verse indices in the `versos` array that belong to this stanza.
    pub versos_indices: Vec<usize>,
    /// Number of verses in this stanza.
    pub total_versos: usize,
}

/// Complete poem analysis output for the web interface.
#[derive(Serialize, Deserialize)]
pub struct AnaliseWeb {
    /// Verses formatted with metric and rhyme data.
    pub versos: Vec<VersoFormatado>,
    /// Stanzas layout.
    pub estrofes: Vec<EstrofeWeb>,
    /// Overall poem rhyme scheme (e.g. "ABBA", "ABCB").
    pub esquema_rima: Option<String>,
    /// Detected poetic form and fuzzy adherence score (e.g. Décima Espinela, Soneto).
    pub forma_poetica: Option<DiagnosticoForma>,
    /// Qualitative and acoustic evaluation of rhymes.
    pub avaliacao_rima: Option<InfoAvaliacaoRima>,
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

    let forma_poetica = avaliar_forma_poetica(poema);
    let avaliacao_rima = if poema.esquema_rima.is_some() {
        Some(avaliar_qualidade_rimas(poema))
    } else {
        None
    };

    let mut estrofes = Vec::new();
    let mut v_idx = 0;
    for (i, e) in poema.estrofes.iter().enumerate() {
        let count = e.versos.len();
        let indices = (v_idx..v_idx + count).collect();
        v_idx += count;
        estrofes.push(EstrofeWeb {
            indice: i,
            versos_indices: indices,
            total_versos: count,
        });
    }

    let web_out = AnaliseWeb {
        versos: resultado,
        estrofes,
        esquema_rima: poema.esquema_rima.clone(),
        forma_poetica,
        avaliacao_rima,
    };

    serde_json::to_string(&web_out).unwrap_or_else(|_| "{}".to_string())
}

/// Returns a JSON list of pre-configured classical poetic examples without `%%` directives.
#[wasm_bindgen]
pub fn carregar_exemplos() -> String {
    let exemplos = vec![
        ExemploPoema {
            id: "camoes".to_string(),
            titulo: "Amor é fogo que arde sem se ver (Soneto Italiano)".to_string(),
            autor: "Luís de Camões".to_string(),
            descricao: "Soneto Petrarquiano (14 versos decassílabos em 4+4+3+3, rimas ABBA ABBA CDC DCD)".to_string(),
            texto: "Amor é fogo que arde sem se ver,\nÉ ferida que dói e não se sente;\nÉ um contentamento descontente,\nÉ dor que desatina sem doer.\n\nÉ um não querer mais que bem querer;\nÉ um andar solitário entre a gente;\nÉ nunca contentar-se de contente;\nÉ um cuidar que ganha em se perder.\n\nÉ querer estar preso por vontade;\nÉ servir a quem vence, o vencedor;\nÉ ter com quem nos mata, lealdade.\n\nMas como causar pode seu favor\nNos corações humanos amizade,\nSe tão contrário a si é o mesmo Amor?".to_string(),
        },
        ExemploPoema {
            id: "espinela".to_string(),
            titulo: "Ninguém viva descurado (Décima Espinela)".to_string(),
            autor: "Vicente Espinel / Tradição Clássica".to_string(),
            descricao: "Décima Espinela (10 versos em Redondilha Maior com rimas estritas ABBAACCDDC)".to_string(),
            texto: "Ninguém viva descurado,\nQue a fortuna é mui mudável;\nNão há glória tão louvável\nQue não pare em triste estado.\nO bem é pouco durado,\nO mal dura e cresce tanto,\nQue torna o prazer em pranto;\nE quem vive mais contente\nAcha no fim mui patente\nCausa de perpétuo espanto.".to_string(),
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

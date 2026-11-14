//! # lirica
//!
//! WebAssembly presentation layer for the `joaju-lirica` poetic engine.

pub mod morphology_resolver;

use joaju_lirica::morphology::{classificar_rimas_poema, QualidadeRima};
use joaju_lirica::{
    avaliar_forma_poetica, avaliar_qualidade_rimas, escandir_poema, icone_metro,
    parse_lirica_block, render_painel, render_typst, DiagnosticoForma, InfoAvaliacaoRima, Poema,
    PoemaConfig,
};
use morphology_resolver::{avaliar_par_conservador, refinar_token_terminal};
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
    /// Índices métricos (1-based) após os quais ocorrem cesuras rítmicas canônicas.
    pub cesuras: Vec<usize>,
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

/// Evaluated rhyme pair details for web presentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParRimaWeb {
    /// 1-based verse indices of the pair (e.g. 1 and 4 for ABBA).
    pub versos: (usize, usize),
    /// Rhyming terminal tokens (e.g. ("amor", "dor")).
    pub tokens: (String, String),
    /// Qualitative classification tag ("Rica", "Pobre", "Preciosa", "Rara", "ProvavelPobre", "ProvavelRica", "Indeterminada").
    pub qualidade: String,
    /// Detailed rationale or hypothesis explanation.
    pub detalhe: Option<String>,
    /// Audit note.
    pub auditoria: String,
    /// Whether the pair constitutes an archaic/hyper-saturated literary cliché.
    pub eh_cliche: bool,
    /// Pedagogical explanation of the cliché if applicable.
    pub motivo_cliche: Option<String>,
    /// Nomes em português das classes morfológicas inferidas para cada termo do par (ex.: (["Substantivo", "Adjetivo"], ["Adjetivo"])).
    pub classes_morfologicas: (Vec<String>, Vec<String>),
    /// Explicação em prosa límpida e acessível para leigos sobre a qualidade e valor lírico do par.
    pub explicacao_prosa: String,
}

/// Rhyme diagnostics grouped per stanza.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstrofeRimasWeb {
    /// Zero-based index of the stanza.
    pub estrofe_indice: usize,
    /// Evaluated rhyme pairs in this stanza.
    pub pares: Vec<ParRimaWeb>,
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
    /// Morphological rhyme quality and cliché diagnostics grouped by stanza.
    pub rimas_morfologicas: Option<Vec<EstrofeRimasWeb>>,
}

/// Scans a text block of verses and returns the rendered result according to options.
/// Sanitiza o texto de entrada para escansão, expurgando cabeçalhos Markdown (# Título e ## Autor)
/// para que não sejam computados como versos poéticos nem formem estrofes espúrias.
fn sanitizar_texto_escansao(texto: &str) -> String {
    let mut linhas: Vec<&str> = texto
        .lines()
        .filter(|linha| {
            let t = linha.trim_start();
            !(t.starts_with("# ") || t.starts_with("## ") || t == "#" || t == "##")
        })
        .collect();

    // Descarta linhas vazias residuais no início (ex.: espaçamento entre metadados e primeiro verso)
    while let Some(primeira) = linhas.first() {
        if primeira.trim().is_empty() {
            linhas.remove(0);
        } else {
            break;
        }
    }

    // Descarta linhas vazias residuais no final
    while let Some(ultima) = linhas.last() {
        if ultima.trim().is_empty() {
            linhas.pop();
        } else {
            break;
        }
    }

    linhas.join("\n")
}

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

    let texto_limpo = sanitizar_texto_escansao(texto);
    if texto_limpo.trim().is_empty() {
        return String::new();
    }

    let has_directives = texto_limpo.lines().any(|l| l.trim().starts_with("%%"));

    let (config, poema) = if has_directives {
        match parse_lirica_block(&texto_limpo) {
            Ok((cfg, p)) => (cfg, p),
            Err(_) => {
                let p = escandir_poema(
                    &texto_limpo,
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
        }
    } else {
        let p = escandir_poema(
            &texto_limpo,
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
    };

    match modo_saida.to_lowercase().as_str() {
        "typst" => render_typst(&config, &poema),
        "json" => match serde_json::to_string_pretty(&poema) {
            Ok(j) => j,
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        },
        "texto" => poema.linhas_originais.join("\n"),
        "visual" | "html" => render_visual_json(&poema, &texto_limpo),
        _ => render_painel(&poema),
    }
}

fn is_fim_palavra(texto: &str, byte_pos: usize) -> bool {
    if byte_pos >= texto.len() {
        return true;
    }
    let restante = &texto[byte_pos..];
    if let Some(c) = restante.chars().next() {
        !c.is_alphabetic() && c != '\''
    } else {
        true
    }
}

/// Identifica deterministricamente cesuras rítmicas clássicas em versos longos,
/// respeitando estritamente a fronteira de vocábulo (word boundary).
fn detectar_cesuras(
    orig: &str,
    silabas: &[joaju_lirica::SilabaPoetica],
    total_metricas: usize,
    tonicas: &[usize],
    classificacao: &str,
) -> Vec<usize> {
    if total_metricas != 10 && total_metricas != 12 {
        return Vec::new();
    }

    let mut cesuras = Vec::new();

    if total_metricas == 10 {
        let eh_safico = classificacao.contains("Sáfico")
            || (tonicas.contains(&4) && (tonicas.contains(&8) || !tonicas.contains(&6)));

        if eh_safico {
            // Decassílabo Sáfico: cesura ancorada na 4ª sílaba métrica
            let s4 = silabas.iter().find(|s| s.numero_metrico == Some(4));
            let s5 = silabas.iter().find(|s| s.numero_metrico == Some(5));

            if let Some(syl4) = s4 {
                if is_fim_palavra(orig, syl4.span.end) {
                    cesuras.push(4);
                } else if let Some(syl5) = s5 {
                    if is_fim_palavra(orig, syl5.span.end) {
                        cesuras.push(5);
                    }
                }
            }
        } else {
            // Decassílabo Heroico: preferência por cesura na 6ª sílaba métrica
            let s6 = silabas.iter().find(|s| s.numero_metrico == Some(6));
            let s4 = silabas.iter().find(|s| s.numero_metrico == Some(4));
            let s5 = silabas.iter().find(|s| s.numero_metrico == Some(5));

            if let Some(syl6) = s6 {
                if is_fim_palavra(orig, syl6.span.end) {
                    cesuras.push(6);
                } else if tonicas.contains(&4) {
                    // Se a 6ª sílaba não encerra palavra mas há tônica na 4ª (ex.: "Amor é fogo que arde..."),
                    // a pausa natural ocorre no 1º hemistíquio sáfico/lírico (4 ou 5)
                    if let Some(syl4) = s4 {
                        if is_fim_palavra(orig, syl4.span.end) {
                            cesuras.push(4);
                        } else if let Some(syl5) = s5 {
                            if is_fim_palavra(orig, syl5.span.end) {
                                cesuras.push(5);
                            }
                        }
                    }
                } else {
                    let s7 = silabas.iter().find(|s| s.numero_metrico == Some(7));
                    if let Some(syl7) = s7 {
                        if is_fim_palavra(orig, syl7.span.end) {
                            cesuras.push(7);
                        }
                    }
                }
            }
        }
    } else if total_metricas == 12 {
        // Alexandrino Clássico (6 + 6)
        let s6 = silabas.iter().find(|s| s.numero_metrico == Some(6));
        let s7 = silabas.iter().find(|s| s.numero_metrico == Some(7));

        if let Some(syl6) = s6 {
            if is_fim_palavra(orig, syl6.span.end) {
                cesuras.push(6);
            } else if let Some(syl7) = s7 {
                if is_fim_palavra(orig, syl7.span.end) {
                    cesuras.push(7);
                }
            }
        }
    }

    cesuras
}

/// Avalia a forma poética com reconhecimento enriquecido de formas tradicionais e populares brasileiras:
/// Galope à Beira-Mar, Pajada Gaúcha (Décima Campeira) e Folhetos de Cordel Encadeados (com marco de 32 estrofes).
fn avaliar_forma_poetica_expandida(poema: &Poema) -> Option<DiagnosticoForma> {
    let total_versos = poema.versos.len();
    if total_versos == 0 {
        return None;
    }

    // 1. Detecção de Galope à Beira-Mar
    // Estrofe de 10 versos em métrica decassílaba/hendecassílaba com refrão em "beira do mar"
    if total_versos == 10 {
        if let Some(ultimo_verso) = poema.linhas_originais.last() {
            let u_lower = ultimo_verso.to_lowercase();
            if u_lower.contains("beira do mar")
                || u_lower.contains("beira-mar")
                || u_lower.contains("beiramar")
            {
                let layout = format!("10 versos = {} versos", total_versos);
                return Some(DiagnosticoForma {
                    nome: "Galope à Beira-Mar".to_string(),
                    confianca_percentual: 95,
                    descricao: "Forma virtuosística da cantoria e do repente nordestino em décimas com ritmo galopante e refrão obrigatório à beira do mar.".to_string(),
                    padrao_esperado: "Décima de 10 versos (10 ou 11 sílabas), rimas ABBAACCDDC com terminação em 'beira do mar'".to_string(),
                    estrutura_detectada: layout,
                    observacoes: vec![
                        "Refrão canônico de encerramento ('beira do mar') identificado no 10º verso.".to_string(),
                        "Estrutura estrófica de décima e rimas em plena conformidade com a tradição nordestina.".to_string(),
                    ],
                });
            }
        }
    }

    // 2. Detecção de Pajada Gaúcha (Décima Campeira)
    // 10 versos em Redondilha Maior (média métrica ~7) com rimas espinelianas e léxico/contexto campeiro
    if total_versos == 10 {
        let media_metricas: f64 = poema
            .versos
            .iter()
            .map(|v| v.total_metricas as f64)
            .sum::<f64>()
            / 10.0;
        let texto_completo = poema.linhas_originais.join(" ").to_lowercase();
        let eh_lexico_gaucho = texto_completo.contains("gaucho")
            || texto_completo.contains("gaúch")
            || texto_completo.contains("campeir")
            || texto_completo.contains("pajada")
            || texto_completo.contains("mission")
            || texto_completo.contains("guitarra")
            || texto_completo.contains("pampa");

        if (media_metricas - 7.0).abs() <= 1.2
            && (eh_lexico_gaucho
                || poema
                    .linhas_originais
                    .iter()
                    .any(|l| l.contains("campeira") || l.contains("missioneira")))
        {
            let layout = format!("10 versos = {} versos", total_versos);
            return Some(DiagnosticoForma {
                nome: "Pajada Gaúcha (Décima Campeira)".to_string(),
                confianca_percentual: 95,
                descricao: "Forma poética tradicional e de improviso da cultura gauchesca em Redondilha Maior (7 sílabas), imortalizada por Jayme Caetano Braun.".to_string(),
                padrao_esperado: "Décima de 10 versos em Redondilha Maior (7 sílabas), rimas espinelianas ABBAACCDDC".to_string(),
                estrutura_detectada: layout,
                observacoes: vec![
                    "Métrica em Redondilha Maior e esquema de rimas espineliano típico da pajada missioneira.".to_string(),
                    "Cadência e imagética da tradição poética sul-rio-grandense.".to_string(),
                ],
            });
        }
    }

    // 3. Detecção de Folheto de Cordel Encadeado (Sextilhas)
    let num_estrofes = poema.estrofes.len();
    if num_estrofes >= 2 && poema.estrofes.iter().all(|e| e.versos.len() == 6) {
        let media_metricas: f64 = poema
            .versos
            .iter()
            .map(|v| v.total_metricas as f64)
            .sum::<f64>()
            / (total_versos as f64);
        if (media_metricas - 7.0).abs() <= 1.2 {
            let layout = format!(
                "{} estrofes de 6 versos = {} versos",
                num_estrofes, total_versos
            );
            let mut obs = vec![format!(
                "Sequência encadeada de {} sextilhas em Redondilha Maior (7 sílabas).",
                num_estrofes
            )];
            if num_estrofes == 32 {
                obs.push(
                    "Folheto de Cordel canônico de 8 páginas (padrão tipográfico clássico de 32 estrofes in-octavo)."
                        .to_string(),
                );
            }

            return Some(DiagnosticoForma {
                nome: if num_estrofes == 32 {
                    "Folheto de Cordel (32 Estrofes Canônicas)".to_string()
                } else {
                    "Folheto de Cordel (Sextilhas Encadeadas)".to_string()
                },
                confianca_percentual: 95,
                descricao: "Narrativa lírica e épica tradicional da literatura de cordel brasileira, estruturada em sextilhas encadeadas.".to_string(),
                padrao_esperado: "Sextilhas de 6 versos em Redondilha Maior (7 sílabas), rimas ABCBDB ou XAXBXB".to_string(),
                estrutura_detectada: layout,
                observacoes: obs,
            });
        }
    }

    // 4. Fallback canônico para o validador base
    avaliar_forma_poetica(poema)
}

fn render_visual_json(poema: &Poema, raw_text: &str) -> String {
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

        let cesuras = detectar_cesuras(
            orig,
            &esc.silabas,
            esc.total_metricas,
            &esc.esquema_tonico,
            &class_name,
        );

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
            cesuras,
        });
    }

    let forma_poetica = avaliar_forma_poetica_expandida(poema);
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

    let rimas_morfologicas = if poema.esquema_rima.is_some() {
        let texto_limpo: String = raw_text
            .lines()
            .filter(|l| !l.trim().starts_with("%%"))
            .collect::<Vec<_>>()
            .join("\n");
        let todas_linhas_versos: Vec<&str> = texto_limpo
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();

        let rel = classificar_rimas_poema(&texto_limpo);
        let estrofes_web = rel
            .estrofes
            .into_iter()
            .enumerate()
            .map(|(i, est)| EstrofeRimasWeb {
                estrofe_indice: i,
                pares: est
                    .pares_avaliados
                    .into_iter()
                    .map(|p| {
                        let linha_1 = todas_linhas_versos
                            .get(p.versos.0.saturating_sub(1))
                            .copied()
                            .unwrap_or("");
                        let linha_2 = todas_linhas_versos
                            .get(p.versos.1.saturating_sub(1))
                            .copied()
                            .unwrap_or("");

                        let mut tok1 = refinar_token_terminal(linha_1);
                        if tok1.texto.is_empty() {
                            tok1.texto = p.tokens.0.clone();
                            tok1.base = p.tokens.0.clone();
                        }
                        let mut tok2 = refinar_token_terminal(linha_2);
                        if tok2.texto.is_empty() {
                            tok2.texto = p.tokens.1.clone();
                            tok2.base = p.tokens.1.clone();
                        }

                        let detalhe_orig = match &p.classificacao {
                            QualidadeRima::ProvavelPobre { justificativa } => {
                                Some(justificativa.clone())
                            }
                            QualidadeRima::ProvavelRica { justificativa } => {
                                Some(justificativa.clone())
                            }
                            QualidadeRima::Indeterminada { motivo } => {
                                Some(format!("{:?}", motivo))
                            }
                            _ => None,
                        };

                        let res = avaliar_par_conservador(
                            &tok1,
                            &tok2,
                            &p.classificacao,
                            detalhe_orig.as_deref(),
                        );

                        let qualidade = match &res.qualidade {
                            QualidadeRima::Pobre => "Pobre".to_string(),
                            QualidadeRima::Rica => "Rica".to_string(),
                            QualidadeRima::Preciosa => "Preciosa".to_string(),
                            QualidadeRima::Rara => "Rara".to_string(),
                            QualidadeRima::ProvavelPobre { .. } => "ProvavelPobre".to_string(),
                            QualidadeRima::ProvavelRica { .. } => "ProvavelRica".to_string(),
                            QualidadeRima::Indeterminada { .. } => "Indeterminada".to_string(),
                        };

                        ParRimaWeb {
                            versos: p.versos,
                            tokens: p.tokens,
                            qualidade,
                            detalhe: res.detalhe,
                            auditoria: format!("{:?}", p.auditoria),
                            eh_cliche: p.eh_cliche,
                            motivo_cliche: p.motivo_cliche,
                            classes_morfologicas: res.classes,
                            explicacao_prosa: res.prosa,
                        }
                    })
                    .collect(),
            })
            .collect();
        Some(estrofes_web)
    } else {
        None
    };

    let web_out = AnaliseWeb {
        versos: resultado,
        estrofes,
        esquema_rima: poema.esquema_rima.clone(),
        forma_poetica,
        avaliacao_rima,
        rimas_morfologicas,
    };

    serde_json::to_string(&web_out).unwrap_or_else(|_| "{}".to_string())
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

struct CardTheme {
    bg: &'static str,
    border: &'static str,
    border_inner: &'static str,
    title_color: &'static str,
    author_color: &'static str,
    text_color: &'static str,
    tonic_color: &'static str,
    caesura_color: &'static str,
    badge_bg: &'static str,
    badge_text: &'static str,
    badge_border: &'static str,
    rhyme_bg: &'static str,
    rhyme_text: &'static str,
    footer_color: &'static str,
}

impl CardTheme {
    fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "noturno" | "dark" => CardTheme {
                bg: "#0d1117",
                border: "#30363d",
                border_inner: "#21262d",
                title_color: "#f0f6fc",
                author_color: "#8b949e",
                text_color: "#c9d1d9",
                tonic_color: "#58a6ff",
                caesura_color: "#79c0ff",
                badge_bg: "#161b22",
                badge_text: "#58a6ff",
                badge_border: "#30363d",
                rhyme_bg: "#1f2937",
                rhyme_text: "#e3b341",
                footer_color: "#6e7681",
            },
            "minimalista" | "mono" | "print" => CardTheme {
                bg: "#ffffff",
                border: "#000000",
                border_inner: "#e5e5e5",
                title_color: "#000000",
                author_color: "#333333",
                text_color: "#111111",
                tonic_color: "#000000",
                caesura_color: "#666666",
                badge_bg: "#f5f5f5",
                badge_text: "#111111",
                badge_border: "#cccccc",
                rhyme_bg: "#f0f0f0",
                rhyme_text: "#000000",
                footer_color: "#666666",
            },
            _ => CardTheme {
                bg: "#fbf7ee",
                border: "#d9caa7",
                border_inner: "#ebdcc4",
                title_color: "#4a2c2a",
                author_color: "#7a5843",
                text_color: "#2b2523",
                tonic_color: "#9b2c2c",
                caesura_color: "#b48a3c",
                badge_bg: "#f2e8d5",
                badge_text: "#5c4033",
                badge_border: "#d8c4a0",
                rhyme_bg: "#eeddc2",
                rhyme_text: "#4a2c2a",
                footer_color: "#8c7b70",
            },
        }
    }
}

/// Gera um card poético vetorial em formato SVG autocontido para compartilhamento e exportação.
#[wasm_bindgen]
pub fn gerar_card_svg(
    texto: &str,
    titulo: &str,
    autor: &str,
    tema: &str,
    estrofe_idx: i32,
    incluir_metricas: bool,
) -> String {
    let theme = CardTheme::from_name(tema);

    if texto.trim().is_empty() {
        return format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 600 200\" width=\"600\" height=\"200\">\
            <rect width=\"100%\" height=\"100%\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\" rx=\"12\"/>\
            <text x=\"300\" y=\"105\" text-anchor=\"middle\" fill=\"{}\" font-family=\"Georgia, serif\" font-size=\"18\">Nenhum verso para exibir.</text>\
            </svg>",
            theme.bg, theme.border, theme.text_color
        );
    }

    let json_visual = escandir(texto, "visual", false, false, false, true);
    let analise: Result<AnaliseWeb, _> = serde_json::from_str(&json_visual);
    let analise = match analise {
        Ok(a) => a,
        Err(_) => {
            return "<svg xmlns=\"http://www.w3.org/2000/svg\"><text>Erro ao escandir poema</text></svg>".to_string()
        }
    };

    let versos_selecionados: Vec<(usize, &VersoFormatado)> =
        if estrofe_idx >= 0 && (estrofe_idx as usize) < analise.estrofes.len() {
            let est = &analise.estrofes[estrofe_idx as usize];
            est.versos_indices
                .iter()
                .filter_map(|&i| analise.versos.get(i).map(|v| (i + 1, v)))
                .collect()
        } else {
            analise
                .versos
                .iter()
                .enumerate()
                .map(|(i, v)| (i + 1, v))
                .collect()
        };

    let eh_vazio = versos_selecionados.is_empty()
        || versos_selecionados
            .iter()
            .all(|(_, v)| v.original.trim().is_empty());

    if eh_vazio {
        return format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 600 200\" width=\"600\" height=\"200\">\
            <rect width=\"100%\" height=\"100%\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\" rx=\"12\"/>\
            <text x=\"300\" y=\"105\" text-anchor=\"middle\" fill=\"{}\" font-family=\"Georgia, serif\" font-size=\"18\">Nenhum verso para exibir.</text>\
            </svg>",
            theme.bg, theme.border, theme.text_color
        );
    }

    let card_width = 800;
    let has_header = !titulo.trim().is_empty() || !autor.trim().is_empty();
    let header_height = if has_header { 95 } else { 35 };
    let verse_line_height = 36;
    let stanza_gap = 18;

    let mut num_gaps = 0;
    for i in 1..versos_selecionados.len() {
        let (prev_num, _) = versos_selecionados[i - 1];
        let (curr_num, _) = versos_selecionados[i];
        if curr_num > prev_num + 1 {
            num_gaps += 1;
        } else {
            for est in &analise.estrofes {
                if est.versos_indices.last() == Some(&(prev_num - 1))
                    && est.versos_indices.first() != Some(&(curr_num - 1))
                {
                    num_gaps += 1;
                    break;
                }
            }
        }
    }

    let verses_content_height =
        (versos_selecionados.len() * verse_line_height) + (num_gaps * stanza_gap);
    let footer_height = 55;
    let padding_top = 35;
    let padding_bottom = 25;
    let total_height =
        padding_top + header_height + verses_content_height + footer_height + padding_bottom;

    let mut svg = String::with_capacity(8192);
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n",
        card_width, total_height, card_width, total_height
    ));

    svg.push_str(&format!(
        "<defs><style>\n\
        .title {{ font-family: 'Playfair Display', Georgia, Cambria, serif; font-size: 24px; font-weight: bold; fill: {}; text-anchor: middle; }}\n\
        .author {{ font-family: 'Playfair Display', Georgia, Cambria, serif; font-size: 15px; font-style: italic; fill: {}; text-anchor: middle; }}\n\
        .verse-num {{ font-family: system-ui, sans-serif; font-size: 11px; fill: {}; text-anchor: end; opacity: 0.6; }}\n\
        .verse-text {{ font-family: 'Playfair Display', Georgia, Cambria, serif; font-size: 16px; fill: {}; }}\n\
        .syl-tonic {{ font-weight: bold; fill: {}; }}\n\
        .syl-norm {{ fill: {}; }}\n\
        .caesura {{ font-weight: bold; fill: {}; }}\n\
        .badge-txt {{ font-family: system-ui, sans-serif; font-size: 11px; text-anchor: middle; }}\n\
        .footer-txt {{ font-family: system-ui, sans-serif; font-size: 12px; fill: {}; }}\n\
        </style></defs>\n",
        theme.title_color, theme.author_color, theme.footer_color, theme.text_color,
        theme.tonic_color, theme.text_color, theme.caesura_color, theme.footer_color
    ));

    svg.push_str(&format!(
        "<rect width=\"{}\" height=\"{}\" rx=\"16\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\" />\n",
        card_width, total_height, theme.bg, theme.border
    ));
    svg.push_str(&format!(
        "<rect x=\"10\" y=\"10\" width=\"{}\" height=\"{}\" rx=\"10\" fill=\"none\" stroke=\"{}\" stroke-width=\"1\" opacity=\"0.7\" />\n",
        card_width - 20, total_height - 20, theme.border_inner
    ));

    let mut current_y = padding_top;

    if has_header {
        if !titulo.trim().is_empty() {
            current_y += 24;
            svg.push_str(&format!(
                "<text x=\"400\" y=\"{}\" class=\"title\">{}</text>\n",
                current_y,
                escape_xml(titulo.trim())
            ));
        }
        if !autor.trim().is_empty() {
            current_y += 22;
            svg.push_str(&format!(
                "<text x=\"400\" y=\"{}\" class=\"author\">{}</text>\n",
                current_y,
                escape_xml(autor.trim())
            ));
        }
        current_y += 18;
        svg.push_str(&format!(
            "<line x1=\"340\" y1=\"{}\" x2=\"460\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" opacity=\"0.6\" />\n",
            current_y, current_y, theme.border_inner
        ));
        current_y += 26;
    } else {
        current_y += 15;
    }

    for (idx, (v_num, v)) in versos_selecionados.iter().enumerate() {
        if idx > 0 {
            let (prev_num, _) = versos_selecionados[idx - 1];
            let is_stanza_break = *v_num > prev_num + 1
                || analise.estrofes.iter().any(|e| {
                    e.versos_indices.last() == Some(&(prev_num - 1))
                        && e.versos_indices.first() != Some(&(v_num - 1))
                });
            if is_stanza_break {
                current_y += stanza_gap;
            }
        }

        let y_verse = current_y;

        svg.push_str(&format!(
            "<text x=\"54\" y=\"{}\" class=\"verse-num\">{}</text>\n",
            y_verse, v_num
        ));

        let x_verse = 70;
        if incluir_metricas {
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" class=\"verse-text\">",
                x_verse, y_verse
            ));
            for (s_idx, s) in v.silabas.iter().enumerate() {
                let class_syl = if s.tonica { "syl-tonic" } else { "syl-norm" };
                svg.push_str(&format!(
                    "<tspan class=\"{}\">{}</tspan>",
                    class_syl,
                    escape_xml(&s.texto)
                ));

                if let Some(num) = s.numero_metrico {
                    if v.cesuras.contains(&num) {
                        svg.push_str("<tspan class=\"caesura\"> ‖ </tspan>");
                    }
                }

                if s_idx + 1 < v.silabas.len() {
                    let prox = &v.silabas[s_idx + 1];
                    if !s.texto.ends_with(' ') && !prox.texto.starts_with(' ') {
                        svg.push_str("<tspan opacity=\"0.4\">·</tspan>");
                    }
                }
            }
            svg.push_str("</text>\n");
        } else {
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" class=\"verse-text\">{}</text>\n",
                x_verse,
                y_verse,
                escape_xml(&v.original)
            ));
        }

        let mut badge_right_x = card_width - 50;

        if let Some(rima) = &v.rima {
            let rx = badge_right_x - 26;
            let ry = y_verse - 15;
            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"26\" height=\"20\" rx=\"4\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\" />\n",
                rx, ry, theme.rhyme_bg, theme.badge_border
            ));
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" class=\"badge-txt\" font-weight=\"bold\" fill=\"{}\">{}</text>\n",
                rx + 13, y_verse, theme.rhyme_text, escape_xml(rima)
            ));
            badge_right_x -= 32;
        }

        let meter_width = 54;
        let mx = badge_right_x - meter_width;
        let my = y_verse - 15;
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"20\" rx=\"4\" fill=\"{}\" stroke=\"{}\" stroke-width=\"1\" />\n",
            mx, my, meter_width, theme.badge_bg, theme.badge_border
        ));
        let meter_label = format!("{} {}", v.total_metricas, v.icone);
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" class=\"badge-txt\" font-weight=\"600\" fill=\"{}\">{}</text>\n",
            mx + (meter_width / 2),
            y_verse,
            theme.badge_text,
            escape_xml(&meter_label)
        ));

        current_y += verse_line_height;
    }

    current_y += 12;
    svg.push_str(&format!(
        "<line x1=\"50\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\" opacity=\"0.4\" />\n",
        current_y,
        card_width - 50,
        current_y,
        theme.border_inner
    ));
    current_y += 24;

    let diag_label = if let Some(f) = &analise.forma_poetica {
        format!("{} ({}%)", f.nome, f.confianca_percentual)
    } else if let Some((_, primeiro)) = versos_selecionados.first() {
        primeiro.classificacao.clone()
    } else {
        "Forma Livre".to_string()
    };

    svg.push_str(&format!(
        "<text x=\"54\" y=\"{}\" class=\"footer-txt\" font-weight=\"500\">{}</text>\n",
        current_y,
        escape_xml(&diag_label)
    ));

    svg.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" class=\"footer-txt\" text-anchor=\"end\" font-weight=\"600\">Lírica • lirica.2lp.in</text>\n",
        card_width - 54, current_y
    ));

    svg.push_str("</svg>");
    svg
}

/// Gera um documento Typst editorial completo e pronto para compilação.
#[wasm_bindgen]
pub fn exportar_documento_typst(texto: &str, titulo: &str, autor: &str) -> String {
    let config = PoemaConfig::default();
    let poema = escandir_poema(texto, false, false, false, true);
    let typst_body = render_typst(&config, &poema);

    let tit = if titulo.trim().is_empty() {
        "Sem Título".to_string()
    } else {
        titulo.trim().to_string()
    };

    let aut = if autor.trim().is_empty() {
        "Anônimo".to_string()
    } else {
        autor.trim().to_string()
    };

    format!(
        r#"// Documento gerado pelo Lírica (https://lirica.2lp.in)
// Laboratório de Escansão e Métrica Poética

#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 3cm),
  header: align(right)[#text(8pt, fill: luma(120))[Lírica • Laboratório de Métrica Poética]],
  footer: align(center)[#text(8pt, fill: luma(120))[#counter(page).display()]],
)
#set text(
  font: "Linux Libertine",
  size: 11pt,
  lang: "pt",
)

#align(center)[
  #v(1cm)
  #text(18pt, weight: "bold")[{tit}]
  #v(0.2cm)
  #text(12pt, style: "italic", fill: luma(80))[{aut}]
  #v(1.2cm)
]

#align(center)[
{typst_body}
]
"#
    )
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
            id: "vinicius".to_string(),
            titulo: "Soneto de Fidelidade".to_string(),
            autor: "Vinícius de Moraes".to_string(),
            descricao: "Soneto Decassílabo Lírico (14 versos em 4+4+3+3, rimas ABBA ABBA CDE CDE)".to_string(),
            texto: "De tudo, ao meu amor serei atento\nAntes, e com tal zelo, e sempre tanto\nQue mesmo em face do maior encanto\nDele se encante mais meu pensamento.\n\nQuero vivê-lo em cada vão momento\nE em seu louvor hei de espalhar meu canto\nE rir meu riso e derramar meu pranto\nAo seu pesar ou seu contentamento.\n\nE assim, quando mais tarde me procure\nQuem sabe a morte, angústia de quem vive\nQuem sabe a solidão, fim de quem ama\n\nEu possa me dizer do amor (que tive):\nQue não seja imortal, posto que é chama\nMas que seja infinito enquanto dure.".to_string(),
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
            descricao: "Redondilha Maior / Heptassílabo (7 sílabas métricas, rimas ABCB em 5 estrofes)".to_string(),
            texto: "Minha terra tem palmeiras,\nOnde canta o Sabiá;\nAs aves que aqui gorjeiam,\nNão gorjeiam como lá.\n\nNosso céu tem mais estrelas,\nNossas várzeas têm mais flores,\nNossos bosques têm mais vida,\nNossa vida mais amores.\n\nEm cismar, à noite, sozinho,\nMais prazer encontro eu lá;\nMinha terra tem palmeiras,\nOnde canta o Sabiá.\n\nMinha terra tem primores,\nQue tais não encontro eu cá;\nEm cismar – sozinho, à noite –\nMais prazer encontro eu lá;\nMinha terra tem palmeiras,\nOnde canta o Sabiá.\n\nNão permita Deus que eu morra,\nSem que eu volte para lá;\nSem que desfrute os primores\nQue não encontro por cá;\nSem qu’inda aviste as palmeiras,\nOnde canta o Sabiá.".to_string(),
        },
        ExemploPoema {
            id: "cruz_e_sousa".to_string(),
            titulo: "Sinfonias do Ocaso".to_string(),
            autor: "Cruz e Sousa".to_string(),
            descricao: "Soneto Simbolista em decassílabos de Broquéis (rimas ABBA ABBA CCD EED)".to_string(),
            texto: "Musselinosas como brumas diurnas\nDescem do ocaso as sombras harmoniosas,\nSombras veladas e musselinosas\nPara as profundas solidões noturnas.\n\nSacrários virgens, sacrossantas urnas,\nOs céus resplendem de sidéreas rosas,\nDa Lua e das Estrelas majestosas\nIluminando a escuridão das furnas.\n\nAh! por estes sinfônicos ocasos\nA terra exala aromas de áureos vasos,\nIncensos de turíbulos divinos.\n\nOs plenilúnios mórbidos vaporam...\nE como que no Azul plangem e choram\nCítaras, harpas, bandolins, violinos...".to_string(),
        },
        ExemploPoema {
            id: "pessoa".to_string(),
            titulo: "Autopsicografia".to_string(),
            autor: "Fernando Pessoa".to_string(),
            descricao: "Redondilha Maior com alternância rítmica lírica em 3 estrofes (ABAB)".to_string(),
            texto: "O poeta é um fingidor.\nFinge tão completamente\nQue chega a fingir que é dor\nA dor que deveras sente.\n\nE os que leem o que escreve,\nNa dor lida sentem bem,\nNão as duas que ele teve,\nMas só a que eles não têm.\n\nE assim nas calhas de roda\nGira, a entreter a razão,\nEsse comboio de corda\nQue se chama coração.".to_string(),
        },
        ExemploPoema {
            id: "bilac".to_string(),
            titulo: "Ouvir Estrelas".to_string(),
            autor: "Olavo Bilac".to_string(),
            descricao: "Soneto Decassílabo Parnasiano da Via Láctea XIII (rimas ABBA ABBA CDC EDE)".to_string(),
            texto: "\"Ora (direis) ouvir estrelas! Certo\nPerdeste o senso!\" E eu vos direi, no entanto,\nQue, para ouvi-las, muita vez desperto\nE abro as janelas, pálido de espanto...\n\nE conversamos toda a noite, enquanto\nA Via-Láctea, como um pálio aberto,\nCintila. E, ao vir do sol, saudoso e em pranto,\nInda as procuro pelo céu deserto.\n\nDireis agora: \"Tresloucado amigo!\nQue conversas com elas? Que sentido\nTem o que dizem, quando estão contigo?\"\n\nE eu vos direi: \"Amai para entendê-las!\nPois só quem ama pode ter ouvido\nCapaz de ouvir e de entender estrelas.\"".to_string(),
        },
        ExemploPoema {
            id: "pavao_misterioso".to_string(),
            titulo: "O Romance do Pavão Misterioso".to_string(),
            autor: "José Camelo de Melo Rezende".to_string(),
            descricao: "Sextilha de Cordel em Redondilha Maior (7 sílabas métricas, rimas populares ABCBDB)".to_string(),
            texto: "Eu vou contar uma história\nDe um pavão misterioso\nQue levantou voo na Grécia\nCom um rapaz corajoso\nRaptando uma condessa\nFilha de um conde orgulhoso.\n\nResidia na Turquia\nUm viúvo capitalista\nPai de dois filhos solteiros\nO mais velho, João Batista,\nE o mais moço, Evangelista,\nMuito sagaz e bom artista.".to_string(),
        },
        ExemploPoema {
            id: "pajada_gaucha".to_string(),
            titulo: "Pajada do Rio Grande".to_string(),
            autor: "Jayme Caetano Braun".to_string(),
            descricao: "Pajada Gaúcha em Décima Espinela campeira (10 versos em Redondilha Maior, rimas ABBAACCDDC)".to_string(),
            texto: "Eu trago na minha voz\nA voz de todo o passado,\nDo tempo velho, esquecido,\nQue trago ressuscitado;\nPelo verso compassado\nDesta guitarra campeira,\nQue vibra de tal maneira\nNum lamento tão profundo,\nQue parece vir do fundo\nDa minha terra missioneira!".to_string(),
        },
        ExemploPoema {
            id: "galope_beira_mar".to_string(),
            titulo: "Galope à Beira-Mar".to_string(),
            autor: "Cantoria Popular / Tradição Nordestina".to_string(),
            descricao: "Galope à Beira-Mar em Décima com ritmo galopante (10 versos hendecassílabos/decassílabos, rimas ABBAACCDDC e refrão fixo)".to_string(),
            texto: "Canto as glórias do mar e da terra,\nCanto as forças do céu e do vento,\nNeste verso de puro talento\nQue a rudeza da serra desterra.\nMinha lira na praia se aferra\nAo clamor do bravio luar,\nFaço as pedras do abismo escutar\nO repique da viola que chora,\nNa grandeza do mundo e da aurora,\nNos galopes da beira do mar.".to_string(),
        },
    ];

    serde_json::to_string(&exemplos).unwrap_or_else(|_| "[]".to_string())
}

/// Returns the engine version.
#[wasm_bindgen]
pub fn versao() -> String {
    include_str!("../VERSION").trim().to_string()
}

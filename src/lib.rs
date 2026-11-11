//! # lirica
//!
//! WebAssembly presentation layer for the `joaju-lirica` poetic engine.

use joaju_lirica::morphology::{classificar_rimas_poema, QualidadeRima};
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
    let linhas_filtradas: Vec<&str> = texto
        .lines()
        .filter(|linha| {
            let t = linha.trim_start();
            !(t.starts_with("# ") || t.starts_with("## ") || t == "#" || t == "##")
        })
        .collect();
    linhas_filtradas.join("\n")
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
        "visual" | "html" => render_visual_json(&poema, texto),
        _ => render_painel(&poema),
    }
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

    let rimas_morfologicas = if poema.esquema_rima.is_some() {
        let texto_limpo: String = raw_text
            .lines()
            .filter(|l| !l.trim().starts_with("%%"))
            .collect::<Vec<_>>()
            .join("\n");
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
                        let (qualidade, detalhe) = match p.classificacao {
                            QualidadeRima::Pobre => ("Pobre".to_string(), None),
                            QualidadeRima::Rica => ("Rica".to_string(), None),
                            QualidadeRima::Preciosa => ("Preciosa".to_string(), None),
                            QualidadeRima::Rara => ("Rara".to_string(), None),
                            QualidadeRima::ProvavelPobre { justificativa } => {
                                ("ProvavelPobre".to_string(), Some(justificativa))
                            }
                            QualidadeRima::ProvavelRica { justificativa } => {
                                ("ProvavelRica".to_string(), Some(justificativa))
                            }
                            QualidadeRima::Indeterminada { motivo } => {
                                ("Indeterminada".to_string(), Some(format!("{:?}", motivo)))
                            }
                        };
                        ParRimaWeb {
                            versos: p.versos,
                            tokens: p.tokens,
                            qualidade,
                            detalhe,
                            auditoria: format!("{:?}", p.auditoria),
                            eh_cliche: p.eh_cliche,
                            motivo_cliche: p.motivo_cliche,
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
    ];

    serde_json::to_string(&exemplos).unwrap_or_else(|_| "[]".to_string())
}

/// Returns the engine version.
#[wasm_bindgen]
pub fn versao() -> String {
    include_str!("../VERSION").trim().to_string()
}

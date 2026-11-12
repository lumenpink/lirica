//! Módulo de resolução conservadora de lookback sintático e convergência nominal.
//!
//! Fornece desambiguação morfológica em janela contextual de até 3 tokens (T-1, T-2, T-3)
//! e aplicação de graus epistêmicos conservadores para zonas cinzentas de substantivos
//! e adjetivos na poesia lusófona, evitando categoricamente falsos positivos.

use joaju_lirica::morphology::{
    consultar_lexico, descolar_enclise, inferir_por_sufixo, PosMask, QualidadeRima,
};

/// Informações morfológicas refinadas de um token rimante terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRefinado {
    /// Texto original.
    pub texto: String,
    /// Base normalizada (sem ênclise).
    pub base: String,
    /// Se possui ênclise ou mesóclise.
    pub tem_enclise: bool,
    /// Máscara morfológica após retroanálise.
    pub pos: PosMask,
    /// Nome da regra que refinou ou desambiguou o token.
    pub regra: Option<String>,
}

/// Avalia e refina o token terminal de um verso usando uma janela de lookback sintático ampliada.
pub fn refinar_token_terminal(verso: &str) -> TokenRefinado {
    let limpo = verso.trim();
    let palavras: Vec<&str> = limpo
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphabetic() && c != '-'))
        .filter(|w| !w.is_empty())
        .collect();

    if palavras.is_empty() {
        return TokenRefinado {
            texto: String::new(),
            base: String::new(),
            tem_enclise: false,
            pos: PosMask::UNKNOWN,
            regra: None,
        };
    }

    let t0 = palavras.last().unwrap();
    let anteriores = &palavras[..palavras.len() - 1];

    // 1. Ênclise / Mesóclise
    let (base_verbal, clitico, tem_enclise) = descolar_enclise(t0);
    if tem_enclise {
        return TokenRefinado {
            texto: (*t0).to_string(),
            base: base_verbal,
            tem_enclise: true,
            pos: PosMask::VERBO,
            regra: Some(format!("ENCLISE({})", clitico.unwrap_or_default())),
        };
    }

    // 2. Consulta léxica primária ou sufixal
    let mut pos = consultar_lexico(&base_verbal).unwrap_or(PosMask::UNKNOWN);
    let mut regra = None;

    if pos != PosMask::UNKNOWN {
        if pos.is_resolved() {
            regra = Some("DAFSA_EXACT".to_string());
        }
    } else if let Some(pos_sufixo) = inferir_por_sufixo(&base_verbal) {
        pos = pos_sufixo;
        regra = Some("SUFIXO_PRODUTIVO".to_string());
    }

    // 3. Lookback sintático ampliado (janela de até 3 tokens: T-1, T-2, T-3)
    if pos.is_ambiguous() && !anteriores.is_empty() {
        let (pos_resolvida, regra_opt) =
            desambiguar_lookback_ampliado(&base_verbal, pos, anteriores);
        if let Some(r) = regra_opt {
            pos = pos_resolvida;
            regra = Some(r);
        }
    }

    TokenRefinado {
        texto: (*t0).to_string(),
        base: base_verbal,
        tem_enclise: false,
        pos,
        regra,
    }
}

/// Executa a retroanálise contextual inspecionando tokens em T-1, T-2 e T-3.
fn desambiguar_lookback_ampliado(
    base: &str,
    mascara_t0: PosMask,
    anteriores: &[&str],
) -> (PosMask, Option<String>) {
    let t_minus_1 = anteriores.last().unwrap().to_lowercase();
    let t_minus_2 = if anteriores.len() >= 2 {
        Some(anteriores[anteriores.len() - 2].to_lowercase())
    } else {
        None
    };
    let t_minus_3 = if anteriores.len() >= 3 {
        Some(anteriores[anteriores.len() - 3].to_lowercase())
    } else {
        None
    };

    // R-PREP-EXTENDED: Preposição que rege infinitivo (direta ou com clítico/negação intercalada)
    if is_infinitivo(base) && mascara_t0.contains(PosMask::VERBO) {
        // T-1 direto: "sem doer", "para amar", "de viver"
        if is_preposicao_infinitiva(&t_minus_1) {
            return (PosMask::VERBO, Some("R-PREP".to_string()));
        }
        // T-2 prep + T-1 clítico ou negação: "sem se ver", "de não saber", "para me amar"
        if let Some(ref t2) = t_minus_2 {
            if is_preposicao_infinitiva(t2) && (is_clitico(&t_minus_1) || is_negacao(&t_minus_1)) {
                return (PosMask::VERBO, Some("R-PREP-CLITIC".to_string()));
            }
        }
    }

    // R-PRON-EXTENDED: Predicação pessoal ou sujeito expresso
    if mascara_t0.contains(PosMask::VERBO) {
        if is_pronome_reto(&t_minus_1) || is_clitico(&t_minus_1) {
            return (PosMask::VERBO, Some("R-PRON".to_string()));
        }
        if let Some(ref t2) = t_minus_2 {
            if is_pronome_reto(t2) && (is_clitico(&t_minus_1) || is_negacao(&t_minus_1)) {
                return (PosMask::VERBO, Some("R-PRON-COMPLEX".to_string()));
            }
        }
    }

    // R-NEG: Negação verbal direta
    if is_negacao(&t_minus_1) && mascara_t0.contains(PosMask::VERBO) {
        return (PosMask::VERBO, Some("R-NEG".to_string()));
    }

    // R-ADV-MOD: Modificação por advérbio de intensidade ("tão frio", "mais puro", "bem triste")
    if is_adverbio_intensidade(&t_minus_1) && mascara_t0.contains(PosMask::ADJETIVO) {
        return (PosMask::ADJETIVO, Some("R-ADV-MOD".to_string()));
    }

    // R-DET-EXTENDED: Substantivação direta e mediata
    if mascara_t0.contains(PosMask::SUBSTANTIVO) {
        // T-1 direto: determinante imediato
        if is_determinante(&t_minus_1) {
            return (PosMask::SUBSTANTIVO, Some("R-DET".to_string()));
        }

        // T-2 determinante + T-1 adjetivo ou modificador nominal ("o meu canto", "um doce canto", "este belo manto")
        if let Some(ref t2) = t_minus_2 {
            if is_determinante(t2) && (is_modificador_nominal(&t_minus_1) || is_possessivo(t2)) {
                return (PosMask::SUBSTANTIVO, Some("R-DET-MOD".to_string()));
            }
        }

        // T-3 determinante + T-2 intensificador/possessivo + T-1 adjetivo ("o meu doce canto", "um tão suave pranto")
        if let Some(ref t3) = t_minus_3 {
            if is_determinante(t3) {
                if let Some(ref t2) = t_minus_2 {
                    if (is_adverbio_intensidade(t2) || is_possessivo(t2))
                        && is_modificador_nominal(&t_minus_1)
                    {
                        return (PosMask::SUBSTANTIVO, Some("R-DET-MULTI".to_string()));
                    }
                }
            }
        }
    }

    // R-MOD: Substantivo inequívoco precedente tornando T0 adjetivo
    if let Some(m1) = consultar_lexico(&t_minus_1) {
        if m1 == PosMask::SUBSTANTIVO && mascara_t0.contains(PosMask::ADJETIVO) {
            return (PosMask::ADJETIVO, Some("R-MOD".to_string()));
        }
    }

    (mascara_t0, None)
}

/// Resultado da avaliação qualitativa conservadora de um par de rimas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultadoAvaliacaoConservadora {
    /// Classificação qualitativa refinada.
    pub qualidade: QualidadeRima,
    /// Justificativa analítica ou hipótese epistêmica.
    pub detalhe: Option<String>,
    /// Classes morfológicas inferidas para cada termo.
    pub classes: (Vec<String>, Vec<String>),
    /// Explicação pedagógica em prosa clara para humanos.
    pub prosa: String,
}

/// Avalia conservadoramente a relação qualitativa entre dois tokens em rima,
/// resolvendo zonas cinzentas de substantivos e adjetivos com salvaguardas epistêmicas.
pub fn avaliar_par_conservador(
    t1: &TokenRefinado,
    t2: &TokenRefinado,
    qualidade_original: &QualidadeRima,
    detalhe_original: Option<&str>,
) -> ResultadoAvaliacaoConservadora {
    let c1: Vec<String> = t1.pos.nomes().into_iter().map(|s| s.to_string()).collect();
    let c2: Vec<String> = t2.pos.nomes().into_iter().map(|s| s.to_string()).collect();

    let c1_final = if c1.is_empty() {
        vec!["Vocábulo".to_string()]
    } else {
        c1
    };
    let c2_final = if c2.is_empty() {
        vec!["Vocábulo".to_string()]
    } else {
        c2
    };

    let c1_str = c1_final.join(" / ").to_lowercase();
    let c2_str = c2_final.join(" / ").to_lowercase();

    // 1. Se o motor básico já encontrou rima Preciosa ou Rara, preserva a especificidade
    match qualidade_original {
        QualidadeRima::Preciosa => {
            let prosa = format!(
                "Rima preciosa construída com artifício fonético raro ou combinação morfológica singular ({c1_str} × {c2_str})."
            );
            return ResultadoAvaliacaoConservadora {
                qualidade: QualidadeRima::Preciosa,
                detalhe: detalhe_original.map(|s| s.to_string()),
                classes: (c1_final, c2_final),
                prosa,
            };
        }
        QualidadeRima::Rara => {
            let prosa = "Rima rara com terminações de baixa frequência ou difícil emparelhamento no léxico português.".to_string();
            return ResultadoAvaliacaoConservadora {
                qualidade: QualidadeRima::Rara,
                detalhe: detalhe_original.map(|s| s.to_string()),
                classes: (c1_final, c2_final),
                prosa,
            };
        }
        _ => {}
    }

    // 2. Se ambos os tokens forem resolvidos a 100% após o lookback ampliado
    if t1.pos.is_resolved() && t2.pos.is_resolved() {
        if t1.pos == t2.pos {
            let prosa = format!(
                "Rima pobre entre palavras da mesma classe gramatical ({c1_str}), comum e espontânea na tradição lírica."
            );
            return ResultadoAvaliacaoConservadora {
                qualidade: QualidadeRima::Pobre,
                detalhe: None,
                classes: (c1_final, c2_final),
                prosa,
            };
        } else {
            let prosa = format!(
                "Rima rica entre classes gramaticais diferentes ({c1_str} × {c2_str}), agregando variedade e expressividade à estrofe."
            );
            return ResultadoAvaliacaoConservadora {
                qualidade: QualidadeRima::Rica,
                detalhe: None,
                classes: (c1_final, c2_final),
                prosa,
            };
        }
    }

    // 3. Se as classes possíveis forem matematicamente disjuntas (100% Rica)
    if !t1.pos.is_empty() && !t2.pos.is_empty() && (t1.pos & t2.pos).is_empty() {
        let prosa = format!(
            "Rima rica entre classes gramaticais diferentes ({c1_str} × {c2_str}), agregando variedade e expressividade à estrofe."
        );
        return ResultadoAvaliacaoConservadora {
            qualidade: QualidadeRima::Rica,
            detalhe: None,
            classes: (c1_final, c2_final),
            prosa,
        };
    }

    // 4. Verbo Categórico vs Esfera Nominal
    let t1_eh_verbo_estrito = t1.pos == PosMask::VERBO;
    let t2_eh_verbo_estrito = t2.pos == PosMask::VERBO;
    let t1_esfera_nominal =
        t1.pos.contains(PosMask::SUBSTANTIVO) || t1.pos.contains(PosMask::ADJETIVO);
    let t2_esfera_nominal =
        t2.pos.contains(PosMask::SUBSTANTIVO) || t2.pos.contains(PosMask::ADJETIVO);

    if (t1_eh_verbo_estrito && t2_esfera_nominal) || (t2_eh_verbo_estrito && t1_esfera_nominal) {
        let motivo = "Verbo categórico em confronto com termo de esfera nominal".to_string();
        let prosa = format!(
            "Rima com provável riqueza: as palavras tendem a pertencer a classes diferentes ({c1_str} × {c2_str}) no contexto poético."
        );
        return ResultadoAvaliacaoConservadora {
            qualidade: QualidadeRima::ProvavelRica {
                justificativa: motivo.clone(),
            },
            detalhe: Some(motivo),
            classes: (c1_final, c2_final),
            prosa,
        };
    }

    // 5. Regra de Convergência Nominal Conservadora (Zona Cinzenta Substantivo / Adjetivo)
    // Se ambos os vocábulos operam na esfera nominal (substantivo / adjetivo) sem evidência verbal
    if t1_esfera_nominal && t2_esfera_nominal && !t1_eh_verbo_estrito && !t2_eh_verbo_estrito {
        let motivo = format!(
            "Convergência nominal: '{}' e '{}' operam na esfera substantivo-adjetival na tradição lírica",
            t1.texto, t2.texto
        );
        let prosa = format!(
            "Rima com provável convergência nominal: '{}' e '{}' pertencem à esfera de substantivos e adjetivos ({c1_str}), operando no mesmo campo semântico-categorial no verso.",
            t1.texto, t2.texto
        );
        return ResultadoAvaliacaoConservadora {
            qualidade: QualidadeRima::ProvavelPobre {
                justificativa: motivo.clone(),
            },
            detalhe: Some(motivo),
            classes: (c1_final, c2_final),
            prosa,
        };
    }

    // 6. Se o motor anterior já havia determinado ProvavelPobre ou ProvavelRica e nenhum caso acima aplicou
    match qualidade_original {
        QualidadeRima::ProvavelPobre { justificativa } => {
            let prosa = format!(
                "Rima com provável convergência: as palavras tendem a atuar na mesma esfera gramatical ({c1_str})."
            );
            ResultadoAvaliacaoConservadora {
                qualidade: QualidadeRima::ProvavelPobre {
                    justificativa: justificativa.clone(),
                },
                detalhe: Some(justificativa.clone()),
                classes: (c1_final, c2_final),
                prosa,
            }
        }
        QualidadeRima::ProvavelRica { justificativa } => {
            let prosa = format!(
                "Rima com provável riqueza: as palavras tendem a pertencer a classes diferentes ({c1_str} × {c2_str}) no contexto poético."
            );
            ResultadoAvaliacaoConservadora {
                qualidade: QualidadeRima::ProvavelRica {
                    justificativa: justificativa.clone(),
                },
                detalhe: Some(justificativa.clone()),
                classes: (c1_final, c2_final),
                prosa,
            }
        }
        // 7. Salvaguarda Epistêmica: Manter Indeterminada sem suposições arbitrárias
        _ => {
            let prosa = format!(
                "Rima com flexibilidade sintática: a função exata de '{}' ou '{}' varia conforme a interpretação da oração.",
                t1.texto, t2.texto
            );
            ResultadoAvaliacaoConservadora {
                qualidade: qualidade_original.clone(),
                detalhe: detalhe_original.map(|s| s.to_string()),
                classes: (c1_final, c2_final),
                prosa,
            }
        }
    }
}

// Funções auxiliares determinísticas

fn is_determinante(t: &str) -> bool {
    matches!(
        t,
        "o" | "a"
            | "os"
            | "as"
            | "um"
            | "uma"
            | "uns"
            | "umas"
            | "do"
            | "da"
            | "dos"
            | "das"
            | "no"
            | "na"
            | "nos"
            | "nas"
            | "pelo"
            | "pela"
            | "pelos"
            | "pelas"
            | "num"
            | "numa"
            | "nuns"
            | "numas"
            | "dum"
            | "duma"
            | "duns"
            | "dumas"
            | "ao"
            | "aos"
            | "à"
            | "às"
            | "meu"
            | "minha"
            | "meus"
            | "minhas"
            | "teu"
            | "tua"
            | "teus"
            | "tuas"
            | "seu"
            | "sua"
            | "seus"
            | "suas"
            | "nosso"
            | "nossa"
            | "nossos"
            | "nossas"
            | "vosso"
            | "vossa"
            | "vossos"
            | "vossas"
            | "este"
            | "esta"
            | "estes"
            | "estas"
            | "esse"
            | "essa"
            | "esses"
            | "essas"
            | "aquele"
            | "aquela"
            | "aqueles"
            | "aquelas"
            | "deste"
            | "desta"
            | "destes"
            | "destas"
            | "desse"
            | "dessa"
            | "desses"
            | "dessas"
            | "daquele"
            | "daquela"
            | "daqueles"
            | "daquelas"
            | "neste"
            | "nesta"
            | "nestes"
            | "nestas"
            | "nesse"
            | "nessa"
            | "nesses"
            | "nessas"
            | "naquele"
            | "naquela"
            | "naqueles"
            | "naquelas"
    )
}

fn is_possessivo(t: &str) -> bool {
    matches!(
        t,
        "meu"
            | "minha"
            | "meus"
            | "minhas"
            | "teu"
            | "tua"
            | "teus"
            | "tuas"
            | "seu"
            | "sua"
            | "seus"
            | "suas"
            | "nosso"
            | "nossa"
            | "nossos"
            | "nossas"
            | "vosso"
            | "vossa"
            | "vossos"
            | "vossas"
    )
}

fn is_pronome_reto(t: &str) -> bool {
    matches!(
        t,
        "eu" | "tu" | "ele" | "ela" | "nós" | "vós" | "eles" | "elas"
    )
}

fn is_clitico(t: &str) -> bool {
    matches!(t, "me" | "te" | "se" | "lhe" | "nos" | "vos" | "lhes")
}

fn is_negacao(t: &str) -> bool {
    matches!(t, "não" | "nunca" | "jamais")
}

fn is_preposicao_infinitiva(t: &str) -> bool {
    matches!(
        t,
        "para" | "de" | "a" | "sem" | "por" | "com" | "até" | "em"
    )
}

fn is_adverbio_intensidade(t: &str) -> bool {
    matches!(
        t,
        "tão"
            | "muito"
            | "mais"
            | "menos"
            | "bem"
            | "pouco"
            | "quase"
            | "bastante"
            | "demais"
            | "tao"
    )
}

fn is_modificador_nominal(t: &str) -> bool {
    if is_adverbio_intensidade(t) {
        return true;
    }
    if let Some(mask) = consultar_lexico(t) {
        return mask.contains(PosMask::ADJETIVO);
    }
    false
}

fn is_infinitivo(p: &str) -> bool {
    let lower = p.to_lowercase();
    lower.ends_with("ar")
        || lower.ends_with("er")
        || lower.ends_with("ir")
        || lower.ends_with("or")
        || lower.ends_with("ôr")
}

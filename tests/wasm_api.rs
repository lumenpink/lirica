use lirica::{carregar_exemplos, escandir, versao, AnaliseWeb, ExemploPoema};

#[test]
fn test_versao() {
    let v = versao();
    assert!(!v.is_empty());
    assert!(v.starts_with("0.1.0"));
}

#[test]
fn test_carregar_exemplos() {
    let json_str = carregar_exemplos();
    let exemplos: Vec<ExemploPoema> =
        serde_json::from_str(&json_str).expect("Exemplos devem ser JSON válido");
    assert_eq!(exemplos.len(), 7);
    assert_eq!(exemplos[0].id, "camoes");
    assert_eq!(exemplos[1].id, "vinicius");
    assert_eq!(exemplos[2].id, "espinela");
    assert_eq!(exemplos[3].id, "goncalves_dias");
    assert_eq!(exemplos[4].id, "cruz_e_sousa");
    assert_eq!(exemplos[5].id, "pessoa");
    assert_eq!(exemplos[6].id, "bilac");

    // Ensure none of the examples contain %% directives
    for ex in &exemplos {
        assert!(
            !ex.texto.contains("%%"),
            "Exemplo '{}' não deve conter diretivas %%, mas continha",
            ex.id
        );
    }

    // Contrato de integralidade dos poemas de exemplo
    let versos_nao_vazios =
        |texto: &str| -> usize { texto.lines().filter(|l| !l.trim().is_empty()).count() };

    assert!(
        versos_nao_vazios(&exemplos[3].texto) >= 20,
        "Canção do Exílio deve conter o poema completo (24 versos em 5 estrofes), mas continha {}",
        versos_nao_vazios(&exemplos[3].texto)
    );
    assert_eq!(
        versos_nao_vazios(&exemplos[4].texto),
        14,
        "Sinfonias do Ocaso deve ser o soneto completo de Cruz e Sousa (14 versos)"
    );
    assert_eq!(
        versos_nao_vazios(&exemplos[5].texto),
        12,
        "Autopsicografia deve conter as 3 estrofes completas (12 versos)"
    );
    assert_eq!(
        versos_nao_vazios(&exemplos[6].texto),
        14,
        "Ouvir Estrelas deve ser o soneto decassílabo completo de Olavo Bilac (14 versos)"
    );

    // Fortificação anti-mutante: contagem estrófica exata
    let estrofes_count =
        |texto: &str| -> usize { texto.split("\n\n").filter(|s| !s.trim().is_empty()).count() };
    assert_eq!(
        estrofes_count(&exemplos[3].texto),
        5,
        "Canção do Exílio deve ter 5 estrofes"
    );
    assert_eq!(
        estrofes_count(&exemplos[4].texto),
        4,
        "Sinfonias do Ocaso deve ter 4 estrofes"
    );
    assert_eq!(
        estrofes_count(&exemplos[5].texto),
        3,
        "Autopsicografia deve ter 3 estrofes"
    );
    assert_eq!(
        estrofes_count(&exemplos[6].texto),
        4,
        "Ouvir Estrelas deve ter 4 estrofes"
    );
}

#[test]
fn test_camoes_quatro_versos_decassilabos_e_rimas() {
    let texto = "Amor é fogo que arde sem se ver,\nÉ ferida que dói e não se sente;\nÉ um contentamento descontente,\nÉ dor que desatina sem doer.";
    let saida = escandir(texto, "visual", false, false, true, true);
    let web: AnaliseWeb =
        serde_json::from_str(&saida).expect("Modo visual deve retornar JSON de AnaliseWeb");
    assert_eq!(web.versos.len(), 4);
    assert_eq!(web.esquema_rima.as_deref(), Some("ABBA"));

    for (i, v) in web.versos.iter().enumerate() {
        assert_eq!(
            v.total_metricas,
            10,
            "Verso {} ('{}') deve ter 10 sílabas métricas, teve {}",
            i + 1,
            v.original,
            v.total_metricas
        );
        assert!(
            v.classificacao.contains("Decassílabo"),
            "Verso {} deve ser Decassílabo",
            i + 1
        );
        assert_eq!(v.icone, "⚔️");
    }

    // Verify ABBA rhyme scheme
    assert_eq!(web.versos[0].rima.as_deref(), Some("A"));
    assert_eq!(web.versos[1].rima.as_deref(), Some("B"));
    assert_eq!(web.versos[2].rima.as_deref(), Some("B"));
    assert_eq!(web.versos[3].rima.as_deref(), Some("A"));
}

#[test]
fn test_escandir_painel() {
    let texto = "Amor é fogo que arde sem se ver,\nÉ ferida que dói e não se sente;";
    let saida = escandir(texto, "painel", false, false, true, true);
    assert!(saida.contains("10"));
    assert!(saida.contains("Decassílabo"));
}

#[test]
fn test_escandir_typst() {
    let texto = "Amor é fogo que arde sem se ver";
    let saida = escandir(texto, "typst", false, false, true, true);
    assert!(saida.contains("#grid") || saida.contains("#block"));
}

#[test]
fn test_escandir_visual() {
    let texto = "Minha terra tem palmeiras";
    let saida = escandir(texto, "visual", false, false, true, true);
    let web: AnaliseWeb =
        serde_json::from_str(&saida).expect("Modo visual deve retornar JSON de AnaliseWeb");
    assert_eq!(web.versos.len(), 1);
    assert_eq!(web.versos[0].total_metricas, 7);
    assert_eq!(web.versos[0].icone, "🌿");
    assert!(
        web.versos[0].classificacao.contains("Heptassílabo")
            || web.versos[0].classificacao.contains("Redondilha")
    );
}

#[test]
fn test_forcar_hiato_na_interface() {
    // Normal: elide
    let saida1 = escandir("que arde", "visual", false, false, true, true);
    let web1: AnaliseWeb = serde_json::from_str(&saida1).unwrap();
    assert_eq!(web1.versos[0].total_metricas, 1);

    // Com barra: não elide
    let saida2 = escandir("que / arde", "visual", false, false, true, true);
    let web2: AnaliseWeb = serde_json::from_str(&saida2).unwrap();
    assert_eq!(web2.versos[0].total_metricas, 2);
}

#[test]
fn test_forma_poetica_e_estrofes_no_wasm() {
    let exemplos_json = carregar_exemplos();
    let exemplos: Vec<ExemploPoema> = serde_json::from_str(&exemplos_json).unwrap();
    let camoes = &exemplos[0];

    // Com elidir_pontuacao = true (padrão clássico contínuo)
    let saida = escandir(&camoes.texto, "visual", true, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&saida).expect("Deve desserializar AnaliseWeb");
    assert_eq!(web.versos.len(), 14);
    assert_eq!(web.estrofes.len(), 4);

    // Todos os 14 versos de Camões devem ser decassílabos perfeitos
    for (i, v) in web.versos.iter().enumerate() {
        assert_eq!(
            v.total_metricas,
            10,
            "Verso {} de Camões ('{}') deve ter exatamente 10 sílabas, teve {}",
            i + 1,
            v.original,
            v.total_metricas
        );
    }

    let forma = web.forma_poetica.expect("Deve detectar forma poética");
    assert_eq!(forma.nome, "Soneto Italiano (Petrarquiano)");
    assert_eq!(forma.confianca_percentual, 100);

    let rima = web.avaliacao_rima.expect("Deve avaliar rima");
    assert_eq!(rima.pureza_acustica, "Consoante (Perfeita)");
    assert_eq!(rima.percentual_rimado, 100);
}

#[test]
fn test_decima_espinela_no_wasm() {
    let exemplos_json = carregar_exemplos();
    let exemplos: Vec<ExemploPoema> = serde_json::from_str(&exemplos_json).unwrap();
    let espinela = exemplos
        .iter()
        .find(|e| e.id == "espinela")
        .expect("Deve encontrar espinela");

    let saida = escandir(&espinela.texto, "visual", false, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&saida).expect("Deve desserializar AnaliseWeb");
    assert_eq!(web.versos.len(), 10);

    let forma = web.forma_poetica.expect("Deve detectar Décima Espinela");
    assert_eq!(forma.nome, "Décima Espinela");
    assert!(forma.confianca_percentual >= 90);
}

#[test]
fn test_escandir_vazio() {
    let saida = escandir("", "painel", false, false, true, true);
    assert!(saida.is_empty());
}

#[test]
fn test_soneto_de_fidelidade_nos_exemplos() {
    let exemplos_json = carregar_exemplos();
    let exemplos: Vec<ExemploPoema> = serde_json::from_str(&exemplos_json).unwrap();
    let vinicius = exemplos
        .iter()
        .find(|e| e.id == "vinicius" || e.titulo.contains("Fidelidade"))
        .expect("Deve conter o Soneto de Fidelidade nos exemplos");

    assert_eq!(vinicius.autor, "Vinícius de Moraes");
    let saida = escandir(&vinicius.texto, "visual", true, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&saida).expect("Deve desserializar AnaliseWeb");
    assert_eq!(web.versos.len(), 14);
    assert_eq!(web.estrofes.len(), 4);

    for (i, v) in web.versos.iter().enumerate() {
        assert_eq!(
            v.total_metricas,
            10,
            "Verso {} de Vinícius ('{}') deve ter 10 sílabas, teve {}",
            i + 1,
            v.original,
            v.total_metricas
        );
        assert!(
            v.classificacao.contains("Decassílabo"),
            "Verso {} de Vinícius deve ser Decassílabo",
            i + 1
        );
    }

    let forma = web.forma_poetica.expect("Deve detectar Soneto");
    assert!(forma.nome.contains("Soneto"));
    assert!(forma.confianca_percentual >= 80);

    let rima = web
        .avaliacao_rima
        .expect("Deve avaliar rima do Soneto de Fidelidade");
    assert!(rima.percentual_rimado >= 85);
}

#[test]
fn test_escansao_sem_diretivas_respeita_flag_rima() {
    let texto = "Minha terra tem palmeiras,\nOnde canta o Sabiá;\nAs aves que aqui gorjeiam,\nNão gorjeiam como lá.";
    // Flag de rima = true
    let saida_com_rima = escandir(texto, "visual", false, false, true, true);
    let web_com: AnaliseWeb = serde_json::from_str(&saida_com_rima).unwrap();
    assert!(
        web_com.esquema_rima.is_some(),
        "Deve gerar esquema de rimas quando flag é true"
    );

    // Flag de rima = false
    let saida_sem_rima = escandir(texto, "visual", false, false, true, false);
    let web_sem: AnaliseWeb = serde_json::from_str(&saida_sem_rima).unwrap();
    assert!(
        web_sem.esquema_rima.is_none(),
        "Não deve gerar esquema de rimas quando flag é false"
    );
}

#[test]
fn test_soneto_shakespeariano_profana_100_por_cento_wasm() {
    let texto = "\
Cobiça a carne, que depois recusa,\n\
Exige o molde, pronta a condenar;\n\
Disfarça a culpa no pudor da musa\n\
Enquanto o vício prende o seu olhar.\n\
\n\
Despreza a mão que levantou seu muro,\n\
No lindo linho limpa o nojo e ri;\n\
Consome o braço no trabalho duro,\n\
Negando a quem mantém seu trono ali.\n\
\n\
Condena o gozo na oração que tece,\n\
Reduz a vida a tráfico venal;\n\
Compra o silêncio na vendida prece,\n\
E julga o mundo em cego tribunal.\n\
\n\
Se o teu altar é palco de pecado,\n\
Sangue, suor ou sêmen é sagrado?";

    let saida = escandir(texto, "visual", false, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&saida).expect("JSON visual");
    let forma = web.forma_poetica.expect("Deve detectar forma");

    assert_eq!(forma.nome, "Soneto Inglês (Shakespeariano)");
    assert_eq!(
        forma.confianca_percentual, 100,
        "Profana deve atingir 100% no WASM"
    );
    assert!(
        forma.observacoes.is_empty(),
        "Nenhum alerta deve ser gerado para Profana"
    );
}

#[test]
fn test_soneto_shakespeariano_contraprova_diagnostico_wasm() {
    let texto = "\
Cobiça a carne, que depois recusa,\n\
Exige o molde, pronta a condenar;\n\
Disfarça a culpa no pudor da musa\n\
Enquanto o vício prende o seu olhar.\n\
\n\
Despreza a mão que levantou seu muro,\n\
No lindo linho limpa o nojo e ri;\n\
Consome o braço no trabalho duro,\n\
Negando a quem mantém seu trono ali.\n\
\n\
Condena o gozo na oração que tece,\n\
Reduz a vida a tráfico venal;\n\
Compra o silêncio na vendida prece,\n\
E julga o mundo em cego tribunal.\n\
\n\
Se o teu altar é palco de pecado,\n\
Todo este pranto agora é vão lamento.";

    let saida = escandir(texto, "visual", false, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&saida).expect("JSON visual");
    let forma = web.forma_poetica.expect("Deve detectar forma");

    assert_eq!(forma.nome, "Soneto Inglês (Shakespeariano)");
    assert!(
        forma.confianca_percentual < 100,
        "Dístico desemparelhado deve pontuar abaixo de 100%"
    );
    assert!(
        forma
            .observacoes
            .iter()
            .any(|o| o.contains("dístico final")),
        "Deve conter observação diagnóstica sobre o desvio de rima no dístico"
    );
}

#[test]
fn test_sonetilha_wasm() {
    let texto = "\
Na margem do rio manso,\n\
Vejo a tarde desmaiar;\n\
Em vão busco o meu descanso,\n\
Cansado de navegar.\n\
\n\
O vento frio me avisa\n\
Que a noite vem ao redor;\n\
Num leve sopro de brisa,\n\
Cresce no peito esta dor.\n\
\n\
As sombras cobrem a terra,\n\
O céu perde a claridade;\n\
A escuridão tudo encerra.\n\
\n\
Fica no peito a saudade,\n\
Toda a dor que a alma aferra,\n\
Chora na calma cidade.";

    let saida = escandir(texto, "visual", false, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&saida).expect("JSON visual");
    let forma = web.forma_poetica.expect("Deve detectar Sonetilha");

    assert_eq!(forma.nome, "Sonetilha (Sonetino)");
    assert_eq!(forma.confianca_percentual, 100);
}

#[test]
fn test_septilha_cordel_wasm() {
    let texto = "\
O verso de sete linhas\n\
É bonito e compassado;\n\
Quem lê com muita atenção\n\
Fica o leitor encantado;\n\
Tem beleza e perfeição,\n\
Tem nobreza no sertão,\n\
E o verso é bem respeitado.";

    let saida = escandir(texto, "visual", false, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&saida).expect("JSON visual");
    let forma = web.forma_poetica.expect("Deve detectar Septilha");

    assert_eq!(forma.nome, "Septilha de Cordel");
    assert_eq!(forma.confianca_percentual, 100);
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct GabaritoPoema {
    id: String,
    autor: String,
    titulo: String,
    forma_esperada: Option<String>,
    texto: String,
    #[serde(default)]
    mutacoes: Vec<GabaritoMutacao>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct GabaritoMutacao {
    id: String,
    tipo: String,
    descricao: String,
    #[serde(default)]
    operacoes: Vec<OperacaoLinha>,
    #[serde(default)]
    flags: FlagsMutacao,
    expectativa: ExpectativaMutacao,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct OperacaoLinha {
    op: String,
    linha: Option<usize>,
    texto: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, Default, Clone)]
struct FlagsMutacao {
    contagem_castilho: Option<bool>,
    elidir_pontuacao: Option<bool>,
    forcar_rima_estrita: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
struct ExpectativaMutacao {
    forma_esperada: Option<String>,
    nota_min: Option<u8>,
    nota_max: Option<u8>,
    observacoes_obrigatorias: Option<Vec<String>>,
}

fn aplicar_mutacoes_texto(texto_original: &str, operacoes: &[OperacaoLinha]) -> String {
    let mut linhas: Vec<String> = texto_original.lines().map(|s| s.to_string()).collect();

    for op in operacoes {
        match op.op.as_str() {
            "remover_espacos_estroficos" => {
                linhas.retain(|l| !l.trim().is_empty());
            }
            "substituir" => {
                if let Some(num_linha) = op.linha {
                    let mut contador = 0;
                    for l in linhas.iter_mut() {
                        if !l.trim().is_empty() {
                            contador += 1;
                            if contador == num_linha {
                                if let Some(novo_texto) = &op.texto {
                                    *l = novo_texto.clone();
                                }
                                break;
                            }
                        }
                    }
                }
            }
            "remover" => {
                if let Some(num_linha) = op.linha {
                    let mut contador = 0;
                    let mut indice_remover = None;
                    for (idx, l) in linhas.iter().enumerate() {
                        if !l.trim().is_empty() {
                            contador += 1;
                            if contador == num_linha {
                                indice_remover = Some(idx);
                                break;
                            }
                        }
                    }
                    if let Some(idx) = indice_remover {
                        linhas.remove(idx);
                    }
                }
            }
            "inserir" => {
                if let Some(num_linha) = op.linha {
                    let mut contador = 0;
                    let mut indice_inserir = linhas.len();
                    for (idx, l) in linhas.iter().enumerate() {
                        if !l.trim().is_empty() {
                            contador += 1;
                            if contador == num_linha {
                                indice_inserir = idx;
                                break;
                            }
                        }
                    }
                    if let Some(novo_texto) = &op.texto {
                        linhas.insert(indice_inserir, novo_texto.clone());
                    }
                }
            }
            outro => panic!("Operação de mutação desconhecida: {}", outro),
        }
    }

    linhas.join("\n")
}

#[test]
fn test_site_gabarito_e_mutacoes_wasm() {
    let raw = std::fs::read_to_string("site/gabarito.json")
        .expect("site/gabarito.json deve existir no repositório");
    let poemas: Vec<GabaritoPoema> =
        serde_json::from_str(&raw).expect("site/gabarito.json deve ser JSON válido");

    assert_eq!(
        poemas.len(),
        55,
        "Gabarito canônico deve ter exatamente 55 poemas puros"
    );

    let total_mutacoes: usize = poemas.iter().map(|p| p.mutacoes.len()).sum();
    assert_eq!(
        total_mutacoes, 102,
        "Total de mutações declaradas deve ser 102"
    );

    for poema in &poemas {
        for mutacao in &poema.mutacoes {
            let texto = aplicar_mutacoes_texto(&poema.texto, &mutacao.operacoes);
            let castilho = mutacao.flags.contagem_castilho.unwrap_or(true);
            let elidir_pont = mutacao.flags.elidir_pontuacao.unwrap_or(false);

            let saida = escandir(&texto, "visual", false, elidir_pont, castilho, true);
            let web: AnaliseWeb = serde_json::from_str(&saida).expect("JSON visual");

            let res = &mutacao.expectativa;
            if res.nota_max == Some(0) {
                if let Some(ref f) = web.forma_poetica {
                    assert_eq!(
                        f.confianca_percentual, 0,
                        "Mutação {} em {}: nota esperada 0%, obteve {}% para '{}'",
                        mutacao.id, poema.id, f.confianca_percentual, f.nome
                    );
                }
            } else {
                let diag = web.forma_poetica.as_ref().unwrap_or_else(|| {
                    panic!(
                        "Mutação {} em {}: esperava diagnóstico de '{:?}', mas retornou None",
                        mutacao.id, poema.id, res.forma_esperada
                    )
                });

                if let Some(ref forma_esp) = res.forma_esperada {
                    assert!(
                        diag.nome.contains(forma_esp),
                        "Mutação {} em {}: esperava '{}', detectou '{}'",
                        mutacao.id,
                        poema.id,
                        forma_esp,
                        diag.nome
                    );
                }

                if let Some(nota_min) = res.nota_min {
                    assert!(
                        diag.confianca_percentual >= nota_min,
                        "Mutação {} em {}: nota {}% abaixo da nota mínima esperada {}%",
                        mutacao.id,
                        poema.id,
                        diag.confianca_percentual,
                        nota_min
                    );
                }

                if let Some(nota_max) = res.nota_max {
                    assert!(
                        diag.confianca_percentual <= nota_max,
                        "Mutação {} em {}: nota {}% acima da nota máxima esperada {}%",
                        mutacao.id,
                        poema.id,
                        diag.confianca_percentual,
                        nota_max
                    );
                }

                if let Some(ref observacoes) = res.observacoes_obrigatorias {
                    for obs_esp in observacoes {
                        let achou = diag
                            .observacoes
                            .iter()
                            .any(|obs| obs.to_lowercase().contains(&obs_esp.to_lowercase()));
                        assert!(
                            achou,
                            "Mutação {} em {}: observação obrigatória '{}' não encontrada em {:?}",
                            mutacao.id, poema.id, obs_esp, diag.observacoes
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_analise_web_rimas_morfologicas_e_cliches() {
    let texto = "O meu primeiro e desolado amor,\nNa solidão profunda desta dor,\nLateja aflito o pobre coração,\nAlimentando a doce ilusão.";
    let saida = escandir(texto, "visual", false, false, true, true);
    let web: AnaliseWeb =
        serde_json::from_str(&saida).expect("Modo visual deve retornar JSON de AnaliseWeb");

    let rimas_morf = web
        .rimas_morfologicas
        .expect("AnaliseWeb deve conter rimas_morfologicas");
    assert!(!rimas_morf.is_empty(), "Deve conter ao menos uma estrofe");

    let pares = &rimas_morf[0].pares;
    assert_eq!(pares.len(), 2, "Quarteto AABB deve conter 2 pares de rima");

    // Par 1: amor x dor
    let p1 = &pares[0];
    assert_eq!(p1.tokens.0.to_lowercase(), "amor");
    assert_eq!(p1.tokens.1.to_lowercase(), "dor");
    assert!(p1.eh_cliche, "amor x dor deve ser marcado como clichê");
    assert!(p1.motivo_cliche.is_some());
}

#[test]
fn test_mobile_touch_css_spec() {
    let css = std::fs::read_to_string("site/style.css").expect("site/style.css deve existir");

    // 1. Breakpoint responsivo mobile
    assert!(
        css.contains("@media (max-width: 768px)") || css.contains("@media (max-width: 640px)"),
        "CSS deve conter media query específica para mobile (<= 768px ou <= 640px)"
    );

    // 2. Dashboard em coluna única no mobile
    assert!(
        css.contains(".poetic-analysis-dashboard") && css.contains("grid-template-columns: 1fr"),
        "Dashboard poético deve colapsar para 1 coluna no mobile"
    );

    // 3. Touch targets confortáveis para exclusão de verso
    assert!(
        css.contains(".btn-del-verse")
            && (css.contains("min-width: 44px")
                || css.contains("min-height: 44px")
                || css.contains("width: 44px")
                || css.contains("width: 2.75rem")),
        "Botão de exclusão de verso deve ter alvo de toque acessível (>= 44px)"
    );

    // 4. Input sem auto-zoom no iOS (font-size >= 16px no mobile)
    assert!(
        css.contains(".verse-input")
            && (css.contains("font-size: 1rem") || css.contains("font-size: 16px")),
        "Input de verso no mobile deve ter font-size >= 16px para anular auto-zoom do Safari iOS"
    );
}

#[test]
fn test_localstorage_draft_persistence_spec() {
    let js = std::fs::read_to_string("site/app.js").expect("site/app.js deve existir");

    // 1. Chave canônica de persistência do rascunho
    assert!(
        js.contains("lirica_draft_poem"),
        "app.js deve declarar e utilizar a chave canônica 'lirica_draft_poem'"
    );

    // 2. Debounce para salvamento contínuo durante a digitação
    assert!(
        js.contains("agendarSalvamentoRascunho") || (js.contains("debounce") && js.contains("400")),
        "app.js deve implementar rotina de debounce (400ms) para salvamento do rascunho"
    );

    // 3. Tratamento defensivo try/catch para navegação anônima e quota
    assert!(
        js.contains("localStorage.setItem") && js.contains("catch"),
        "Chamadas a localStorage devem conter tratamento defensivo try/catch"
    );

    // 4. Restauração do rascunho salvo na inicialização
    assert!(
        js.contains("localStorage.getItem")
            && (js.contains("recuperarRascunho") || js.contains("lirica_draft_poem")),
        "app.js deve recuperar o rascunho salvo do localStorage na inicialização"
    );

    // 5. Limpeza do rascunho ao resetar laboratório
    assert!(
        js.contains("localStorage.removeItem") || js.contains("limparRascunho"),
        "app.js deve remover o rascunho persistido ao acionar limparLaboratorio"
    );

    // 6. Prevenção de perda de dados nos eventos de saída/troca de aba
    assert!(
        js.contains("beforeunload") && js.contains("visibilitychange"),
        "app.js deve monitorar beforeunload e visibilitychange para flush imediato do rascunho pendente"
    );

    // 7. Fortificação anti-mutante: debounce com cancelamento e constante de 400ms
    assert!(
        js.contains("STORAGE_DEBOUNCE_MS = 400") || js.contains("400"),
        "Constante de debounce deve ser configurada para 400ms"
    );
    assert!(
        js.contains("clearTimeout"),
        "Rotina de debounce e limpeza deve invocar clearTimeout para anular timers órfãos"
    );

    // 8. Fortificação anti-mutante: proteção contra rascunho em background no mobile
    assert!(
        js.contains("visibilityState === 'hidden'") || js.contains("hidden"),
        "Listener de visibilitychange deve detectar estado hidden para salvar dados no mobile"
    );

    // 9. Precedência estrita: exemplos não devem sobrescrever rascunho recuperado
    assert!(
        js.contains("rascunhoSalvo === null"),
        "Carga de exemplos no bootstrap deve ser condicionada a rascunhoSalvo === null"
    );

    // 10. Sorteio aleatório de exemplos no bootstrap sem rascunho
    assert!(
        js.contains("Math.random()") && js.contains("exemplos.length"),
        "Bootstrap sem rascunho salvo deve sortear aleatoriamente um poema da biblioteca de exemplos"
    );

    // 11. Fortificação anti-mutante: função desacoplada de sorteio com guarda de array
    assert!(
        js.contains("function sortearExemploAleatorio"),
        "app.js deve declarar a função canônica sortearExemploAleatorio"
    );
    assert!(
        !js.contains("verses = defaultEx.texto.split"),
        "Sobrescrita incondicional de Camões deve ser eliminada de app.js"
    );

    // 12. Dirty state: carregarPoema não deve salvar rascunho de forma passiva
    let fn_carregar_poema = js
        .split("function carregarPoema")
        .nth(1)
        .and_then(|s| s.split('}').next())
        .unwrap_or("");
    assert!(
        !fn_carregar_poema.contains("salvarRascunho"),
        "carregarPoema não deve salvar no localStorage para não poluir o rascunho passivamente"
    );

    // 13. Fortificação anti-mutante: controle explícito por flag isDirty
    assert!(
        js.contains("let isDirty = false") || js.contains("isDirty = true"),
        "app.js deve gerenciar a flag de estado dirty (isDirty) para persistência seletiva"
    );
    assert!(
        js.contains("isDirty = false") && fn_carregar_poema.contains("isDirty = false"),
        "carregarPoema deve marcar isDirty = false ao visualizar exemplos em memória"
    );
}

#[test]
fn test_markdown_title_author_spec() {
    let html = std::fs::read_to_string("site/index.html").expect("site/index.html deve existir");
    let js = std::fs::read_to_string("site/app.js").expect("site/app.js deve existir");

    // 1. Inputs visuais de título e autor
    assert!(
        html.contains("id=\"input-titulo\"") && html.contains("id=\"input-autor\""),
        "index.html deve conter os inputs #input-titulo e #input-autor"
    );

    // 2. Manipulação de título e autor em app.js
    assert!(
        js.contains("inputTitulo") && js.contains("inputAutor"),
        "app.js deve declarar referências aos elementos inputTitulo e inputAutor"
    );

    // 3. Suporte a cabeçalhos Markdown (# e ##) no modo texto livre
    assert!(
        js.contains("startsWith('# ')") || js.contains("startsWith('#')"),
        "app.js deve extrair título a partir de cabeçalho Markdown (# )"
    );
    assert!(
        js.contains("startsWith('## ')") || js.contains("startsWith('##')"),
        "app.js deve extrair autor a partir de cabeçalho Markdown (## )"
    );

    // 4. Fortificação anti-mutante: extração modularizada e limpeza no reset
    assert!(
        js.contains("function extrairMetadadosMarkdown"),
        "app.js deve modularizar a extração em extrairMetadadosMarkdown"
    );
    assert!(
        js.contains("inputTitulo.value = ''") && js.contains("inputAutor.value = ''"),
        "limparLaboratorio deve resetar os campos de título e autor"
    );
    assert!(
        js.contains("parts.push(`# ${tit}`)") || (js.contains("# ") && js.contains("getFullText")),
        "getFullText deve exportar título e autor formatados em Markdown"
    );
}

#[test]
fn test_escansao_ignora_cabecalhos_markdown() {
    let poema_com_metadados = "# Amor é Fogo\n## Luís de Camões\n\nAmor é um fogo que arde sem se ver,\nÉ ferida que dói, e não se sente;";
    let json_str = escandir(poema_com_metadados, "visual", true, false, true, true);
    let web: AnaliseWeb = serde_json::from_str(&json_str)
        .expect("Escansão de poema com título e autor Markdown deve gerar JSON válido");

    // Não deve conter versos para o título nem para o autor
    assert_eq!(
        web.versos.len(),
        2,
        "Deve conter exatamente 2 versos poéticos, ignorando os cabeçalhos Markdown"
    );
    assert_eq!(
        web.versos[0].original, "Amor é um fogo que arde sem se ver,",
        "Primeiro verso deve ser o verso lírico real, não o título"
    );
    assert_eq!(
        web.estrofes.len(),
        1,
        "Deve computar exatamente 1 estrofe real, sem estrofe espúria no índice 0"
    );

    // Texto com apenas títulos não deve criar versos artificiais nem entrar em pânico
    let apenas_titulos = escandir(
        "# Apenas Titulo\n## Apenas Autor\n\n",
        "visual",
        true,
        false,
        true,
        true,
    );
    assert!(
        apenas_titulos.is_empty(),
        "Texto composto somente de metadados Markdown deve resultar em retorno vazio"
    );

    // Validação de correspondência exata para estrofe única de Ninguém viva descurado
    let espinela_com_header = "# Ninguém viva descurado\n## Gregório de Matos\n\nNinguém viva descurado,\nDo que lhe pode acontecer,\nQue o que hoje vemos viver,\nDepois de morto é pesado:\nDe um dia para outro o fado\nMudou tanto de feição,\nQue a flor de maior botão\nEm terra se viu murchada,\nPor mais que estivesse guardada\nNo centro do coração.";
    let json_esp = escandir(espinela_com_header, "visual", false, false, true, true);
    let web_esp: AnaliseWeb =
        serde_json::from_str(&json_esp).expect("Deve desserializar espinela com headers Markdown");
    assert_eq!(
        web_esp.versos.len(),
        10,
        "Décima espinela deve ter 10 versos líricos"
    );
    assert_eq!(
        web_esp.estrofes.len(),
        1,
        "Décima espinela deve ter 1 estrofe"
    );
    let morf = web_esp
        .rimas_morfologicas
        .expect("Deve conter rimas morfológicas");
    assert_eq!(morf.len(), 1);
    assert_eq!(
        morf[0].estrofe_indice, 0,
        "A estrofe lírica deve estar no índice 0 para bater com o topo da estrofe visual"
    );
    assert!(
        !morf[0].pares.is_empty(),
        "Estrofe 0 deve conter pares de rima avaliados"
    );

    // Validação de contrato no frontend: app.js não deve alimentar escansão com getFullText()
    let js = std::fs::read_to_string("site/app.js").expect("site/app.js deve existir");
    assert!(
        !js.contains("escandir(fullText"),
        "app.js jamais deve passar fullText (com headers Markdown) para a escansao do motor"
    );
    assert!(
        js.contains("poemText = verses.join('\\n')") && js.contains("escandir(poemText,"),
        "app.js deve passar poemText (derivado de verses.join('\\n')) para escandir"
    );
}

#[test]
fn test_modal_interativo_legenda_spec() {
    let html = std::fs::read_to_string("site/index.html").expect("site/index.html deve existir");
    let js = std::fs::read_to_string("site/app.js").expect("site/app.js deve existir");
    let css = std::fs::read_to_string("site/style.css").expect("site/style.css deve existir");

    // 1. Estrutura semântica do modal de legenda
    assert!(
        html.contains("id=\"modal-legenda\""),
        "index.html deve conter o container #modal-legenda"
    );
    assert!(
        html.contains("id=\"btn-fechar-modal-legenda\""),
        "index.html deve conter o botão de fechar #btn-fechar-modal-legenda"
    );
    assert!(
        html.contains("role=\"dialog\"") && html.contains("aria-modal=\"true\""),
        "Modal de legenda deve respeitar os atributos ARIA de acessibilidade"
    );

    // 2. Cobertura de todos os ícones fundamentais no conteúdo do modal
    let icones_obrigatorios = [
        "⚔️", "🏛️", "🔨", "📜", "🌿", "🍃", "👑", "⚜️", "🎵", "🌊", "🕊️", "🪶", // Metros
        "🔥", "✨", "💎", "✦", "⚡", // Rimas
    ];
    for icone in &icones_obrigatorios {
        assert!(
            html.contains(icone),
            "Modal de legenda em index.html deve conter o ícone '{}'",
            icone
        );
    }

    // 3. Integração de clique e listeners em app.js
    assert!(
        js.contains("abrirModalLegenda") || js.contains("modalLegenda.hidden = false"),
        "app.js deve conter a rotina para abrir o modal de legenda"
    );
    assert!(
        js.contains("btn-fechar-modal-legenda") && js.contains("btn-fechar-modal-legenda-bottom"),
        "app.js deve vincular os botões de fechar do modal de legenda no topo e na base"
    );
    assert!(
        js.contains("fecharModalLegenda") && js.contains("Escape"),
        "app.js deve fechar o modal de legenda via tecla Escape"
    );

    // 4. Badges interativos e acessibilidade ARIA no CSS e JS
    assert!(
        css.contains(".meter-badge") && css.contains("cursor: pointer"),
        "CSS deve definir cursor: pointer para badges interativos"
    );
    assert!(
        js.contains("setAttribute('role', 'button')")
            && js.contains("setAttribute('tabindex', '0')"),
        "Badges de metro e rima devem receber atributos ARIA role='button' e tabindex='0'"
    );
}

#[test]
fn test_legenda_estatica_rodape_spec() {
    let html = std::fs::read_to_string("site/index.html").expect("site/index.html deve existir");

    // Localiza a seção estática do rodapé
    let inicio_legenda = html
        .find("class=\"notation-guide\"")
        .expect("Deve conter .notation-guide");
    let fim_legenda = html[inicio_legenda..]
        .find("</section>")
        .expect("Deve fechar a section da legenda");
    let bloco_legenda = &html[inicio_legenda..inicio_legenda + fim_legenda];

    // 1. Metros que estavam ausentes na legenda estática
    let metros_obrigatorios = [
        ("📜", "Decassílabo"),
        ("🎵", "Octossílabo"),
        ("🌊", "Eneassílabo"),
        ("🕊️", "Hendecassílabo"),
        ("⚜️", "Dodecassílabo"),
    ];
    for (ico, nome) in &metros_obrigatorios {
        assert!(
            bloco_legenda.contains(ico),
            "Legenda estática do rodapé deve conter o ícone '{}' ({})",
            ico,
            nome
        );
    }

    // 2. Linha de avaliação de rimas com todos os ícones
    let rimas_obrigatorias = [
        ("🔥", "Clichê"),
        ("✨", "Rica"),
        ("💎", "Preciosa"),
        ("✦", "Consoante"),
        ("⚡", "Toante"),
    ];
    for (ico, desc) in &rimas_obrigatorias {
        assert!(
            bloco_legenda.contains(ico),
            "Legenda estática do rodapé deve conter o ícone de rima '{}' ({})",
            ico,
            desc
        );
    }

    // 3. Fortificação anti-mutante: botão de atalho e vinculação com modal
    let js = std::fs::read_to_string("site/app.js").expect("site/app.js deve existir");
    assert!(
        html.contains("id=\"btn-abrir-guia-completo\""),
        "index.html deve conter o botão atalho #btn-abrir-guia-completo na legenda"
    );
    assert!(
        js.contains("btn-abrir-guia-completo"),
        "app.js deve vincular o botão de atalho para abrir o modal de legenda"
    );
}


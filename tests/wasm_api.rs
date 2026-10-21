use lirica::{carregar_exemplos, escandir, versao, AnaliseWeb, ExemploPoema};

#[test]
fn test_versao() {
    let v = versao();
    assert!(!v.is_empty());
    assert_eq!(v, "0.1.0-dev.1");
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

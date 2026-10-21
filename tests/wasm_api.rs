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
    assert_eq!(exemplos.len(), 5);
    assert_eq!(exemplos[0].id, "camoes");
    assert_eq!(exemplos[1].id, "goncalves_dias");
    assert_eq!(exemplos[2].id, "cruz_e_sousa");
    assert_eq!(exemplos[3].id, "pessoa");
    assert_eq!(exemplos[4].id, "bilac");

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
fn test_escandir_vazio() {
    let saida = escandir("", "painel", false, false, true, true);
    assert!(saida.is_empty());
}

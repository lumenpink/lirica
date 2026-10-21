use lirica::{carregar_exemplos, escandir, versao, ExemploPoema, VersoFormatado};

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
    let texto = "%% metro = \"decassilabo\"\nAmor é fogo que arde sem se ver";
    let saida = escandir(texto, "typst", false, false, true, true);
    assert!(saida.contains("#grid") || saida.contains("#block"));
}

#[test]
fn test_escandir_visual() {
    let texto = "Minha terra tem palmeiras";
    let saida = escandir(texto, "visual", false, false, true, true);
    let versos: Vec<VersoFormatado> =
        serde_json::from_str(&saida).expect("Modo visual deve retornar JSON de VersoFormatado");
    assert_eq!(versos.len(), 1);
    assert_eq!(versos[0].total_metricas, 7);
    assert!(
        versos[0].classificacao.contains("Heptassílabo")
            || versos[0].classificacao.contains("Redondilha")
    );
}

#[test]
fn test_escandir_vazio() {
    let saida = escandir("", "painel", false, false, true, true);
    assert!(saida.is_empty());
}

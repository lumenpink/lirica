use std::fs;

#[test]
fn test_site_gabarito_sync_count_and_pieces() {
    let json_content =
        fs::read_to_string("site/gabarito.json").expect("site/gabarito.json deve existir");
    let gabarito: serde_json::Value =
        serde_json::from_str(&json_content).expect("site/gabarito.json deve ser um JSON válido");

    let array = gabarito
        .as_array()
        .expect("gabarito deve ser um array JSON");
    assert_eq!(
        array.len(),
        61,
        "site/gabarito.json deve conter exatamente 61 poemas após expansão canônica"
    );

    let ids: Vec<&str> = array
        .iter()
        .filter_map(|p| p.get("id").and_then(|id| id.as_str()))
        .collect();

    assert!(
        ids.contains(&"cordel_pavao_misterioso"),
        "Falta Romance do Pavão Misterioso no site/gabarito.json"
    );
    assert!(
        ids.contains(&"pajada_gaucha_pampa"),
        "Falta Pajada Gaúcha no site/gabarito.json"
    );
    assert!(
        ids.contains(&"galope_beira_mar_calunga"),
        "Falta Galope à Beira-Mar no site/gabarito.json"
    );
    assert!(
        ids.contains(&"augusto_anjos_vencido"),
        "Falta Augusto dos Anjos no site/gabarito.json"
    );
    assert!(
        ids.contains(&"cruz_sousa_carceres"),
        "Falta Cruz e Sousa no site/gabarito.json"
    );
    assert!(
        ids.contains(&"sentinelas_fonotaticos_fronteira"),
        "Falta matriz de sentinelas fonotáticos no site/gabarito.json"
    );

    let mut total_mutacoes = 0;
    for p in array {
        if let Some(mutacoes) = p.get("mutacoes").and_then(|m| m.as_array()) {
            total_mutacoes += mutacoes.len();
        }
    }
    assert!(
        total_mutacoes >= 106,
        "site/gabarito.json deve conter pelo menos 106 mutações declarativas, encontrou {}",
        total_mutacoes
    );
}

#[test]
fn test_joaju_lirica_dependency_version() {
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Cargo.toml deve existir");
    assert!(
        cargo_toml.contains("joaju-lirica = { version = \"1.0.0\""),
        "Cargo.toml deve requerer joaju-lirica 1.0.0 estável"
    );
}

#[test]
fn test_site_gabarito_html_documentation() {
    let html = fs::read_to_string("site/docs/gabarito.html")
        .expect("site/docs/gabarito.html deve existir");
    assert!(
        html.contains("61 obras-primas"),
        "site/docs/gabarito.html deve citar 61 obras-primas"
    );
    assert!(
        html.contains("106 mutações declarativas"),
        "site/docs/gabarito.html deve citar 106 mutações declarativas"
    );
    assert!(
        !html.contains("51 poemas"),
        "site/docs/gabarito.html não deve conter resquícios de 51 poemas"
    );
}

#[test]
fn test_mutacao_e_limites_novas_pecas_regionais_e_fonotaticas() {
    use lirica::escandir;

    let json_content =
        fs::read_to_string("site/gabarito.json").expect("site/gabarito.json deve existir");
    let gabarito: Vec<serde_json::Value> =
        serde_json::from_str(&json_content).expect("site/gabarito.json deve ser um JSON válido");

    let pavao = gabarito
        .iter()
        .find(|p| p["id"] == "cordel_pavao_misterioso")
        .expect("Pavão Misterioso deve estar no gabarito");
    let pavao_texto = pavao["texto"].as_str().expect("texto do cordel");
    let pavao_saida = escandir(pavao_texto, "visual", false, false, true, true);
    let pavao_web: serde_json::Value = serde_json::from_str(&pavao_saida).unwrap();
    let forma = &pavao_web["forma_poetica"];
    assert_eq!(
        forma["confianca_percentual"], 100,
        "Pavão Misterioso puro deve atingir 100% de confiança"
    );
    assert_eq!(
        forma["nome"], "Sextilha de Cordel",
        "Forma reconhecida deve ser Sextilha de Cordel"
    );

    let pajada = gabarito
        .iter()
        .find(|p| p["id"] == "pajada_gaucha_pampa")
        .expect("Pajada Gaúcha deve estar no gabarito");
    let pajada_texto = pajada["texto"].as_str().expect("texto da pajada");
    let pajada_saida = escandir(pajada_texto, "visual", false, false, true, true);
    let pajada_web: serde_json::Value = serde_json::from_str(&pajada_saida).unwrap();
    let pajada_forma = &pajada_web["forma_poetica"];
    assert!(
        pajada_forma["confianca_percentual"].as_u64().unwrap() >= 95,
        "Pajada Gaúcha pura deve atingir pelo menos 95% de confiança"
    );
    assert_eq!(
        pajada_forma["nome"], "Pajada Gaúcha (Décima Campeira)",
        "Forma reconhecida deve ser Pajada Gaúcha (Décima Campeira)"
    );
}

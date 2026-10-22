import init, { escandir, carregar_exemplos, versao } from './pkg/lirica.js?v=0.1.8';

let exemplos = [];
let verses = [
  "Amor é fogo que arde sem se ver,",
  "É ferida que dói e não se sente;",
  "É um contentamento descontente,",
  "É dor que desatina sem doer.",
  "",
  "É um não querer mais que bem querer;",
  "É um andar solitário entre a gente;",
  "É nunca contentar-se de contente;",
  "É um cuidar que ganha em se perder.",
  "",
  "É querer estar preso por vontade;",
  "É servir a quem vence, o vencedor;",
  "É ter com quem nos mata, lealdade.",
  "",
  "Mas como causar pode seu favor",
  "Nos corações humanos amizade,",
  "Se tão contrário a si é o mesmo Amor?"
];

// Elements
const btnAbrirExemplos = document.getElementById('btn-abrir-exemplos');
const btnHeaderLimpar = document.getElementById('btn-header-limpar');
const modalExemplos = document.getElementById('modal-exemplos');
const btnFecharModal = document.getElementById('btn-fechar-modal');
const btnFecharModalBottom = document.getElementById('btn-fechar-modal-bottom');
const inputBuscaExemplos = document.getElementById('input-busca-exemplos');
const gridExemplos = document.getElementById('grid-exemplos');

const selectModo = document.getElementById('select-modo');
const chkCastilho = document.getElementById('chk-castilho');
const chkPontuacao = document.getElementById('chk-pontuacao');
const chkDierese = document.getElementById('chk-dierese');
const chkRima = document.getElementById('chk-rima');

const counterVersos = document.getElementById('counter-versos');
const counterEstrofes = document.getElementById('counter-estrofes');
const schemeBadge = document.getElementById('scheme-badge');

const btnToggleRaw = document.getElementById('btn-toggle-raw');
const btnAddVerso = document.getElementById('btn-add-verso');
const btnAddEstrofe = document.getElementById('btn-add-estrofe');
const btnLimpar = document.getElementById('btn-limpar');
const btnCopiar = document.getElementById('btn-copiar');
const lblVersao = document.getElementById('lbl-versao');

// Top Dashboard Elements
const formIcon = document.getElementById('form-icon');
const formName = document.getElementById('form-name');
const formScoreBadge = document.getElementById('form-score-badge');
const formDesc = document.getElementById('form-desc');
const formStructureTag = document.getElementById('form-structure-tag');
const formPatternTag = document.getElementById('form-pattern-tag');
const formAlerts = document.getElementById('form-alerts');

const rhymeSchemeTitle = document.getElementById('rhyme-scheme-title');
const rhymePurityBadge = document.getElementById('rhyme-purity-badge');
const rhymeDispositionDesc = document.getElementById('rhyme-disposition-desc');
const rhymePercentTag = document.getElementById('rhyme-percent-tag');

const rawEditorBox = document.getElementById('raw-editor-box');
const rawTextarea = document.getElementById('raw-textarea');
const btnApplyRaw = document.getElementById('btn-apply-raw');

const alignedWorkspace = document.getElementById('aligned-workspace');
const versesContainer = document.getElementById('verses-container');

const codeWorkspace = document.getElementById('code-workspace');
const codeContent = document.getElementById('code-content');

// Modal de Exemplos
function abrirModal() {
  if (!modalExemplos) return;
  modalExemplos.hidden = false;
  if (inputBuscaExemplos) {
    inputBuscaExemplos.value = '';
    renderExemplosGrid('');
    inputBuscaExemplos.focus();
  }
}

function fecharModal() {
  if (!modalExemplos) return;
  modalExemplos.hidden = true;
}

function renderExemplosGrid(filtro = '') {
  if (!gridExemplos) return;
  gridExemplos.innerHTML = '';

  const filtrados = exemplos.filter(ex => {
    if (!filtro) return true;
    return ex.titulo.toLowerCase().includes(filtro) ||
           ex.autor.toLowerCase().includes(filtro) ||
           (ex.genero && ex.genero.toLowerCase().includes(filtro)) ||
           (ex.texto && ex.texto.toLowerCase().includes(filtro));
  });

  if (filtrados.length === 0) {
    gridExemplos.innerHTML = `<div style="grid-column: 1 / -1; text-align: center; color: var(--text-muted); padding: 2.5rem 1rem;">Nenhum exemplo poético encontrado para "<strong>${filtro}</strong>".</div>`;
    return;
  }

  filtrados.forEach(ex => {
    const card = document.createElement('button');
    card.type = 'button';
    card.className = 'exemplo-card';
    card.setAttribute('aria-label', `Carregar ${ex.titulo} de ${ex.autor}`);

    const header = document.createElement('div');
    header.className = 'exemplo-card-header';

    const title = document.createElement('div');
    title.className = 'exemplo-card-title';
    title.textContent = ex.titulo;
    header.appendChild(title);

    if (ex.genero) {
      const badge = document.createElement('span');
      badge.className = 'exemplo-card-badge';
      badge.textContent = ex.genero;
      header.appendChild(badge);
    }

    const autor = document.createElement('div');
    autor.className = 'exemplo-card-autor';
    autor.textContent = ex.autor;

    const preview = document.createElement('div');
    preview.className = 'exemplo-card-preview';
    const lines = ex.texto.split('\n').filter(l => l.trim().length > 0).slice(0, 3);
    preview.textContent = lines.join('\n');

    card.appendChild(header);
    card.appendChild(autor);
    card.appendChild(preview);

    card.addEventListener('click', () => {
      carregarPoema(ex.texto);
      fecharModal();
    });

    gridExemplos.appendChild(card);
  });
}

function carregarPoema(texto) {
  verses = texto.split('\n');
  if (rawTextarea) rawTextarea.value = texto;
  renderWorkspace();
  focusVerse(0);
}

function limparLaboratorio() {
  verses = [''];
  if (rawTextarea) rawTextarea.value = '';
  renderWorkspace();
  focusVerse(0);
}

async function main() {
  await init('./pkg/lirica_bg.wasm?v=0.1.8');

  // Version canônica
  try {
    const v = versao();
    if (lblVersao) {
      lblVersao.textContent = v.startsWith('v') ? v : `v${v}`;
    }
  } catch (e) {
    console.error('Erro de versão:', e);
  }

  // Load Examples
  try {
    exemplos = JSON.parse(carregar_exemplos());
    renderExemplosGrid('');

    // Default Camões se nenhum poema for fornecido via sessionStorage
    const defaultEx = exemplos.find(ex => ex.id === 'camoes');
    if (defaultEx) {
      verses = defaultEx.texto.split('\n');
    }
  } catch (e) {
    console.error('Erro ao carregar exemplos:', e);
  }

  // Verificar se há poema customizado via sessionStorage (ex.: vindo de docs/gabarito.html)
  try {
    const customPoemJson = sessionStorage.getItem('lirica_custom_poem');
    if (customPoemJson) {
      sessionStorage.removeItem('lirica_custom_poem');
      const customPoem = JSON.parse(customPoemJson);
      if (customPoem && customPoem.texto) {
        verses = customPoem.texto.split('\n');
      }
    }
  } catch (e) {
    console.warn('Erro ao carregar poema de sessionStorage:', e);
  }

  // Event Listeners para Modal
  if (btnAbrirExemplos) btnAbrirExemplos.addEventListener('click', abrirModal);
  if (btnFecharModal) btnFecharModal.addEventListener('click', fecharModal);
  if (btnFecharModalBottom) btnFecharModalBottom.addEventListener('click', fecharModal);
  if (modalExemplos) {
    modalExemplos.addEventListener('click', (e) => {
      if (e.target === modalExemplos) fecharModal();
    });
  }
  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && modalExemplos && !modalExemplos.hidden) {
      fecharModal();
    }
  });
  if (inputBuscaExemplos) {
    inputBuscaExemplos.addEventListener('input', (e) => {
      renderExemplosGrid(e.target.value.toLowerCase().trim());
    });
  }

  // Botões de Limpeza
  if (btnHeaderLimpar) btnHeaderLimpar.addEventListener('click', limparLaboratorio);
  if (btnLimpar) btnLimpar.addEventListener('click', limparLaboratorio);

  selectModo.addEventListener('change', renderWorkspace);
  chkCastilho.addEventListener('change', renderWorkspace);
  chkPontuacao.addEventListener('change', renderWorkspace);
  chkDierese.addEventListener('change', renderWorkspace);
  chkRima.addEventListener('change', renderWorkspace);

  btnAddVerso.addEventListener('click', () => {
    verses.push('');
    renderWorkspace();
    focusVerse(verses.length - 1);
  });

  if (btnAddEstrofe) {
    btnAddEstrofe.addEventListener('click', () => {
      if (verses.length > 0 && verses[verses.length - 1] !== '') {
        verses.push('');
      }
      verses.push('');
      renderWorkspace();
      focusVerse(verses.length - 1);
    });
  }

  btnToggleRaw.addEventListener('click', toggleRawEditor);

  btnApplyRaw.addEventListener('click', () => {
    const rawVal = rawTextarea.value;
    const lines = rawVal.split('\n').filter(l => !l.trim().startsWith('%%'));
    verses = lines.length > 0 ? lines : [''];
    rawEditorBox.style.display = 'none';
    btnToggleRaw.textContent = 'Modo Texto Livre';
    renderWorkspace();
  });

  btnCopiar.addEventListener('click', copiarAnalise);

  renderWorkspace();
}

function toggleRawEditor() {
  if (rawEditorBox.style.display === 'none') {
    rawTextarea.value = verses.join('\n');
    rawEditorBox.style.display = 'flex';
    btnToggleRaw.textContent = 'Fechar Texto Livre';
    rawTextarea.focus();
  } else {
    rawEditorBox.style.display = 'none';
    btnToggleRaw.textContent = 'Modo Texto Livre';
  }
}

function getFullText() {
  return verses.join('\n');
}

function updateStats(webData) {
  const nonEmptyVerses = verses.filter(v => v.trim().length > 0);
  const count = nonEmptyVerses.length;
  counterVersos.textContent = `${count} ${count === 1 ? 'verso' : 'versos'}`;

  // Stanzas count
  if (webData && Array.isArray(webData.estrofes) && webData.estrofes.length > 1) {
    if (counterEstrofes) {
      counterEstrofes.style.display = 'inline-block';
      counterEstrofes.textContent = `${webData.estrofes.length} estrofes`;
    }
  } else if (counterEstrofes) {
    counterEstrofes.style.display = 'none';
  }

  // Rhyme Badge in toolbar
  const esquema = webData ? webData.esquema_rima : null;
  if (chkRima.checked && esquema) {
    schemeBadge.textContent = `Rima: ${esquema}`;
    schemeBadge.style.display = 'inline-block';
  } else {
    schemeBadge.style.display = 'none';
  }

  // Top Dashboard: Poetic Form Card
  if (webData && webData.forma_poetica) {
    const forma = webData.forma_poetica;
    formName.textContent = forma.nome;
    formScoreBadge.textContent = `${forma.confianca_percentual}%`;
    formScoreBadge.className = 'score-badge ' + (forma.confianca_percentual >= 85 ? 'high' : forma.confianca_percentual >= 70 ? 'medium' : 'low');
    formDesc.textContent = forma.descricao;
    formStructureTag.textContent = `Estrutura: ${forma.estrutura_detectada}`;
    formPatternTag.textContent = `Padrão: ${forma.padrao_esperado}`;

    if (forma.nome.includes('Soneto')) {
      formIcon.textContent = '👑';
    } else if (forma.nome.includes('Espinela')) {
      formIcon.textContent = '📜';
    } else if (forma.nome.includes('Martelo')) {
      formIcon.textContent = '🔨';
    } else if (forma.nome.includes('Cordel') || forma.nome.includes('Sextilha')) {
      formIcon.textContent = '🌿';
    } else if (forma.nome.includes('Haicai')) {
      formIcon.textContent = '🌸';
    } else {
      formIcon.textContent = '🏆';
    }

    if (forma.observacoes && forma.observacoes.length > 0) {
      formAlerts.innerHTML = forma.observacoes.map(o => `<div>⚠️ ${o}</div>`).join('');
      formAlerts.style.display = 'flex';
    } else {
      formAlerts.style.display = 'none';
    }
  } else {
    formName.textContent = 'Forma Livre';
    formScoreBadge.textContent = '100%';
    formScoreBadge.className = 'score-badge high';
    formDesc.textContent = 'Composição sem forma fixa estrita catalogada.';
    formStructureTag.textContent = `Estrutura: ${count} versos`;
    formPatternTag.textContent = 'Ritmo e métrica livres';
    formAlerts.style.display = 'none';
    formIcon.textContent = '🪶';
  }

  // Top Dashboard: Rhyme Card
  if (webData && webData.avaliacao_rima && chkRima.checked) {
    const ar = webData.avaliacao_rima;
    rhymeSchemeTitle.textContent = ar.esquema ? `Rima: ${ar.esquema}` : 'Sem Rimas';
    rhymePurityBadge.textContent = ar.pureza_acustica;
    rhymeDispositionDesc.textContent = ar.tipo_disposicao;
    rhymePercentTag.textContent = `${ar.percentual_rimado}% Versos Rimados`;
  } else {
    rhymeSchemeTitle.textContent = 'Rima: --';
    rhymePurityBadge.textContent = 'Desativada';
    rhymeDispositionDesc.textContent = 'Análise de rimas desativada ou sem versos.';
    rhymePercentTag.textContent = '0% Versos Rimados';
  }
}

function renderWorkspace() {
  const modo = selectModo.value;
  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  const fullText = getFullText();

  if (modo === 'visual') {
    alignedWorkspace.style.display = 'flex';
    codeWorkspace.style.display = 'none';
    renderAlignedRows(fullText, castilho, pontuacao, dierese, rima);
  } else {
    alignedWorkspace.style.display = 'none';
    codeWorkspace.style.display = 'block';
    const output = escandir(fullText, modo, pontuacao, dierese, castilho, rima);
    codeContent.textContent = output || '// Nenhum texto para analisar.';
    updateStats(null);
  }
}

function renderAlignedRows(fullText, castilho, pontuacao, dierese, rima) {
  let webData = null;
  let parsed = [];
  if (fullText.trim().length > 0) {
    try {
      const jsonStr = escandir(fullText, 'visual', pontuacao, dierese, castilho, rima);
      webData = JSON.parse(jsonStr);
      if (webData && Array.isArray(webData.versos)) {
        parsed = webData.versos;
      } else if (Array.isArray(webData)) {
        parsed = webData;
      }
    } catch (e) {
      console.error('Falha de escansão:', e);
    }
  }

  updateStats(webData);

  versesContainer.innerHTML = '';

  let parsedIdx = 0;
  let lineCounter = 1;
  let stanzaCounter = 1;

  verses.forEach((verseText, idx) => {
    // Quebra de estrofe (linha vazia entre versos)
    if (verseText.trim().length === 0) {
      const divider = document.createElement('div');
      divider.className = 'stanza-divider-row';

      stanzaCounter++;
      const badge = document.createElement('span');
      badge.className = 'stanza-badge';
      badge.innerHTML = `❦ Estrofe ${stanzaCounter}`;

      const btnDel = document.createElement('button');
      btnDel.className = 'btn-del-stanza';
      btnDel.title = 'Remover quebra de estrofe';
      btnDel.innerHTML = '&times;';
      btnDel.addEventListener('click', () => {
        verses.splice(idx, 1);
        renderWorkspace();
      });

      divider.appendChild(badge);
      divider.appendChild(btnDel);
      versesContainer.appendChild(divider);
      return;
    }

    const row = document.createElement('div');
    row.className = 'verse-row';

    // Coluna Esquerda: Verso Editável
    const colLeft = document.createElement('div');
    colLeft.className = 'verse-col-left';

    const numSpan = document.createElement('span');
    numSpan.className = 'line-num';
    numSpan.textContent = `${lineCounter++}`;

    const input = document.createElement('input');
    input.type = 'text';
    input.className = 'verse-input';
    input.value = verseText;
    input.placeholder = `Verso ${idx + 1}...`;
    input.dataset.index = idx;

    input.addEventListener('input', (e) => {
      verses[idx] = e.target.value;
      updateSingleRowLive();
    });

    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        if (e.shiftKey) {
          // Shift+Enter insere quebra de estrofe
          verses.splice(idx + 1, 0, '', '');
          renderWorkspace();
          focusVerse(idx + 2);
        } else {
          verses.splice(idx + 1, 0, '');
          renderWorkspace();
          focusVerse(idx + 1);
        }
      } else if (e.key === 'Backspace' && e.target.value === '' && verses.length > 1) {
        e.preventDefault();
        verses.splice(idx, 1);
        renderWorkspace();
        focusVerse(Math.max(0, idx - 1));
      }
    });

    const btnDel = document.createElement('button');
    btnDel.className = 'btn-del-verse';
    btnDel.title = 'Remover verso';
    btnDel.innerHTML = '&times;';
    btnDel.addEventListener('click', () => {
      if (verses.length > 1) {
        verses.splice(idx, 1);
      } else {
        verses = [''];
      }
      renderWorkspace();
    });

    colLeft.appendChild(numSpan);
    colLeft.appendChild(input);
    colLeft.appendChild(btnDel);

    // Coluna Direita: Escansão Irmã
    const colRight = document.createElement('div');
    colRight.className = 'verse-col-right';

    const vData = parsed[parsedIdx++];

    if (vData) {
      // Compact Rhyme badge
      if (vData.rima) {
        const rhymeBadge = document.createElement('span');
        rhymeBadge.className = 'badge-rima-compact';
        rhymeBadge.textContent = vData.rima;
        const term = vData.rima_terminacao ? ` (som: "${vData.rima_terminacao}")` : '';
        rhymeBadge.title = `Rima: ${vData.rima}${term}`;
        colRight.appendChild(rhymeBadge);
      }

      // Compact Meter badge: count + icon with tooltip
      const meterBadge = document.createElement('span');
      meterBadge.className = 'badge-metro-compact';
      meterBadge.title = `${vData.classificacao} (${vData.total_metricas} sílabas métricas)`;
      meterBadge.innerHTML = `${vData.total_metricas} <span class="ico">${vData.icone || '🪶'}</span>`;
      colRight.appendChild(meterBadge);

      // Syllables strip
      const strip = document.createElement('div');
      strip.className = 'syllables-strip';

      vData.silabas.forEach(s => {
        const syl = document.createElement('span');
        syl.className = 'syl-pill';
        if (s.tonica) syl.classList.add('tonica');
        if (s.elidida) syl.classList.add('elidida');
        if (s.numero_metrico === null) syl.classList.add('extrametrica');

        const idxSpan = document.createElement('span');
        idxSpan.className = 'idx';
        idxSpan.textContent = s.numero_metrico !== null ? s.numero_metrico : '·';

        const txtSpan = document.createElement('span');
        txtSpan.className = 'txt';
        txtSpan.textContent = s.texto;

        syl.appendChild(idxSpan);
        syl.appendChild(txtSpan);
        strip.appendChild(syl);
      });

      colRight.appendChild(strip);
    } else {
      const emptySpan = document.createElement('span');
      emptySpan.style.color = 'var(--text-muted)';
      emptySpan.style.fontSize = '0.75rem';
      emptySpan.style.fontStyle = 'italic';
      emptySpan.textContent = 'Verso vazio';
      colRight.appendChild(emptySpan);
    }

    row.appendChild(colLeft);
    row.appendChild(colRight);
    versesContainer.appendChild(row);
  });
}

function updateSingleRowLive() {
  const fullText = getFullText();
  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  let webData = null;
  let parsed = [];
  if (fullText.trim().length > 0) {
    try {
      const jsonStr = escandir(fullText, 'visual', pontuacao, dierese, castilho, rima);
      webData = JSON.parse(jsonStr);
      if (webData && Array.isArray(webData.versos)) {
        parsed = webData.versos;
      } else if (Array.isArray(webData)) {
        parsed = webData;
      }
    } catch (e) {
      console.error(e);
    }
  }

  updateStats(webData);

  const rows = versesContainer.querySelectorAll('.verse-row');
  let parsedIdx = 0;

  let currentVerseRowIdx = 0;
  verses.forEach((verseText) => {
    if (verseText.trim().length === 0) {
      return; // stanza divider row
    }

    const row = rows[currentVerseRowIdx++];
    if (!row) return;

    const colRight = row.querySelector('.verse-col-right');
    if (!colRight) return;

    colRight.innerHTML = '';
    const vData = parsed[parsedIdx++];

    if (vData) {
      // Compact Rhyme badge
      if (vData.rima) {
        const rhymeBadge = document.createElement('span');
        rhymeBadge.className = 'badge-rima-compact';
        rhymeBadge.textContent = vData.rima;
        const term = vData.rima_terminacao ? ` (som: "${vData.rima_terminacao}")` : '';
        rhymeBadge.title = `Rima: ${vData.rima}${term}`;
        colRight.appendChild(rhymeBadge);
      }

      // Compact Meter badge: count + icon with tooltip
      const meterBadge = document.createElement('span');
      meterBadge.className = 'badge-metro-compact';
      meterBadge.title = `${vData.classificacao} (${vData.total_metricas} sílabas métricas)`;
      meterBadge.innerHTML = `${vData.total_metricas} <span class="ico">${vData.icone || '🪶'}</span>`;
      colRight.appendChild(meterBadge);

      const strip = document.createElement('div');
      strip.className = 'syllables-strip';

      vData.silabas.forEach(s => {
        const syl = document.createElement('span');
        syl.className = 'syl-pill';
        if (s.tonica) syl.classList.add('tonica');
        if (s.elidida) syl.classList.add('elidida');
        if (s.numero_metrico === null) syl.classList.add('extrametrica');

        const idxSpan = document.createElement('span');
        idxSpan.className = 'idx';
        idxSpan.textContent = s.numero_metrico !== null ? s.numero_metrico : '·';

        const txtSpan = document.createElement('span');
        txtSpan.className = 'txt';
        txtSpan.textContent = s.texto;

        syl.appendChild(idxSpan);
        syl.appendChild(txtSpan);
        strip.appendChild(syl);
      });

      colRight.appendChild(strip);
    } else {
      const emptySpan = document.createElement('span');
      emptySpan.style.color = 'var(--text-muted)';
      emptySpan.style.fontSize = '0.75rem';
      emptySpan.style.fontStyle = 'italic';
      emptySpan.textContent = 'Verso vazio';
      colRight.appendChild(emptySpan);
    }
  });
}

function focusVerse(index) {
  setTimeout(() => {
    const input = versesContainer.querySelector(`input[data-index="${index}"]`);
    if (input) {
      input.focus();
    }
  }, 30);
}

async function copiarAnalise() {
  const fullText = getFullText();
  if (!fullText.trim()) return;

  const modo = selectModo.value;
  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  const content = escandir(fullText, modo === 'visual' ? 'painel' : modo, pontuacao, dierese, castilho, rima);

  try {
    await navigator.clipboard.writeText(content);
    const orig = btnCopiar.textContent;
    btnCopiar.textContent = 'Copiado! ✓';
    btnCopiar.style.borderColor = 'var(--accent-green)';
    btnCopiar.style.color = 'var(--accent-green)';
    setTimeout(() => {
      btnCopiar.textContent = orig;
      btnCopiar.style.borderColor = '';
      btnCopiar.style.color = '';
    }, 1800);
  } catch (err) {
    console.error('Falha ao copiar:', err);
  }
}

main();

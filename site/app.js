import init, { escandir, carregar_exemplos, versao } from './pkg/lirica.js?v=0.1.8';

// Persistência em LocalStorage (Ticket #20 / #23)
const STORAGE_KEY = 'lirica_draft_poem';
const STORAGE_DEBOUNCE_MS = 400;
let saveTimer = null;
let isDirty = false;

function salvarRascunhoImediato(texto = null) {
  isDirty = true;
  try {
    if (saveTimer) {
      clearTimeout(saveTimer);
      saveTimer = null;
    }
    const conteudo = typeof texto === 'string' ? texto : getFullText();
    localStorage.setItem(STORAGE_KEY, conteudo);
  } catch (err) {
    console.warn('Falha ao persistir rascunho no localStorage:', err);
  }
}

function agendarSalvamentoRascunho(texto = null) {
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    salvarRascunhoImediato(texto);
  }, STORAGE_DEBOUNCE_MS);
}

function flushRascunhoPendente() {
  if (saveTimer && isDirty) {
    clearTimeout(saveTimer);
    saveTimer = null;
    salvarRascunhoImediato();
  }
}

function recuperarRascunho() {
  try {
    const salvo = localStorage.getItem(STORAGE_KEY);
    return typeof salvo === 'string' && salvo.trim().length > 0 ? salvo : null;
  } catch (err) {
    console.warn('Falha ao ler rascunho do localStorage:', err);
    return null;
  }
}

function limparRascunho() {
  try {
    if (saveTimer) {
      clearTimeout(saveTimer);
      saveTimer = null;
    }
    localStorage.removeItem(STORAGE_KEY);
  } catch (err) {
    console.warn('Falha ao remover rascunho do localStorage:', err);
  }
}

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

// Metadados Poéticos (Ticket #24)
const inputTitulo = document.getElementById('input-titulo');
const inputAutor = document.getElementById('input-autor');

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
      carregarPoema(ex.texto, ex.titulo, ex.autor);
      fecharModal();
    });

    gridExemplos.appendChild(card);
  });
}

function carregarPoema(texto, titulo = '', autor = '') {
  if (inputTitulo) inputTitulo.value = titulo;
  if (inputAutor) inputAutor.value = autor;
  verses = texto.split('\n');
  if (rawTextarea) rawTextarea.value = getFullText();
  limparRascunho();
  isDirty = false;
  renderWorkspace();
  focusVerse(0);
}

function sortearExemploAleatorio() {
  if (!Array.isArray(exemplos) || exemplos.length === 0) return null;
  const idx = Math.floor(Math.random() * exemplos.length);
  return exemplos[idx] && exemplos[idx].texto ? exemplos[idx] : null;
}

function extrairMetadadosMarkdown(allLines) {
  let titulo = '';
  let autor = '';
  const poemLines = [];

  for (const l of allLines) {
    const trimmed = l.trim();
    if (trimmed.startsWith('%%')) continue;
    if (trimmed.startsWith('# ')) {
      titulo = trimmed.slice(2).trim();
    } else if (trimmed.startsWith('## ')) {
      autor = trimmed.slice(3).trim();
    } else if (trimmed.startsWith('#') && !trimmed.startsWith('##')) {
      titulo = trimmed.replace(/^#+/, '').trim();
    } else if (trimmed.startsWith('##')) {
      autor = trimmed.replace(/^#+/, '').trim();
    } else {
      poemLines.push(l);
    }
  }

  while (poemLines.length > 0 && poemLines[0].trim() === '') {
    poemLines.shift();
  }

  return { titulo, autor, verses: poemLines.length > 0 ? poemLines : [''] };
}

function limparLaboratorio() {
  limparRascunho();
  isDirty = false;
  if (inputTitulo) inputTitulo.value = '';
  if (inputAutor) inputAutor.value = '';
  verses = [''];
  if (rawTextarea) rawTextarea.value = '';
  renderWorkspace();
  focusVerse(0);
}

async function main() {
  // Recuperar rascunho persistido no localStorage (Ticket #20 / #24)
  const rascunhoSalvo = recuperarRascunho();
  if (rascunhoSalvo !== null) {
    const meta = extrairMetadadosMarkdown(rascunhoSalvo.split('\n'));
    if (inputTitulo) inputTitulo.value = meta.titulo;
    if (inputAutor) inputAutor.value = meta.autor;
    verses = meta.verses;
    isDirty = true;
  }

  try {
    await init('./pkg/lirica_bg.wasm?v=0.1.8');
  } catch (err) {
    console.error('Falha ao inicializar WebAssembly do Lírica:', err);
    if (formDesc) {
      formDesc.textContent = 'Erro ao carregar o motor WebAssembly. Verifique a conexão e recarregue a página.';
      formDesc.style.color = 'var(--accent-red)';
    }
    return;
  }

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

    // Se não houver rascunho salvo, sortear aleatoriamente um poema da biblioteca de exemplos
    if (rascunhoSalvo === null) {
      const exAleatorio = sortearExemploAleatorio();
      if (exAleatorio && exAleatorio.texto) {
        if (inputTitulo) inputTitulo.value = exAleatorio.titulo || '';
        if (inputAutor) inputAutor.value = exAleatorio.autor || '';
        verses = exAleatorio.texto.split('\n');
      }
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
    salvarRascunhoImediato();
    renderWorkspace();
    focusVerse(verses.length - 1);
  });

  if (btnAddEstrofe) {
    btnAddEstrofe.addEventListener('click', () => {
      if (verses.length > 0 && verses[verses.length - 1] !== '') {
        verses.push('');
      }
      verses.push('');
      salvarRascunhoImediato();
      renderWorkspace();
      focusVerse(verses.length - 1);
    });
  }

  // Listeners de Título e Autor (Ticket #24)
  if (inputTitulo) {
    inputTitulo.addEventListener('input', () => {
      isDirty = true;
      agendarSalvamentoRascunho();
    });
  }
  if (inputAutor) {
    inputAutor.addEventListener('input', () => {
      isDirty = true;
      agendarSalvamentoRascunho();
    });
  }

  btnToggleRaw.addEventListener('click', toggleRawEditor);

  btnApplyRaw.addEventListener('click', () => {
    const rawVal = rawTextarea.value;
    const meta = extrairMetadadosMarkdown(rawVal.split('\n'));

    if (inputTitulo) inputTitulo.value = meta.titulo;
    if (inputAutor) inputAutor.value = meta.autor;
    verses = meta.verses;

    isDirty = true;
    salvarRascunhoImediato();
    rawEditorBox.style.display = 'none';
    btnToggleRaw.textContent = 'Modo Texto Livre';
    renderWorkspace();
  });

  btnCopiar.addEventListener('click', copiarAnalise);

  renderWorkspace();
}

function toggleRawEditor() {
  if (rawEditorBox.style.display === 'none') {
    rawTextarea.value = getFullText();
    rawEditorBox.style.display = 'flex';
    btnToggleRaw.textContent = 'Fechar Texto Livre';
    rawTextarea.focus();
  } else {
    rawEditorBox.style.display = 'none';
    btnToggleRaw.textContent = 'Modo Texto Livre';
  }
}

function getFullText() {
  const parts = [];
  const tit = inputTitulo ? inputTitulo.value.trim() : '';
  const aut = inputAutor ? inputAutor.value.trim() : '';
  if (tit) parts.push(`# ${tit}`);
  if (aut) parts.push(`## ${aut}`);
  if (parts.length > 0) parts.push('');
  return parts.concat(verses).join('\n');
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
    } else if (forma.nome.includes('Sonetilha')) {
      formIcon.textContent = '✨';
    } else if (forma.nome.includes('Espinela')) {
      formIcon.textContent = '📜';
    } else if (forma.nome.includes('Martelo')) {
      formIcon.textContent = '🔨';
    } else if (forma.nome.includes('Cordel') || forma.nome.includes('Sextilha') || forma.nome.includes('Septilha')) {
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

    let morfResumo = '';
    if (webData.rimas_morfologicas && webData.rimas_morfologicas.length > 0) {
      let ricas = 0, pobres = 0, cliches = 0, provaveis = 0;
      webData.rimas_morfologicas.forEach(est => {
        if (Array.isArray(est.pares)) {
          est.pares.forEach(p => {
            if (p.qualidade === 'Rica') ricas++;
            else if (p.qualidade === 'Pobre') pobres++;
            else if (p.qualidade && p.qualidade.startsWith('Provavel')) provaveis++;
            if (p.eh_cliche) cliches++;
          });
        }
      });
      const parts = [];
      if (ricas > 0) parts.push(`${ricas} Rica${ricas > 1 ? 's' : ''}`);
      if (pobres > 0) parts.push(`${pobres} Pobre${pobres > 1 ? 's' : ''}`);
      if (provaveis > 0) parts.push(`${provaveis} Provável`);
      if (cliches > 0) parts.push(`${cliches} Clichê${cliches > 1 ? 's' : ''}`);
      if (parts.length > 0) {
        morfResumo = ` • ${parts.join(', ')}`;
      }
    }

    rhymeDispositionDesc.textContent = `${ar.tipo_disposicao}${morfResumo}`;
    rhymePercentTag.textContent = `${ar.percentual_rimado}% Versos Rimados`;
  } else {
    rhymeSchemeTitle.textContent = 'Rima: --';
    rhymePurityBadge.textContent = 'Desativada';
    rhymeDispositionDesc.textContent = 'Análise de rimas desativada ou sem versos.';
    rhymePercentTag.textContent = '0% Versos Rimados';
  }
}

let activeRhymePair = null;

function highlightActiveRhymeVerses() {
  const rows = versesContainer.querySelectorAll('.verse-row');
  rows.forEach(r => {
    const lineNum = parseInt(r.dataset.lineNumber, 10);
    if (activeRhymePair && (lineNum === activeRhymePair.versos[0] || lineNum === activeRhymePair.versos[1])) {
      r.classList.add('verse-highlight-rhyme');
    } else {
      r.classList.remove('verse-highlight-rhyme');
    }
  });
}

function createStanzaHeaderBlock(stanzaIdx, webData, canDelete, onDelete) {
  const block = document.createElement('div');
  block.className = 'stanza-header-block';
  if (stanzaIdx === 0) {
    block.classList.add('first-stanza');
  }

  const row = document.createElement('div');
  row.className = 'stanza-header-row';

  const left = document.createElement('div');
  left.className = 'stanza-header-left';

  const badge = document.createElement('span');
  badge.className = 'stanza-badge';
  badge.innerHTML = `❦ Estrofe ${stanzaIdx + 1}`;
  left.appendChild(badge);

  const drawer = document.createElement('div');
  drawer.className = 'stanza-audit-drawer';
  drawer.style.display = 'none';

  const morfStanza = webData && Array.isArray(webData.rimas_morfologicas)
    ? webData.rimas_morfologicas.find(s => s.estrofe_indice === stanzaIdx)
    : null;

  if (morfStanza && Array.isArray(morfStanza.pares) && morfStanza.pares.length > 0) {
    const strip = document.createElement('div');
    strip.className = 'stanza-rhyme-strip';

    morfStanza.pares.forEach((par, pIdx) => {
      const btn = document.createElement('button');
      btn.type = 'button';
      btn.className = 'rhyme-pair-badge';

      let qualityClass = 'quality-pobre';
      let icon = '';
      let qualLabel = par.qualidade;

      switch (par.qualidade) {
        case 'Rica':
          qualityClass = 'quality-rica';
          icon = '✨ ';
          break;
        case 'Preciosa':
          qualityClass = 'quality-preciosa';
          icon = '💎 ';
          break;
        case 'Rara':
          qualityClass = 'quality-rara';
          icon = '✦ ';
          break;
        case 'ProvavelPobre':
          qualityClass = 'quality-provavel-pobre';
          icon = '⚡ ';
          qualLabel = 'Provável';
          break;
        case 'ProvavelRica':
          qualityClass = 'quality-provavel-rica';
          icon = '⚡ ';
          qualLabel = 'Provável Rica';
          break;
        case 'Indeterminada':
          qualityClass = 'quality-indeterminada';
          icon = '? ';
          break;
        default:
          qualityClass = 'quality-pobre';
          break;
      }

      btn.classList.add(qualityClass);
      btn.innerHTML = `${icon}${qualLabel}: ${par.tokens[0]} × ${par.tokens[1]}`;

      if (par.eh_cliche) {
        const cTag = document.createElement('span');
        cTag.className = 'cliche-tag';
        cTag.textContent = '🔥 Clichê';
        btn.appendChild(cTag);
      }

      btn.addEventListener('click', () => {
        const isCurrent = activeRhymePair &&
          activeRhymePair.stanzaIdx === stanzaIdx &&
          activeRhymePair.pairIdx === pIdx;

        document.querySelectorAll('.rhyme-pair-badge').forEach(b => b.classList.remove('is-selected'));
        document.querySelectorAll('.stanza-audit-drawer').forEach(d => { d.style.display = 'none'; });

        if (isCurrent) {
          activeRhymePair = null;
          highlightActiveRhymeVerses();
        } else {
          activeRhymePair = { stanzaIdx, pairIdx: pIdx, versos: par.versos };
          btn.classList.add('is-selected');
          highlightActiveRhymeVerses();

          drawer.innerHTML = `
            <div class="audit-card-header">
              <span class="audit-card-title">✦ Auditoria da Rima • v${par.versos[0]} (${par.tokens[0]}) × v${par.versos[1]} (${par.tokens[1]})</span>
              <button type="button" class="btn-close-audit" aria-label="Fechar">&times;</button>
            </div>
            <div class="audit-card-body">
              <div class="audit-card-row">
                <strong>Classificação:</strong> <span>${par.qualidade}</span>
              </div>
              ${par.detalhe ? `<div class="audit-card-row"><strong>Diagnóstico:</strong> <span>${par.detalhe}</span></div>` : ''}
              ${par.eh_cliche ? `
                <div class="audit-cliche-alert">
                  <strong>🔥 Clichê Histórico:</strong> ${par.motivo_cliche || 'Par hiper-saturado na tradição lírica.'}
                </div>
              ` : ''}
              <div class="audit-card-row" style="font-size: 0.75rem; opacity: 0.8; margin-top: 0.2rem;">
                <strong>Auditoria Fonotática:</strong> <span>${par.auditoria}</span>
              </div>
            </div>
          `;
          drawer.style.display = 'block';

          drawer.querySelector('.btn-close-audit').addEventListener('click', () => {
            drawer.style.display = 'none';
            btn.classList.remove('is-selected');
            activeRhymePair = null;
            highlightActiveRhymeVerses();
          });
        }
      });

      strip.appendChild(btn);
    });

    left.appendChild(strip);
  }

  row.appendChild(left);

  if (canDelete) {
    const btnDel = document.createElement('button');
    btnDel.className = 'btn-del-stanza';
    btnDel.title = 'Remover quebra de estrofe';
    btnDel.innerHTML = '&times;';
    btnDel.addEventListener('click', onDelete);
    row.appendChild(btnDel);
  }

  block.appendChild(row);
  block.appendChild(drawer);
  return block;
}

function renderWorkspace() {
  const modo = selectModo.value;
  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  const poemText = verses.join('\n');

  if (modo === 'visual') {
    alignedWorkspace.style.display = 'flex';
    codeWorkspace.style.display = 'none';
    renderAlignedRows(poemText, castilho, pontuacao, dierese, rima);
  } else {
    alignedWorkspace.style.display = 'none';
    codeWorkspace.style.display = 'block';
    const output = escandir(poemText, modo, pontuacao, dierese, castilho, rima);
    codeContent.textContent = output || '// Nenhum texto para analisar.';
    updateStats(null);
  }
}

function renderAlignedRows(poemText, castilho, pontuacao, dierese, rima) {
  let webData = null;
  let parsed = [];
  if (poemText.trim().length > 0) {
    try {
      const jsonStr = escandir(poemText, 'visual', pontuacao, dierese, castilho, rima);
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

  // Render first stanza header block if poem has stanzas
  if (webData && Array.isArray(webData.estrofes) && webData.estrofes.length > 0) {
    versesContainer.appendChild(createStanzaHeaderBlock(0, webData, false, null));
  }

  let parsedIdx = 0;
  let lineCounter = 1;
  let stanzaCounter = 1;

  verses.forEach((verseText, idx) => {
    // Quebra de estrofe (linha vazia entre versos)
    if (verseText.trim().length === 0) {
      stanzaCounter++;
      const block = createStanzaHeaderBlock(stanzaCounter - 1, webData, true, () => {
        verses.splice(idx, 1);
        salvarRascunhoImediato();
        renderWorkspace();
      });
      versesContainer.appendChild(block);
      return;
    }

    const row = document.createElement('div');
    row.className = 'verse-row';
    const currentLineNum = lineCounter++;
    row.dataset.lineNumber = `${currentLineNum}`;

    // Coluna Esquerda: Verso Editável
    const colLeft = document.createElement('div');
    colLeft.className = 'verse-col-left';

    const numSpan = document.createElement('span');
    numSpan.className = 'line-num';
    numSpan.textContent = `${currentLineNum}`;

    const input = document.createElement('input');
    input.type = 'text';
    input.className = 'verse-input';
    input.value = verseText;
    input.placeholder = `Verso ${idx + 1}...`;
    input.dataset.index = idx;

    input.addEventListener('input', (e) => {
      verses[idx] = e.target.value;
      updateSingleRowLive();
      agendarSalvamentoRascunho();
    });

    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        if (e.shiftKey) {
          // Shift+Enter insere quebra de estrofe
          verses.splice(idx + 1, 0, '', '');
          salvarRascunhoImediato();
          renderWorkspace();
          focusVerse(idx + 2);
        } else {
          verses.splice(idx + 1, 0, '');
          salvarRascunhoImediato();
          renderWorkspace();
          focusVerse(idx + 1);
        }
      } else if (e.key === 'Backspace' && e.target.value === '' && verses.length > 1) {
        e.preventDefault();
        verses.splice(idx, 1);
        salvarRascunhoImediato();
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
      salvarRascunhoImediato();
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

        let verseIsCliche = false;
        let clicheMotivo = '';
        if (webData && Array.isArray(webData.rimas_morfologicas)) {
          for (const est of webData.rimas_morfologicas) {
            if (Array.isArray(est.pares)) {
              for (const p of est.pares) {
                if (p.eh_cliche && (p.versos[0] === currentLineNum || p.versos[1] === currentLineNum)) {
                  verseIsCliche = true;
                  clicheMotivo = p.motivo_cliche || '';
                  break;
                }
              }
            }
            if (verseIsCliche) break;
          }
        }

        if (verseIsCliche) {
          rhymeBadge.classList.add('is-cliche');
          rhymeBadge.innerHTML = `${vData.rima} <span class="ico-cliche">🔥</span>`;
          rhymeBadge.title = `Rima: ${vData.rima} (Clichê lírico: ${clicheMotivo})`;
        } else {
          rhymeBadge.textContent = vData.rima;
          const term = vData.rima_terminacao ? ` (som: "${vData.rima_terminacao}")` : '';
          rhymeBadge.title = `Rima: ${vData.rima}${term}`;
        }
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
  const poemText = verses.join('\n');
  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  let webData = null;
  let parsed = [];
  if (poemText.trim().length > 0) {
    try {
      const jsonStr = escandir(poemText, 'visual', pontuacao, dierese, castilho, rima);
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
        const lineNum = currentVerseRowIdx; // currentVerseRowIdx is 1-based line number

        let verseIsCliche = false;
        let clicheMotivo = '';
        if (webData && Array.isArray(webData.rimas_morfologicas)) {
          for (const est of webData.rimas_morfologicas) {
            if (Array.isArray(est.pares)) {
              for (const p of est.pares) {
                if (p.eh_cliche && (p.versos[0] === lineNum || p.versos[1] === lineNum)) {
                  verseIsCliche = true;
                  clicheMotivo = p.motivo_cliche || '';
                  break;
                }
              }
            }
            if (verseIsCliche) break;
          }
        }

        if (verseIsCliche) {
          rhymeBadge.classList.add('is-cliche');
          rhymeBadge.innerHTML = `${vData.rima} <span class="ico-cliche">🔥</span>`;
          rhymeBadge.title = `Rima: ${vData.rima} (Clichê lírico: ${clicheMotivo})`;
        } else {
          rhymeBadge.textContent = vData.rima;
          const term = vData.rima_terminacao ? ` (som: "${vData.rima_terminacao}")` : '';
          rhymeBadge.title = `Rima: ${vData.rima}${term}`;
        }
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
  const poemText = verses.join('\n');
  if (!poemText.trim()) return;

  const modo = selectModo.value;
  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  const content = escandir(poemText, modo === 'visual' ? 'painel' : modo, pontuacao, dierese, castilho, rima);

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

main().catch(err => {
  console.error('Falha crítica na inicialização do Lírica:', err);
});

// Flush imediato de rascunho em saída da página ou troca de aba (Ticket #20)
window.addEventListener('beforeunload', flushRascunhoPendente);
document.addEventListener('visibilitychange', () => {
  if (document.visibilityState === 'hidden') {
    flushRascunhoPendente();
  }
});

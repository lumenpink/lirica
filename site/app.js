import init, { escandir, carregar_exemplos, versao } from './pkg/lirica.js';

let exemplos = [];
let verses = [
  "Amor é fogo que arde sem se ver,",
  "É ferida que dói e não se sente;",
  "É um contentamento descontente,",
  "É dor que desatina sem doer."
];

// Elements
const selectExemplo = document.getElementById('select-exemplo');
const selectModo = document.getElementById('select-modo');
const chkCastilho = document.getElementById('chk-castilho');
const chkPontuacao = document.getElementById('chk-pontuacao');
const chkDierese = document.getElementById('chk-dierese');
const chkRima = document.getElementById('chk-rima');

const counterVersos = document.getElementById('counter-versos');
const schemeBadge = document.getElementById('scheme-badge');

const btnToggleRaw = document.getElementById('btn-toggle-raw');
const btnAddVerso = document.getElementById('btn-add-verso');
const btnLimpar = document.getElementById('btn-limpar');
const btnCopiar = document.getElementById('btn-copiar');
const lblVersao = document.getElementById('lbl-versao');

const rawEditorBox = document.getElementById('raw-editor-box');
const rawTextarea = document.getElementById('raw-textarea');
const btnApplyRaw = document.getElementById('btn-apply-raw');

const alignedWorkspace = document.getElementById('aligned-workspace');
const versesContainer = document.getElementById('verses-container');

const codeWorkspace = document.getElementById('code-workspace');
const codeContent = document.getElementById('code-content');

async function main() {
  await init();

  // Version
  try {
    const v = versao();
    if (lblVersao) lblVersao.textContent = `v${v}`;
  } catch (e) {
    console.error('Erro de versão:', e);
  }

  // Load Examples
  try {
    exemplos = JSON.parse(carregar_exemplos());
    selectExemplo.innerHTML = '<option value="">— Escolha um exemplo clássico —</option>';
    exemplos.forEach(ex => {
      const opt = document.createElement('option');
      opt.value = ex.id;
      opt.textContent = `${ex.titulo} (${ex.autor})`;
      selectExemplo.appendChild(opt);
    });

    // Default Camões
    const defaultEx = exemplos.find(ex => ex.id === 'camoes');
    if (defaultEx) {
      selectExemplo.value = defaultEx.id;
      verses = defaultEx.texto.split('\n').filter(l => l.trim().length > 0 && !l.trim().startsWith('%%'));
    }
  } catch (e) {
    console.error('Erro ao carregar exemplos:', e);
  }

  // Event Listeners
  selectExemplo.addEventListener('change', (e) => {
    const selected = exemplos.find(ex => ex.id === e.target.value);
    if (selected) {
      verses = selected.texto.split('\n').filter(l => l.trim().length > 0 && !l.trim().startsWith('%%'));
      renderWorkspace();
    }
  });

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

  btnLimpar.addEventListener('click', () => {
    verses = [''];
    selectExemplo.value = '';
    rawTextarea.value = '';
    renderWorkspace();
    focusVerse(0);
  });

  btnToggleRaw.addEventListener('click', toggleRawEditor);

  btnApplyRaw.addEventListener('click', () => {
    const lines = rawTextarea.value
      .split('\n')
      .map(l => l.trim())
      .filter(l => l.length > 0 && !l.startsWith('%%'));
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
  return verses.filter(v => v.trim().length > 0).join('\n');
}

function updateStats(parsedVersos) {
  const count = verses.length;
  counterVersos.textContent = `${count} ${count === 1 ? 'verso' : 'versos'}`;

  if (chkRima.checked && parsedVersos && parsedVersos.length > 0) {
    const scheme = parsedVersos
      .map(v => v.rima || '-')
      .filter(r => r !== '-')
      .join('');
    if (scheme) {
      schemeBadge.textContent = `Rima: ${scheme}`;
      schemeBadge.style.display = 'inline-block';
    } else {
      schemeBadge.style.display = 'none';
    }
  } else {
    schemeBadge.style.display = 'none';
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
    updateStats([]);
  }
}

function renderAlignedRows(fullText, castilho, pontuacao, dierese, rima) {
  let parsed = [];
  if (fullText.trim().length > 0) {
    try {
      const jsonStr = escandir(fullText, 'visual', pontuacao, dierese, castilho, rima);
      parsed = JSON.parse(jsonStr);
    } catch (e) {
      console.error('Falha de escansão:', e);
    }
  }

  updateStats(parsed);

  versesContainer.innerHTML = '';

  let parsedIdx = 0;

  verses.forEach((verseText, idx) => {
    const row = document.createElement('div');
    row.className = 'verse-row';

    // Coluna Esquerda: Verso Editável
    const colLeft = document.createElement('div');
    colLeft.className = 'verse-col-left';

    const numSpan = document.createElement('span');
    numSpan.className = 'line-num';
    numSpan.textContent = `${idx + 1}`;

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
        verses.splice(idx + 1, 0, '');
        renderWorkspace();
        focusVerse(idx + 1);
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

    const isNonEmpty = verseText.trim().length > 0;
    const vData = isNonEmpty ? parsed[parsedIdx++] : null;

    if (vData) {
      // Rhyme badge
      if (vData.rima) {
        const rhymeBadge = document.createElement('span');
        rhymeBadge.className = 'badge-rima';
        rhymeBadge.textContent = vData.rima;
        if (vData.rima_terminacao) {
          rhymeBadge.title = `Terminação fonética: ${vData.rima_terminacao}`;
        }
        colRight.appendChild(rhymeBadge);
      }

      // Meter badge
      const meterBadge = document.createElement('span');
      meterBadge.className = 'badge-metro';
      meterBadge.textContent = `${vData.total_metricas} ${vData.classificacao}`;
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

  let parsed = [];
  if (fullText.trim().length > 0) {
    try {
      const jsonStr = escandir(fullText, 'visual', pontuacao, dierese, castilho, rima);
      parsed = JSON.parse(jsonStr);
    } catch (e) {
      console.error(e);
    }
  }

  updateStats(parsed);

  const rows = versesContainer.querySelectorAll('.verse-row');
  let parsedIdx = 0;

  verses.forEach((verseText, idx) => {
    const row = rows[idx];
    if (!row) return;

    const colRight = row.querySelector('.verse-col-right');
    if (!colRight) return;

    colRight.innerHTML = '';
    const isNonEmpty = verseText.trim().length > 0;
    const vData = isNonEmpty ? parsed[parsedIdx++] : null;

    if (vData) {
      if (vData.rima) {
        const rhymeBadge = document.createElement('span');
        rhymeBadge.className = 'badge-rima';
        rhymeBadge.textContent = vData.rima;
        if (vData.rima_terminacao) {
          rhymeBadge.title = `Terminação fonética: ${vData.rima_terminacao}`;
        }
        colRight.appendChild(rhymeBadge);
      }

      const meterBadge = document.createElement('span');
      meterBadge.className = 'badge-metro';
      meterBadge.textContent = `${vData.total_metricas} ${vData.classificacao}`;
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

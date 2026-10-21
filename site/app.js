import init, { escandir, carregar_exemplos, versao } from './pkg/lirica.js';

let exemplos = [];

const editorEl = document.getElementById('input-poema');
const outputContainer = document.getElementById('output-container');
const outputTitle = document.getElementById('output-title');
const selectExemplo = document.getElementById('select-exemplo');
const selectModo = document.getElementById('select-modo');
const chkCastilho = document.getElementById('chk-castilho');
const chkPontuacao = document.getElementById('chk-pontuacao');
const chkDierese = document.getElementById('chk-dierese');
const chkRima = document.getElementById('chk-rima');
const counterLinhas = document.getElementById('counter-linhas');
const btnLimpar = document.getElementById('btn-limpar');
const btnCopiar = document.getElementById('btn-copiar');
const lblVersao = document.getElementById('lbl-versao');

async function main() {
  await init();

  // Version
  try {
    const v = versao();
    if (lblVersao) lblVersao.textContent = `v${v}`;
  } catch (e) {
    console.error('Erro ao obter versão:', e);
  }

  // Load Examples
  try {
    exemplos = JSON.parse(carregar_exemplos());
    selectExemplo.innerHTML = '<option value="">— Selecione um exemplo clássico —</option>';
    exemplos.forEach(ex => {
      const opt = document.createElement('option');
      opt.value = ex.id;
      opt.textContent = `${ex.titulo} (${ex.autor})`;
      selectExemplo.appendChild(opt);
    });

    // Default to Camões example
    const defaultEx = exemplos.find(ex => ex.id === 'camoes');
    if (defaultEx) {
      editorEl.value = defaultEx.texto;
      selectExemplo.value = defaultEx.id;
    }
  } catch (e) {
    console.error('Erro ao carregar exemplos:', e);
  }

  // Event Listeners
  editorEl.addEventListener('input', runScansion);
  selectModo.addEventListener('change', () => {
    updateTitle();
    runScansion();
  });
  chkCastilho.addEventListener('change', runScansion);
  chkPontuacao.addEventListener('change', runScansion);
  chkDierese.addEventListener('change', runScansion);
  chkRima.addEventListener('change', runScansion);

  selectExemplo.addEventListener('change', (e) => {
    const selected = exemplos.find(ex => ex.id === e.target.value);
    if (selected) {
      editorEl.value = selected.texto;
      runScansion();
    }
  });

  btnLimpar.addEventListener('click', () => {
    editorEl.value = '';
    selectExemplo.value = '';
    runScansion();
    editorEl.focus();
  });

  btnCopiar.addEventListener('click', copiarSaida);

  runScansion();
}

function updateTitle() {
  const modo = selectModo.value;
  switch (modo) {
    case 'visual':
      outputTitle.textContent = 'Painel Visual Interativo';
      break;
    case 'typst':
      outputTitle.textContent = 'Marcação Typst';
      break;
    case 'json':
      outputTitle.textContent = 'AST JSON';
      break;
    case 'texto':
      outputTitle.textContent = 'Texto Puro';
      break;
    case 'painel':
      outputTitle.textContent = 'Texto Estruturado (Dual Panel)';
      break;
  }
}

function updateLineCounter() {
  const raw = editorEl.value;
  const lines = raw.split('\n').filter(l => l.trim().length > 0 && !l.trim().startsWith('%%'));
  const count = lines.length;
  counterLinhas.textContent = `${count} ${count === 1 ? 'verso' : 'versos'}`;
}

function runScansion() {
  updateLineCounter();

  const texto = editorEl.value;
  if (!texto.trim()) {
    outputContainer.innerHTML = `
      <div class="empty-state">
        <span class="empty-icon">✍️</span>
        <p>Digite versos à esquerda ou escolha um exemplo clássico acima para ver a escansão métrica em tempo real.</p>
      </div>
    `;
    return;
  }

  const modo = selectModo.value;
  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  try {
    if (modo === 'visual') {
      const jsonRes = escandir(texto, 'visual', pontuacao, dierese, castilho, rima);
      renderVisual(JSON.parse(jsonRes));
    } else {
      const res = escandir(texto, modo, pontuacao, dierese, castilho, rima);
      renderRaw(res);
    }
  } catch (err) {
    outputContainer.innerHTML = `
      <div class="code-output" style="color: #e06c75;">
        Erro de análise escansiva: ${escapeHtml(String(err))}
      </div>
    `;
  }
}

function renderVisual(versos) {
  if (!versos || versos.length === 0) {
    outputContainer.innerHTML = `
      <div class="empty-state">
        <span class="empty-icon">🔍</span>
        <p>Nenhum verso detectado no texto de entrada.</p>
      </div>
    `;
    return;
  }

  const list = document.createElement('div');
  list.className = 'verse-list';

  versos.forEach((v, idx) => {
    const card = document.createElement('div');
    card.className = 'verse-card';

    // Header row
    const metaRow = document.createElement('div');
    metaRow.className = 'verse-meta-row';

    const numSpan = document.createElement('span');
    numSpan.className = 'verse-num';
    numSpan.textContent = `v.${idx + 1}`;

    const classSpan = document.createElement('span');
    classSpan.className = 'verse-classification';
    classSpan.textContent = v.classificacao;

    const statsSpan = document.createElement('span');
    statsSpan.className = 'verse-stats';
    statsSpan.textContent = `${v.total_metricas} métricas / ${v.total_foneticas} fonéticas`;

    metaRow.appendChild(numSpan);
    metaRow.appendChild(classSpan);
    metaRow.appendChild(statsSpan);

    // Original verse
    const origP = document.createElement('div');
    origP.className = 'verse-original';
    origP.textContent = v.original;

    // Syllables row
    const sylRow = document.createElement('div');
    sylRow.className = 'verse-syllables';

    v.silabas.forEach((s) => {
      const chip = document.createElement('div');
      chip.className = 'syllable-chip';
      if (s.tonica) chip.classList.add('tonica');
      if (s.elidida) chip.classList.add('elidida');
      if (s.numero_metrico === null) chip.classList.add('extrametrica');

      const idxSpan = document.createElement('span');
      idxSpan.className = 'syl-index';
      idxSpan.textContent = s.numero_metrico !== null ? s.numero_metrico : '·';

      const txtSpan = document.createElement('span');
      txtSpan.className = 'syl-text';
      txtSpan.textContent = s.texto;

      chip.appendChild(idxSpan);
      chip.appendChild(txtSpan);
      sylRow.appendChild(chip);
    });

    card.appendChild(metaRow);
    card.appendChild(origP);
    card.appendChild(sylRow);

    list.appendChild(card);
  });

  outputContainer.innerHTML = '';
  outputContainer.appendChild(list);
}

function renderRaw(text) {
  const pre = document.createElement('pre');
  pre.className = 'code-output';
  const code = document.createElement('code');
  code.textContent = text;
  pre.appendChild(code);
  outputContainer.innerHTML = '';
  outputContainer.appendChild(pre);
}

async function copiarSaida() {
  const modo = selectModo.value;
  const texto = editorEl.value;
  if (!texto.trim()) return;

  const castilho = chkCastilho.checked;
  const pontuacao = chkPontuacao.checked;
  const dierese = chkDierese.checked;
  const rima = chkRima.checked;

  const content = escandir(texto, modo === 'visual' ? 'painel' : modo, pontuacao, dierese, castilho, rima);

  try {
    await navigator.clipboard.writeText(content);
    const origText = btnCopiar.textContent;
    btnCopiar.textContent = 'Copiado! ✓';
    btnCopiar.style.borderColor = 'var(--accent-green)';
    btnCopiar.style.color = 'var(--accent-green)';
    setTimeout(() => {
      btnCopiar.textContent = origText;
      btnCopiar.style.borderColor = '';
      btnCopiar.style.color = '';
    }, 1800);
  } catch (err) {
    console.error('Falha ao copiar:', err);
  }
}

function escapeHtml(str) {
  return str.replace(/[&<>'"]/g, tag => ({
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    "'": '&#39;',
    '"': '&quot;'
  }[tag] || tag));
}

main();

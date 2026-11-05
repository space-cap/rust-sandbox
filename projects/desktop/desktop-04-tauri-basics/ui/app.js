// ============================================================
// Tauri 2.0 Basics - Frontend Interaction Controller
// ============================================================

// 1. Tauri 2 IPC 바인딩 해석기 (글로벌 API 또는 런타임 인터널 자동 바인딩)
function getTauriInvoke() {
  if (typeof window !== 'undefined') {
    if (window.__TAURI__ && window.__TAURI__.core && typeof window.__TAURI__.core.invoke === 'function') {
      return window.__TAURI__.core.invoke;
    }
    if (window.__TAURI_INTERNALS__ && typeof window.__TAURI_INTERNALS__.invoke === 'function') {
      return window.__TAURI_INTERNALS__.invoke;
    }
    if (window.__TAURI__ && typeof window.__TAURI__.invoke === 'function') {
      return window.__TAURI__.invoke;
    }
  }
  return null;
}

function getTauriListen() {
  if (typeof window !== 'undefined') {
    if (window.__TAURI__ && window.__TAURI__.event && typeof window.__TAURI__.event.listen === 'function') {
      return window.__TAURI__.event.listen;
    }
  }
  return null;
}

async function invoke(cmd, args = {}) {
  const fn = getTauriInvoke();
  if (fn) {
    try {
      return await fn(cmd, args);
    } catch (err) {
      console.error(`[Tauri IPC Error on ${cmd}]:`, err);
      throw err;
    }
  }
  console.warn(`[Fallback] Tauri IPC unavailable for '${cmd}', args:`, args);
  return null;
}

// ============================================================
// 2. 커스텀 타이틀바 윈도우 조작
// ============================================================
document.getElementById('btn-minimize').addEventListener('click', async () => {
  try {
    await invoke('minimize_window');
  } catch (err) {
    console.error('최소화 실패:', err);
  }
});

document.getElementById('btn-maximize').addEventListener('click', async () => {
  try {
    await invoke('toggle_maximize_window');
  } catch (err) {
    console.error('최대화/복원 실패:', err);
  }
});

document.getElementById('btn-close').addEventListener('click', async () => {
  try {
    await invoke('close_window');
  } catch (err) {
    console.error('창 닫기 실패:', err);
  }
});

// ============================================================
// 3. 탭 전환
// ============================================================
const tabButtons = document.querySelectorAll('.tab-btn');
const tabContents = document.querySelectorAll('.tab-content');

tabButtons.forEach(btn => {
  btn.addEventListener('click', () => {
    const targetId = btn.getAttribute('data-tab');

    tabButtons.forEach(b => b.classList.remove('active'));
    tabContents.forEach(c => c.classList.remove('active'));

    btn.classList.add('active');
    const targetEl = document.getElementById(targetId);
    if (targetEl) targetEl.classList.add('active');

    // 하드웨어 탭 전환 시 데이터 자동 조회
    if (targetId === 'tab-hardware') {
      loadSystemInfo();
    }
  });
});

// ============================================================
// 4. IPC Command: Greet
// ============================================================
const greetInput = document.getElementById('greet-name');
const btnGreet = document.getElementById('btn-greet');
const greetResponse = document.getElementById('greet-response');

async function handleGreet() {
  const name = greetInput.value.trim();
  try {
    const start = performance.now();
    const result = await invoke('greet', { name });
    const latency = (performance.now() - start).toFixed(2);
    greetResponse.innerHTML = `<strong>${result}</strong> <span style="font-size:11px; color:#94a3b8;">(${latency}ms)</span>`;
  } catch (err) {
    greetResponse.innerHTML = `<span style="color:#ef4444;">호출 오류: ${err}</span>`;
  }
}

btnGreet.addEventListener('click', handleGreet);
greetInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') handleGreet();
});

// ============================================================
// 5. State Management: In-Memory Items
// ============================================================
const itemInput = document.getElementById('item-input');
const btnAddItem = document.getElementById('btn-add-item');
const btnClearItems = document.getElementById('btn-clear-items');
const itemsList = document.getElementById('items-list');

function renderItems(items) {
  itemsList.innerHTML = '';
  if (!items || items.length === 0) {
    itemsList.innerHTML = '<li style="color:#94a3b8; justify-content:center;">저장된 상태 항목이 없습니다.</li>';
    return;
  }

  items.forEach((item, index) => {
    const li = document.createElement('li');
    li.innerHTML = `
      <span>${item}</span>
      <button class="item-delete-btn" data-index="${index}">삭제</button>
    `;
    itemsList.appendChild(li);
  });

  // 개별 삭제 버튼 이벤트 바인딩
  document.querySelectorAll('.item-delete-btn').forEach(btn => {
    btn.addEventListener('click', async (e) => {
      const idx = parseInt(e.target.getAttribute('data-index'), 10);
      try {
        const updated = await invoke('delete_item', { index: idx });
        renderItems(updated);
      } catch (err) {
        alert('삭제 실패: ' + err);
      }
    });
  });
}

async function loadItems() {
  try {
    const items = await invoke('get_items');
    renderItems(items);
  } catch (err) {
    console.error('항목 로드 실패:', err);
  }
}

btnAddItem.addEventListener('click', async () => {
  const val = itemInput.value.trim();
  if (!val) return;
  try {
    const updated = await invoke('add_item', { item: val });
    itemInput.value = '';
    renderItems(updated);
  } catch (err) {
    alert('추가 실패: ' + err);
  }
});

itemInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') btnAddItem.click();
});

btnClearItems.addEventListener('click', async () => {
  if (!confirm('정말 모든 상태 항목을 비우시겠습니까?')) return;
  try {
    const cleared = await invoke('clear_items');
    renderItems(cleared);
  } catch (err) {
    alert('초기화 실패: ' + err);
  }
});

// ============================================================
// 6. SHA-256 Hasher
// ============================================================
const hashInput = document.getElementById('hash-input');
const btnHash = document.getElementById('btn-hash');
const hashAlgo = document.getElementById('hash-algo');
const hashLen = document.getElementById('hash-len');
const hashElapsed = document.getElementById('hash-elapsed');
const hashHex = document.getElementById('hash-hex');

btnHash.addEventListener('click', async () => {
  const content = hashInput.value;
  try {
    const res = await invoke('calculate_hash', { content });
    if (res) {
      hashAlgo.textContent = res.algorithm;
      hashLen.textContent = `${res.input_length} 바이트`;
      hashElapsed.textContent = `${res.elapsed_micros} μs (${(res.elapsed_micros / 1000).toFixed(3)} ms)`;
      hashHex.textContent = res.hex_digest;
    }
  } catch (err) {
    hashHex.textContent = '해시 계산 오류: ' + err;
  }
});

// ============================================================
// 7. 하드웨어 시스템 정보 조회
// ============================================================
async function loadSystemInfo() {
  try {
    const info = await invoke('get_system_info');
    if (!info) return;

    document.getElementById('sys-os-name').textContent = info.os_name;
    document.getElementById('sys-os-ver').textContent = `Version ${info.os_version}`;
    document.getElementById('sys-arch').textContent = info.cpu_arch.toUpperCase();
    document.getElementById('sys-kernel').textContent = `Kernel ${info.kernel_version}`;
    document.getElementById('sys-cpu-count').textContent = `${info.cpu_count} 코어`;

    const totalGb = (info.total_memory_mb / 1024).toFixed(1);
    const usedGb = (info.used_memory_mb / 1024).toFixed(1);
    const memPercent = ((info.used_memory_mb / info.total_memory_mb) * 100).toFixed(1);

    document.getElementById('sys-memory').textContent = `${usedGb} GB / ${totalGb} GB`;
    document.getElementById('sys-memory-usage').textContent = `점유율: ${memPercent}%`;
    document.getElementById('sys-hostname').textContent = info.host_name;

    const days = Math.floor(info.uptime_secs / 86400);
    const hours = Math.floor((info.uptime_secs % 86400) / 3600);
    const mins = Math.floor((info.uptime_secs % 3600) / 60);
    document.getElementById('sys-uptime').textContent = `${days}일 ${hours}시간 ${mins}분`;
    document.getElementById('sys-uptime-raw').textContent = `총 ${info.uptime_secs.toLocaleString()}초 가동 중`;
  } catch (err) {
    console.error('시스템 정보 로드 실패:', err);
  }
}

document.getElementById('btn-refresh-sysinfo').addEventListener('click', loadSystemInfo);

// ============================================================
// 8. 실시간 비동기 이벤트 스트림 리스너
// ============================================================
const liveCpu = document.getElementById('live-cpu');
const liveMemory = document.getElementById('live-memory');
const liveItems = document.getElementById('live-items');
const liveTick = document.getElementById('live-tick');
const pulseLogList = document.getElementById('pulse-log-list');
const pulseStatusText = document.getElementById('pulse-status-text');

let logCount = 0;
const MAX_LOGS = 15;

function appendPulseLog(pulse) {
  const item = document.createElement('div');
  item.className = 'log-item';
  item.innerHTML = `
    <span class="log-time">[${pulse.timestamp}]</span>
    <span>틱 #${pulse.tick_count}</span>
    <span class="log-cpu">CPU: ${pulse.cpu_usage.toFixed(1)}%</span>
    <span>RAM: ${pulse.memory_used_mb}MB / ${pulse.memory_total_mb}MB</span>
    <span>관리 항목: ${pulse.active_items_count}개</span>
  `;

  pulseLogList.prepend(item);
  logCount++;
  if (logCount > MAX_LOGS) {
    const last = pulseLogList.lastElementChild;
    if (last) pulseLogList.removeChild(last);
  }
}

async function setupEventListener() {
  const listen = getTauriListen();
  if (listen) {
    try {
      await listen('sys-pulse', (event) => {
        const pulse = event.payload;
        liveCpu.textContent = `${pulse.cpu_usage.toFixed(1)}%`;
        liveMemory.textContent = `${pulse.memory_used_mb} MB`;
        liveItems.textContent = `${pulse.active_items_count}개`;
        liveTick.textContent = `#${pulse.tick_count}`;
        appendPulseLog(pulse);
      });
      pulseStatusText.textContent = 'Rust 스트림 수신 중 (1초 주기)';
    } catch (err) {
      pulseStatusText.textContent = '이벤트 리스닝 실패: ' + err;
    }
  } else {
    pulseStatusText.textContent = 'Tauri 이벤트 리스너 대기 중...';
  }
}

document.getElementById('btn-clear-log').addEventListener('click', () => {
  pulseLogList.innerHTML = '';
  logCount = 0;
});

// ============================================================
// 9. 앱 안전 초기화 (DOM 로드 상태에 무관하게 즉시 또는 이벤트로 실행)
// ============================================================
function initApp() {
  loadItems();
  btnHash.click();
  setupEventListener();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initApp);
} else {
  initApp();
}

// ============================================================
// Tauri 2.0 Resident Utility - Frontend Controller
// ============================================================

// 1. Tauri 2 IPC 바인딩 해석기
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
// 2. DOM 요소 참조
// ============================================================
const valShortcutCount = document.getElementById('val-shortcut-count');
const valNotificationCount = document.getElementById('val-notification-count');

const notifTitleInput = document.getElementById('notif-title');
const notifBodyInput = document.getElementById('notif-body');
const btnSendNotif = document.getElementById('btn-send-notif');
const btnQuickNotif = document.getElementById('btn-quick-notif');

const timerDisplay = document.getElementById('timer-display');
const timerLabel = document.getElementById('timer-label');
const btnStopTimer = document.getElementById('btn-stop-timer');

const noteInput = document.getElementById('note-input');
const btnAddNote = document.getElementById('btn-add-note');
const btnClearNotes = document.getElementById('btn-clear-notes');
const notesList = document.getElementById('notes-list');

const instanceLogList = document.getElementById('instance-log-list');
const eventLogList = document.getElementById('event-log-list');

// ============================================================
// 3. 커스텀 타이틀바 및 창 제어
// ============================================================
document.getElementById('btn-tray-hide').addEventListener('click', async () => {
  try {
    await invoke('hide_to_tray');
  } catch (err) {
    console.error('트레이 숨김 실패:', err);
  }
});

document.getElementById('btn-minimize').addEventListener('click', async () => {
  try {
    await invoke('minimize_window');
  } catch (err) {
    console.error('최소화 실패:', err);
  }
});

document.getElementById('btn-maximize').addEventListener('click', async () => {
  try {
    await invoke('toggle_maximize');
  } catch (err) {
    console.error('최대화 토글 실패:', err);
  }
});

// 타이틀바의 X 클릭 시 종료 대신 트레이로 안전하게 숨김
document.getElementById('btn-close').addEventListener('click', async () => {
  try {
    await invoke('hide_to_tray');
  } catch (err) {
    console.error('창 숨김 실패:', err);
  }
});

document.getElementById('btn-hide-now').addEventListener('click', async () => {
  try {
    await invoke('hide_to_tray');
  } catch (err) {
    console.error('트레이 숨김 실패:', err);
  }
});

document.getElementById('btn-exit-app').addEventListener('click', async () => {
  if (confirm('프로그램을 완전히 종료하시겠습니까? (트레이 상주 해제)')) {
    try {
      await invoke('exit_app');
    } catch (err) {
      console.error('앱 종료 실패:', err);
    }
  }
});

// ============================================================
// 4. 탭 네비게이션
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
  });
});

// ============================================================
// 5. 토스트 알림 발송 핸들러
// ============================================================
async function handleSendNotification(title, body) {
  try {
    const count = await invoke('send_toast_notification', {
      payload: { title, body }
    });
    valNotificationCount.textContent = `${count}회 발송`;
    appendEventLog(`[알림 발송] "${title}" - ${body}`);
  } catch (err) {
    alert('알림 발송 오류: ' + err);
  }
}

btnSendNotif.addEventListener('click', () => {
  const title = notifTitleInput.value.trim() || 'Tauri 2.0 알림';
  const body = notifBodyInput.value.trim() || '알림 본문 내용입니다.';
  handleSendNotification(title, body);
});

btnQuickNotif.addEventListener('click', () => {
  handleSendNotification('⚡ 즉시 알림 테스트', 'Windows 작업표시줄 토스트 알림이 정상적으로 수신되었습니다.');
});

// ============================================================
// 6. 백그라운드 퀵 타이머
// ============================================================
function formatSeconds(sec) {
  const m = Math.floor(sec / 60).toString().padStart(2, '0');
  const s = (sec % 60).toString().padStart(2, '0');
  return `${m}:${s}`;
}

function updateTimerDisplay(status) {
  if (!status) return;
  timerDisplay.textContent = formatSeconds(status.remaining_secs);
  timerLabel.textContent = status.is_running
    ? `실행 중: ${status.label} (${status.remaining_secs}초 남음)`
    : (status.label || '대기 중');
}

document.querySelectorAll('.preset-btn[data-secs]').forEach(btn => {
  btn.addEventListener('click', async () => {
    const secs = parseInt(btn.getAttribute('data-secs'), 10);
    const label = btn.getAttribute('data-label');
    try {
      const status = await invoke('start_timer', { secs, label });
      updateTimerDisplay(status);
      appendEventLog(`[타이머 시작] ${label} (${secs}초)`);
    } catch (err) {
      alert('타이머 시작 실패: ' + err);
    }
  });
});

btnStopTimer.addEventListener('click', async () => {
  try {
    const status = await invoke('stop_timer');
    updateTimerDisplay(status);
    appendEventLog('[타이머 중지] 사용자가 타이머를 취소했습니다.');
  } catch (err) {
    console.error('타이머 중지 실패:', err);
  }
});

// ============================================================
// 7. 클립보드 & 퀵 메모
// ============================================================
function renderNotes(notes) {
  notesList.innerHTML = '';
  if (!notes || notes.length === 0) {
    notesList.innerHTML = '<li class="log-empty">저장된 퀵 메모가 없습니다.</li>';
    return;
  }

  notes.forEach(note => {
    const li = document.createElement('li');
    li.className = 'note-item';
    li.innerHTML = `
      <div class="note-content">
        <span class="note-text">${escapeHtml(note.text)}</span>
        <span class="note-time">작성: ${note.created_at}</span>
      </div>
      <button class="note-delete-btn" data-id="${note.id}" title="메모 삭제">✕</button>
    `;
    notesList.appendChild(li);
  });

  // 삭제 버튼 이벤트 바인딩
  notesList.querySelectorAll('.note-delete-btn').forEach(btn => {
    btn.addEventListener('click', async (e) => {
      const id = parseInt(btn.getAttribute('data-id'), 10);
      try {
        const updated = await invoke('delete_clipboard_note', { id });
        renderNotes(updated);
        appendEventLog(`[메모 삭제] ID #${id}`);
      } catch (err) {
        console.error('메모 삭제 실패:', err);
      }
    });
  });
}

async function handleAddNote() {
  const text = noteInput.value.trim();
  if (!text) return;

  try {
    const updated = await invoke('add_clipboard_note', { text });
    noteInput.value = '';
    renderNotes(updated);
    appendEventLog(`[메모 추가] "${text.substring(0, 20)}..."`);
  } catch (err) {
    alert('메모 추가 실패: ' + err);
  }
}

btnAddNote.addEventListener('click', handleAddNote);
noteInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') handleAddNote();
});

btnClearNotes.addEventListener('click', async () => {
  if (confirm('저장된 모든 메모를 비우시겠습니까?')) {
    try {
      const cleared = await invoke('clear_clipboard_notes');
      renderNotes(cleared);
      appendEventLog('[메모 전체 삭제]');
    } catch (err) {
      console.error('메모 전체 삭제 실패:', err);
    }
  }
});

// ============================================================
// 8. 활동 로그 렌더링
// ============================================================
function appendEventLog(msg) {
  const li = document.createElement('li');
  li.className = 'log-item highlight';
  const now = new Date().toTimeString().split(' ')[0];
  li.textContent = `[${now}] ${msg}`;
  eventLogList.prepend(li);

  if (eventLogList.children.length > 50) {
    eventLogList.removeChild(eventLogList.lastElementChild);
  }
}

function renderInstanceLogs(events) {
  instanceLogList.innerHTML = '';
  if (!events || events.length === 0) {
    instanceLogList.innerHTML = '<li class="log-empty">아직 중복 실행 시도 이벤트가 없습니다.</li>';
    return;
  }

  events.forEach(entry => {
    const li = document.createElement('li');
    li.className = 'log-item';
    li.textContent = entry;
    instanceLogList.appendChild(li);
  });
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

// ============================================================
// 9. 실시간 이벤트 스트림 리스너 (Tauri Events)
// ============================================================
async function setupEventListeners() {
  const listen = getTauriListen();
  if (!listen) {
    console.warn('Tauri event listen API unavailable');
    return;
  }

  try {
    // 1초 주기 백그라운드 틱 수신
    await listen('status-tick', (event) => {
      const summary = event.payload;
      valShortcutCount.textContent = `${summary.shortcut_pressed_count}회 호출`;
      valNotificationCount.textContent = `${summary.notification_count}회 발송`;
      updateTimerDisplay(summary.timer);
    });

    // 단축키 눌림 이벤트 수신
    await listen('shortcut-triggered', (event) => {
      const count = event.payload;
      valShortcutCount.textContent = `${count}회 호출`;
      appendEventLog(`[전역 단축키] Ctrl+Shift+U 입력 감지 (누적 ${count}회)`);
    });

    // 백그라운드 타이머 완료 이벤트 수신
    await listen('timer-finished', (event) => {
      const label = event.payload;
      appendEventLog(`[타이머 완료] '${label}' 백그라운드 완료 토스트 발송됨`);
      timerDisplay.textContent = '00:00';
      timerLabel.textContent = `🎉 완료됨: ${label}`;
    });

    // 중복 실행 감지 이벤트 수신
    await listen('single-instance-event', (event) => {
      const args = event.payload;
      appendEventLog(`[중복 실행 차단] 외부 프로세스 실행 시도 감지 (인수: ${JSON.stringify(args)})`);
      loadInitialState();
    });
  } catch (err) {
    console.error('이벤트 리스너 등록 실패:', err);
  }
}

// ============================================================
// 10. 앱 초기 상태 로드
// ============================================================
async function loadInitialState() {
  try {
    const summary = await invoke('get_app_status');
    if (summary) {
      valShortcutCount.textContent = `${summary.shortcut_pressed_count}회 호출`;
      valNotificationCount.textContent = `${summary.notification_count}회 발송`;
      updateTimerDisplay(summary.timer);
      renderNotes(summary.notes);
      renderInstanceLogs(summary.single_instance_events);
      appendEventLog('[앱 시작] 상주 백그라운드 유틸리티 초기화 완료');
    }
  } catch (err) {
    console.error('초기 상태 로드 실패:', err);
  }
}

function initApp() {
  loadInitialState();
  setupEventListeners();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initApp);
} else {
  initApp();
}


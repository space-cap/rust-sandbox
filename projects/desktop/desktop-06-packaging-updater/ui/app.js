// ============================================================
// Tauri 2.0 Packaging & Auto-Updater - Frontend Controller
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
// 2. 창 제어 & 타이틀바
// ============================================================
document.getElementById('btn-minimize').addEventListener('click', async () => {
  try { await invoke('minimize_window'); } catch (e) { console.error(e); }
});

document.getElementById('btn-maximize').addEventListener('click', async () => {
  try { await invoke('toggle_maximize'); } catch (e) { console.error(e); }
});

document.getElementById('btn-close').addEventListener('click', async () => {
  try { await invoke('close_window'); } catch (e) { console.error(e); }
});

// ============================================================
// 3. 탭 네비게이션
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
// 4. 자동 업데이트(Auto-Updater) 시뮬레이션
// ============================================================
const inputTargetVer = document.getElementById('input-target-ver');
const btnCheckUpdate = document.getElementById('btn-check-update');
const btnResetUpdate = document.getElementById('btn-reset-update');
const updateProgressBar = document.getElementById('update-progress-bar');
const updateStatusText = document.getElementById('update-status-text');
const updateProgressPercent = document.getElementById('update-progress-percent');
const manifestViewer = document.getElementById('manifest-viewer');

btnCheckUpdate.addEventListener('click', async () => {
  const currentVer = 'v1.0.0';
  const targetVer = inputTargetVer.value.trim();

  try {
    updateStatusText.textContent = `SemVer 비교 중 (${currentVer} vs ${targetVer})...`;
    const isNewer = await invoke('check_update_version', {
      currentVer,
      remoteVer: targetVer
    });

    if (!isNewer) {
      updateStatusText.textContent = `현재 최신 버전입니다 (${currentVer} >= ${targetVer}).`;
      return;
    }

    btnCheckUpdate.disabled = true;
    updateStatusText.textContent = `새 버전(${targetVer}) 발견! 5단계 파이프라인 시작...`;

    // 백엔드 시뮬레이션 스트림 호출
    await invoke('simulate_update_stream', { targetVer });
  } catch (err) {
    alert('업데이트 검사 오류: ' + err);
    btnCheckUpdate.disabled = false;
  }
});

btnResetUpdate.addEventListener('click', () => {
  updateProgressBar.style.width = '0%';
  updateProgressPercent.textContent = '0%';
  updateStatusText.textContent = '대기 중 (업데이트 확인 버튼을 누르세요)';
  btnCheckUpdate.disabled = false;
  for (let i = 1; i <= 5; i++) {
    const el = document.getElementById(`step-${i}`);
    if (el) el.className = 'step-card';
  }
});

function handleUpdateProgress(step) {
  updateProgressBar.style.width = `${step.progress_percent}%`;
  updateProgressPercent.textContent = `${step.progress_percent}%`;
  updateStatusText.textContent = step.message;

  // 단계 카드 하이라이트
  for (let i = 1; i <= 5; i++) {
    const el = document.getElementById(`step-${i}`);
    if (!el) continue;
    if (i < step.step) {
      el.className = 'step-card done';
    } else if (i === step.step) {
      el.className = 'step-card active';
    } else {
      el.className = 'step-card';
    }
  }

  if (step.step === 5) {
    btnCheckUpdate.disabled = false;
  }
}

// ============================================================
// 5. Ed25519 암호화 서명 스튜디오
// ============================================================
const btnGenKeys = document.getElementById('btn-gen-keys');
const pubkeyDisplay = document.getElementById('pubkey-display');
const privkeyDisplay = document.getElementById('privkey-display');
const signPayloadInput = document.getElementById('sign-payload');
const btnSignData = document.getElementById('btn-sign-data');
const signResultBox = document.getElementById('sign-result-box');
const resSha256 = document.getElementById('res-sha256');
const resSig = document.getElementById('res-sig');

const verifyPubkey = document.getElementById('verify-pubkey');
const verifyPayload = document.getElementById('verify-payload');
const verifySig = document.getElementById('verify-sig');
const btnVerifySig = document.getElementById('btn-verify-sig');
const btnTamperTest = document.getElementById('btn-tamper-test');
const verifyBanner = document.getElementById('verify-banner');

btnGenKeys.addEventListener('click', async () => {
  try {
    const keys = await invoke('generate_keypair');
    pubkeyDisplay.value = keys.public_key_base64;
    privkeyDisplay.value = keys.private_key_base64;
    verifyPubkey.value = keys.public_key_base64;

    // latest.json 매니페스트 샘플 자동 생성
    const manifest = await invoke('generate_manifest', {
      version: '1.1.0',
      notes: 'v1.1.0 프로덕션 보안 업데이트 및 Ed25519 전자 서명 적용',
      privateKey: keys.private_key_base64
    });
    manifestViewer.textContent = JSON.stringify(manifest, null, 2);
  } catch (err) {
    alert('키 생성 실패: ' + err);
  }
});

btnSignData.addEventListener('click', async () => {
  const privKey = privkeyDisplay.value.trim();
  const payload = signPayloadInput.value.trim();
  if (!privKey) {
    alert('먼저 [새 키 쌍 생성] 버튼을 눌러 키를 생성하세요.');
    return;
  }

  try {
    const res = await invoke('sign_data', {
      privateKeyBase64: privKey,
      payload
    });

    resSha256.textContent = res.payload_sha256;
    resSig.textContent = res.signature_base64;
    signResultBox.style.display = 'flex';

    // 검증 폼으로 값 복사
    verifyPayload.value = payload;
    verifySig.value = res.signature_base64;
  } catch (err) {
    alert('서명 생성 실패: ' + err);
  }
});

btnVerifySig.addEventListener('click', async () => {
  const pubKey = verifyPubkey.value.trim();
  const payload = verifyPayload.value.trim();
  const sig = verifySig.value.trim();

  if (!pubKey || !sig) {
    alert('공개키와 서명 값을 입력하세요.');
    return;
  }

  try {
    const valid = await invoke('verify_data', {
      publicKeyBase64: pubKey,
      payload,
      signatureBase64: sig
    });

    if (valid) {
      verifyBanner.className = 'verify-status-banner success';
      verifyBanner.innerHTML = '✅ <strong>서명 검증 성공!</strong><br/>데이터가 변조되지 않았으며 신뢰할 수 있는 개발자의 개인키로 서명되었음이 100% 입증되었습니다.';
    } else {
      verifyBanner.className = 'verify-status-banner fail';
      verifyBanner.innerHTML = '❌ <strong>서명 검증 실패!</strong><br/>공개키와 서명이 일치하지 않거나 데이터가 중간에 조작되었습니다.';
    }
  } catch (err) {
    verifyBanner.className = 'verify-status-banner fail';
    verifyBanner.textContent = '검증 오류: ' + err;
  }
});

btnTamperTest.addEventListener('click', async () => {
  verifyPayload.value += '_TAMPERED';
  btnVerifySig.click();
});

// ============================================================
// 6. 이벤트 리스너 및 초기화
// ============================================================
async function setupEventListeners() {
  const listen = getTauriListen();
  if (!listen) return;

  try {
    await listen('update-step-progress', (event) => {
      handleUpdateProgress(event.payload);
    });
  } catch (err) {
    console.error('이벤트 리스너 등록 실패:', err);
  }
}

function initApp() {
  setupEventListeners();
  // 자동 초기 키 생성 트리거
  btnGenKeys.click();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initApp);
} else {
  initApp();
}


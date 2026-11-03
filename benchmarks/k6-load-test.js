import http from 'k6/http';
import { check, sleep } from 'k6';

// -------------------------------------------------------------
// 부하 테스트 시나리오 설정 (점진적 가상 유저 증가: Ramping VUs)
// -------------------------------------------------------------
export const options = {
  stages: [
    { duration: '10s', target: 50 },  // 10초 동안 가상 유저 0명 -> 50명으로 증가 (워밍업)
    { duration: '20s', target: 150 }, // 20초 동안 150명 동시 접속 유지 (피크 부하)
    { duration: '10s', target: 0 },   // 10초 동안 0명으로 쿨다운
  ],
  thresholds: {
    // 95%의 요청이 20ms 이내에 완료되어야 함 (고성능 목표치)
    http_req_duration: ['p(95)<20'],
    // 실패율(에러)이 0.1% 미만이어야 함
    http_req_failed: ['rate<0.001'],
  },
};

const BASE_URL = __ENV.TARGET_URL || 'http://127.0.0.1:3001';

export default function () {
  // 1. 전체 Todo 목록 조회 (GET /api/todos)
  const listRes = http.get(`${BASE_URL}/api/todos`);
  check(listRes, {
    'GET /api/todos 상태 코드 200': (r) => r.status === 200,
  });

  // 2. 신규 Todo 등록 (POST /api/todos)
  const payload = JSON.stringify({
    title: `부하 테스트 할 일 - VU:${__VU} - Iter:${__ITER}`,
    description: 'k6 고성능 부하 테스트 자동 생성 데이터',
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  const createRes = http.post(`${BASE_URL}/api/todos`, payload, params);
  check(createRes, {
    'POST /api/todos 상태 코드 201': (r) => r.status === 201,
  });

  // 가상 사용자의 자연스러운 요청 간격 (0.05초 대기)
  sleep(0.05);
}

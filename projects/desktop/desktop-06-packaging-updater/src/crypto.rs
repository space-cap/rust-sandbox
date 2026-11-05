//! Ed25519 비대칭 암호화 서명 및 SHA-256 무결성 검증 엔진
use crate::models::{KeyPairResult, SignResult};
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

/// 신규 Ed25519 키 쌍 (공개키, 비밀키) 생성
pub fn generate_ed25519_keypair() -> KeyPairResult {
    let mut rng = rand::rngs::OsRng;
    let mut secret_bytes = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rng, &mut secret_bytes);

    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    KeyPairResult {
        public_key_base64: BASE64_STANDARD.encode(verifying_key.to_bytes()),
        private_key_base64: BASE64_STANDARD.encode(signing_key.to_bytes()),
    }
}

/// 데이터 페이로드(문자열/바이너리)에 대해 Ed25519 디지털 서명 생성
pub fn sign_payload(private_key_base64: &str, payload: &str) -> Result<SignResult, String> {
    let key_bytes = BASE64_STANDARD
        .decode(private_key_base64.trim())
        .map_err(|e| format!("비밀키 Base64 디코딩 실패: {}", e))?;

    if key_bytes.len() != 32 {
        return Err(format!("비밀키 길이는 32바이트여야 합니다 (현재: {}바이트)", key_bytes.len()));
    }

    let mut secret_array = [0u8; 32];
    secret_array.copy_from_slice(&key_bytes);

    let signing_key = SigningKey::from_bytes(&secret_array);
    let verifying_key = signing_key.verifying_key();

    let payload_bytes = payload.as_bytes();
    let signature: Signature = signing_key.sign(payload_bytes);

    // 자체 검증 수행
    let verified = verifying_key.verify(payload_bytes, &signature).is_ok();

    // SHA-256 해시 계산
    let hash = calculate_sha256(payload_bytes);

    Ok(SignResult {
        payload_sha256: hash,
        signature_base64: BASE64_STANDARD.encode(signature.to_bytes()),
        verified,
    })
}

/// 공개키를 활용한 서명 무결성 검증
pub fn verify_signature(
    public_key_base64: &str,
    payload: &str,
    signature_base64: &str,
) -> Result<bool, String> {
    let pub_bytes = BASE64_STANDARD
        .decode(public_key_base64.trim())
        .map_err(|e| format!("공개키 Base64 디코딩 실패: {}", e))?;

    if pub_bytes.len() != 32 {
        return Err(format!("공개키 길이는 32바이트여야 합니다 (현재: {}바이트)", pub_bytes.len()));
    }

    let mut pub_array = [0u8; 32];
    pub_array.copy_from_slice(&pub_bytes);

    let verifying_key = VerifyingKey::from_bytes(&pub_array)
        .map_err(|e| format!("Ed25519 공개키 파싱 실패: {}", e))?;

    let sig_bytes = BASE64_STANDARD
        .decode(signature_base64.trim())
        .map_err(|e| format!("서명 Base64 디코딩 실패: {}", e))?;

    if sig_bytes.len() != 64 {
        return Err(format!("Ed25519 서명 길이는 64바이트여야 합니다 (현재: {}바이트)", sig_bytes.len()));
    }

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&sig_bytes);

    let signature = Signature::from_bytes(&sig_array);

    Ok(verifying_key.verify(payload.as_bytes(), &signature).is_ok())
}

/// SHA-256 64자리 Hex 문자열 계산
pub fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}


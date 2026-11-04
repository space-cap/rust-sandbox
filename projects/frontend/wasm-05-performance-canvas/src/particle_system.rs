//! 고성능 파티클 물리 연산 엔진 (순수 Rust 단위 테스트 가능)
use web_sys::CanvasRenderingContext2d;

/// 경량 Xorshift 의사 난수 생성기 (OS 난수 의존성 없이 Wasm에서 초고속 동작)
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x & 0xFFFF_FFFF) as u32
    }

    /// [0.0, 1.0) 범위의 f64 난수 반환
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u32() as f64) / 4294967296.0
    }

    /// [min, max] 범위의 f64 난수 반환
    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }
}

/// 마우스/터치 인터랙션 모드
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionMode {
    Attract,  // 중력 인력 (마우스 방향으로 끌어당김)
    Repel,    // 반발력 (마우스로부터 밀어냄)
    Vortex,   // 소용돌이 (마우스 주위를 나선형으로 회전)
    Floating, // 부유 모드 (상호작용 없이 경계면 반사)
}

impl InteractionMode {
    pub fn label(&self) -> &'static str {
        match self {
            InteractionMode::Attract => "🧲 인력 (Attract)",
            InteractionMode::Repel => "💥 척력 (Repel)",
            InteractionMode::Vortex => "🌀 소용돌이 (Vortex)",
            InteractionMode::Floating => "✨ 유영 (Float)",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub hue: f64,
}

#[derive(Debug, PartialEq)]
pub struct ParticleUniverse {
    pub width: f64,
    pub height: f64,
    pub particles: Vec<Particle>,
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub is_hovering: bool,
    pub mode: InteractionMode,
    pub connect_lines: bool,
    pub speed_multiplier: f64,
    rng: SimpleRng,
}

impl ParticleUniverse {
    pub fn new(width: f64, height: f64, count: usize) -> Self {
        let mut rng = SimpleRng::new(123456789);
        let mut particles = Vec::with_capacity(count);

        for _ in 0..count {
            particles.push(Self::create_random_particle(&mut rng, width, height));
        }

        Self {
            width,
            height,
            particles,
            cursor_x: width / 2.0,
            cursor_y: height / 2.0,
            is_hovering: false,
            mode: InteractionMode::Attract,
            connect_lines: true,
            speed_multiplier: 1.0,
            rng,
        }
    }

    fn create_random_particle(rng: &mut SimpleRng, w: f64, h: f64) -> Particle {
        let speed = rng.range_f64(0.5, 2.0);
        let angle = rng.range_f64(0.0, std::f64::consts::PI * 2.0);

        Particle {
            x: rng.range_f64(0.0, w),
            y: rng.range_f64(0.0, h),
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            radius: rng.range_f64(1.5, 3.5),
            hue: rng.range_f64(170.0, 280.0), // 에메랄드 ~ 사이언 ~ 퍼플 색상
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn set_cursor(&mut self, x: f64, y: f64, is_hovering: bool) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.is_hovering = is_hovering;
    }

    pub fn set_mode(&mut self, mode: InteractionMode) {
        self.mode = mode;
    }

    pub fn set_connect_lines(&mut self, enabled: bool) {
        self.connect_lines = enabled;
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed_multiplier = speed.clamp(0.1, 5.0);
    }

    pub fn set_particle_count(&mut self, new_count: usize) {
        let current_count = self.particles.len();
        if new_count > current_count {
            for _ in current_count..new_count {
                let p = Self::create_random_particle(&mut self.rng, self.width, self.height);
                self.particles.push(p);
            }
        } else if new_count < current_count {
            self.particles.truncate(new_count);
        }
    }

    /// 마우스 클릭 시 폭발 효과(Burst)
    pub fn burst(&mut self, cx: f64, cy: f64, burst_count: usize) {
        for _ in 0..burst_count {
            let speed = self.rng.range_f64(3.0, 9.0);
            let angle = self.rng.range_f64(0.0, std::f64::consts::PI * 2.0);

            self.particles.push(Particle {
                x: cx,
                y: cy,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                radius: self.rng.range_f64(2.0, 4.5),
                hue: self.rng.range_f64(40.0, 70.0), // 폭발 시 눈부신 골드/앰버 색상
            });
        }
        // 파티클 수가 과도하게 늘지 않도록 5,000개 이내로 제한
        if self.particles.len() > 5000 {
            let excess = self.particles.len() - 5000;
            self.particles.drain(0..excess);
        }
    }

    /// 1프레임 물리 시뮬레이션 계산
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;
        let cx = self.cursor_x;
        let cy = self.cursor_y;
        let hovering = self.is_hovering;
        let mode = self.mode;
        let speed = self.speed_multiplier;

        for p in &mut self.particles {
            // 1. 마우스 상호작용 힘 계산
            if hovering && mode != InteractionMode::Floating {
                let dx = cx - p.x;
                let dy = cy - p.y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt().max(1.0);

                if dist < 300.0 {
                    let force = (1.0 - dist / 300.0) * 0.45;
                    match mode {
                        InteractionMode::Attract => {
                            p.vx += (dx / dist) * force * 1.5;
                            p.vy += (dy / dist) * force * 1.5;
                        }
                        InteractionMode::Repel => {
                            p.vx -= (dx / dist) * force * 2.5;
                            p.vy -= (dy / dist) * force * 2.5;
                        }
                        InteractionMode::Vortex => {
                            // 접선 방향(Tangent) 벡터로 회전력 부여
                            p.vx += (-dy / dist) * force * 2.0 + (dx / dist) * 0.2;
                            p.vy += (dx / dist) * force * 2.0 + (dy / dist) * 0.2;
                        }
                        InteractionMode::Floating => {}
                    }
                }
            }

            // 2. 속도 마찰(Damping) 및 최대 속도 제한
            p.vx *= 0.985;
            p.vy *= 0.985;

            let max_v = 12.0;
            let current_v = (p.vx * p.vx + p.vy * p.vy).sqrt();
            if current_v > max_v {
                p.vx = (p.vx / current_v) * max_v;
                p.vy = (p.vy / current_v) * max_v;
            }

            // 3. 위치 갱신
            p.x += p.vx * speed;
            p.y += p.vy * speed;

            // 4. 벽면 탄성 반사
            if p.x <= p.radius {
                p.x = p.radius;
                p.vx = -p.vx * 0.9;
            } else if p.x >= w - p.radius {
                p.x = w - p.radius;
                p.vx = -p.vx * 0.9;
            }

            if p.y <= p.radius {
                p.y = p.radius;
                p.vy = -p.vy * 0.9;
            } else if p.y >= h - p.radius {
                p.y = h - p.radius;
                p.vy = -p.vy * 0.9;
            }
        }
    }

    /// Canvas 2D 컨텍스트에 렌더링
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        let w = self.width;
        let h = self.height;

        // 잔상(Motion Blur) 효과를 위해 약간 투명한 검정색으로 덮기
        ctx.set_fill_style_str("rgba(7, 10, 18, 0.28)");
        ctx.fill_rect(0.0, 0.0, w, h);

        // 1. 근접 파티클 간 연결 선(Constellation) 그리기 (성능 최적화를 위해 일부만 샘플링)
        if self.connect_lines && self.particles.len() <= 2000 {
            ctx.set_line_width(0.65);
            let threshold = 65.0;
            let threshold_sq = threshold * threshold;

            let len = self.particles.len();
            for i in 0..len {
                let p1 = &self.particles[i];
                for j in (i + 1)..len {
                    let p2 = &self.particles[j];
                    let dx = p1.x - p2.x;
                    let dy = p1.y - p2.y;
                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq < threshold_sq {
                        let alpha = 1.0 - (dist_sq / threshold_sq);
                        let stroke_color = format!("rgba(56, 189, 248, {alpha:.2})");
                        ctx.set_stroke_style_str(&stroke_color);
                        ctx.begin_path();
                        ctx.move_to(p1.x, p1.y);
                        ctx.line_to(p2.x, p2.y);
                        ctx.stroke();
                    }
                }
            }
        }

        // 2. 파티클 본체 렌더링
        for p in &self.particles {
            let color = format!("hsl({:.0}, 90%, 65%)", p.hue);
            ctx.set_fill_style_str(&color);
            ctx.begin_path();
            let _ = ctx.arc(p.x, p.y, p.radius, 0.0, std::f64::consts::PI * 2.0);
            ctx.fill();
        }

        // 3. 마우스 커서 위치 주변 인터랙션 링 표시
        if self.is_hovering && self.mode != InteractionMode::Floating {
            ctx.begin_path();
            let _ = ctx.arc(self.cursor_x, self.cursor_y, 35.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.set_stroke_style_str("rgba(255, 255, 255, 0.35)");
            ctx.set_line_width(1.5);
            ctx.stroke();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_initialization() {
        let universe = ParticleUniverse::new(800.0, 600.0, 200);
        assert_eq!(universe.particles.len(), 200);
        assert_eq!(universe.width, 800.0);
        assert_eq!(universe.height, 600.0);

        for p in &universe.particles {
            assert!(p.x >= 0.0 && p.x <= 800.0);
            assert!(p.y >= 0.0 && p.y <= 600.0);
        }
    }

    #[test]
    fn test_resize_and_particle_count() {
        let mut universe = ParticleUniverse::new(800.0, 600.0, 100);
        universe.resize(1024.0, 768.0);
        assert_eq!(universe.width, 1024.0);
        assert_eq!(universe.height, 768.0);

        universe.set_particle_count(250);
        assert_eq!(universe.particles.len(), 250);

        universe.set_particle_count(50);
        assert_eq!(universe.particles.len(), 50);
    }

    #[test]
    fn test_step_motion_and_boundary_bounce() {
        let mut universe = ParticleUniverse::new(500.0, 500.0, 1);
        universe.particles[0] = Particle {
            x: 495.0,
            y: 250.0,
            vx: 10.0, // 오른쪽 벽면을 향해 이동
            vy: 0.0,
            radius: 2.0,
            hue: 200.0,
        };

        universe.step();

        // 벽에 부딪쳐서 반사되어 x 속도가 음수가 됨
        assert!(universe.particles[0].vx < 0.0);
        assert!(universe.particles[0].x <= 500.0);
    }

    #[test]
    fn test_attraction_force() {
        let mut universe = ParticleUniverse::new(1000.0, 1000.0, 1);
        universe.particles[0] = Particle {
            x: 500.0,
            y: 500.0,
            vx: 0.0,
            vy: 0.0,
            radius: 2.0,
            hue: 200.0,
        };

        // 커서를 파티클 우측(600, 500)에 배치하고 인력(Attract) 적용
        universe.set_cursor(600.0, 500.0, true);
        universe.set_mode(InteractionMode::Attract);
        universe.step();

        // 파티클이 커서 방향(우측: +x)으로 당겨졌으므로 vx > 0
        assert!(universe.particles[0].vx > 0.0);
    }
}

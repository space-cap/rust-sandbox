use leptos::prelude::*;
use crate::models::calculate_progress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformView {
    GitHubPages,
    Vercel,
    Netlify,
    DockerNginx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecklistItem {
    pub id: usize,
    pub title: &'static str,
    pub desc: &'static str,
    pub code: Option<&'static str>,
    pub checked: bool,
}

#[component]
pub fn DeploymentTab() -> impl IntoView {
    let initial_checklist = vec![
        ChecklistItem {
            id: 0,
            title: "Trunk 릴리즈 빌드 최적화",
            desc: "디버그 심볼을 제거하고 Rust 컴파일러의 LTO 및 최적화 레벨을 활성화합니다.",
            code: Some("trunk build --release"),
            checked: true,
        },
        ChecklistItem {
            id: 1,
            title: "Wasm 바이너리 추가 압축 (wasm-opt)",
            desc: "Binaryen 도구로 Wasm 바이트코드를 재분석하여 파일 용량을 40~60% 추가 감축합니다.",
            code: Some("wasm-opt -O4 -o dist/app_opt.wasm dist/app.wasm"),
            checked: true,
        },
        ChecklistItem {
            id: 2,
            title: "SPA 404 라우팅 새로고침 트릭",
            desc: "정적 호스팅 서버에서 하위 경로 새로고침 시 404 에러를 방지하도록 index.html을 복사합니다.",
            code: Some("cp dist/index.html dist/404.html"),
            checked: false,
        },
        ChecklistItem {
            id: 3,
            title: "GitHub Pages .nojekyll 플래그",
            desc: "GitHub Pages가 _trunk 등의 디렉터리를 Jekyll 특수 폴더로 오인하여 무시하는 것을 방지합니다.",
            code: Some("touch dist/.nojekyll"),
            checked: false,
        },
        ChecklistItem {
            id: 4,
            title: "올바른 MIME 타입 및 캐싱 헤더",
            desc: "웹 서버가 .wasm 파일을 반드시 'application/wasm' Content-Type 헤더로 서빙하도록 설정합니다.",
            code: Some("Content-Type: application/wasm\nCache-Control: public, max-age=31536000, immutable"),
            checked: false,
        },
    ];

    let (checklist, set_checklist) = signal(initial_checklist);
    let (platform, set_platform) = signal(PlatformView::GitHubPages);
    let (copied_text, set_copied_text) = signal(Option::<&'static str>::None);

    let progress = Memo::new(move |_| {
        let list = checklist.get();
        let checked_cnt = list.iter().filter(|it| it.checked).count();
        calculate_progress(checked_cnt, list.len())
    });

    let toggle_check = move |id: usize| {
        set_checklist.update(|list| {
            if let Some(item) = list.iter_mut().find(|it| it.id == id) {
                item.checked = !item.checked;
            }
        });
    };

    let copy_to_clipboard = move |content: &'static str| {
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            let clipboard = nav.clipboard();
            let _ = clipboard.write_text(content);
            set_copied_text.set(Some(content));
        }
    };

    // 설정 파일 템플릿 코드
    let gh_actions_yaml = r#"name: Deploy Leptos Wasm to GitHub Pages

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Download & Install Trunk
        run: |
          wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.22.1/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin/

      - name: Build with Trunk (Release)
        run: |
          cd projects/frontend/wasm-06-leptos-showcase
          trunk build --release
          cp dist/index.html dist/404.html
          touch dist/.nojekyll

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: projects/frontend/wasm-06-leptos-showcase/dist
"#;

    let vercel_json = r#"{
  "buildCommand": "trunk build --release",
  "outputDirectory": "dist",
  "rewrites": [
    { "source": "/(.*)", "destination": "/index.html" }
  ],
  "headers": [
    {
      "source": "/(.*).wasm",
      "headers": [
        { "key": "Content-Type", "value": "application/wasm" },
        { "key": "Cache-Control", "value": "public, max-age=31536000, immutable" }
      ]
    }
  ]
}
"#;

    let netlify_toml = r#"[build]
  command = "trunk build --release"
  publish = "dist"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200

[[headers]]
  for = "/*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
    Cache-Control = "public, max-age=31536000, immutable"
"#;

    let dockerfile_nginx = r#"# Multi-stage Dockerfile for Leptos Wasm
FROM rust:1.80-slim as builder
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y wget && \
    wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.22.1/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin/

COPY . .
WORKDIR /app/projects/frontend/wasm-06-leptos-showcase
RUN trunk build --release

FROM nginx:alpine
COPY --from=builder /app/projects/frontend/wasm-06-leptos-showcase/dist /usr/share/nginx/html
# Nginx mime.types includes application/wasm by default
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
"#;

    view! {
        <div class="tab-content deployment-tab">
            // 상단 배포 준비도 게이지
            <section class="card progress-card">
                <div class="card-header">
                    <h2>"🚀 프로덕션 정적 배포 준비도 (Readiness Score)"</h2>
                    <span class="badge-status">
                        {move || {
                            let (checked, total, pct) = progress.get();
                            format!("{}/{} 완료 ({}%)", checked, total, pct)
                        }}
                    </span>
                </div>

                <div class="progress-bar-wrapper">
                    <div
                        class="progress-bar-fill"
                        style=move || format!("width: {}%;", progress.get().2)
                    ></div>
                </div>

                <p class="progress-notice">
                    {move || {
                        let (_, _, pct) = progress.get();
                        if pct == 100 {
                            "🎉 축하합니다! 모든 프로덕션 배포 요구사항을 완벽하게 만족했습니다!"
                        } else if pct >= 60 {
                            "⚡ 정적 호스팅 배포가 가능한 수준입니다. 남은 최적화 항목을 확인하세요."
                        } else {
                            "📌 체크리스트 항목을 확인하여 배포 준비도를 높여보세요."
                        }
                    }}
                </p>
            </section>

            <div class="grid-2-col">
                // 인터랙티브 체크리스트
                <section class="card checklist-card">
                    <div class="card-header">
                        <h3>"📋 배포 필수 체크리스트"</h3>
                        <span class="badge-mini">"Interactive Checklist"</span>
                    </div>

                    <div class="checklist-items">
                        <For
                            each=move || checklist.get()
                            key=|it| it.id
                            let:item
                        >
                            {
                                let id = item.id;
                                let is_checked = item.checked;
                                view! {
                                    <div
                                        class=if is_checked { "checklist-row checked" } else { "checklist-row" }
                                        on:click=move |_| toggle_check(id)
                                    >
                                        <div class="check-box">
                                            {if is_checked { "✓" } else { "" }}
                                        </div>
                                        <div class="check-content">
                                            <h4 class="check-title">{item.title}</h4>
                                            <p class="check-desc">{item.desc}</p>
                                            {item.code.map(|c| {
                                                view! {
                                                    <code class="check-code">{c}</code>
                                                }
                                            })}
                                        </div>
                                    </div>
                                }
                            }
                        </For>
                    </div>
                </section>

                // 플랫폼별 설정 파일 뷰어
                <section class="card config-card">
                    <div class="card-header">
                        <h3>"⚙️ 플랫폼별 설정 파일 & CI/CD"</h3>
                        <div class="platform-chips">
                            <button
                                class=move || if platform.get() == PlatformView::GitHubPages { "p-chip active" } else { "p-chip" }
                                on:click=move |_| set_platform.set(PlatformView::GitHubPages)
                            >
                                "GitHub Pages"
                            </button>
                            <button
                                class=move || if platform.get() == PlatformView::Vercel { "p-chip active" } else { "p-chip" }
                                on:click=move |_| set_platform.set(PlatformView::Vercel)
                            >
                                "Vercel"
                            </button>
                            <button
                                class=move || if platform.get() == PlatformView::Netlify { "p-chip active" } else { "p-chip" }
                                on:click=move |_| set_platform.set(PlatformView::Netlify)
                            >
                                "Netlify"
                            </button>
                            <button
                                class=move || if platform.get() == PlatformView::DockerNginx { "p-chip active" } else { "p-chip" }
                                on:click=move |_| set_platform.set(PlatformView::DockerNginx)
                            >
                                "Docker / Nginx"
                            </button>
                        </div>
                    </div>

                    <div class="code-viewer">
                        <div class="code-viewer-header">
                            <span class="file-name">
                                {move || match platform.get() {
                                    PlatformView::GitHubPages => ".github/workflows/deploy.yml",
                                    PlatformView::Vercel => "vercel.json",
                                    PlatformView::Netlify => "netlify.toml",
                                    PlatformView::DockerNginx => "Dockerfile (Multi-stage)",
                                }}
                            </span>
                            <button
                                class="btn-copy"
                                on:click=move |_| {
                                    let content = match platform.get() {
                                        PlatformView::GitHubPages => gh_actions_yaml,
                                        PlatformView::Vercel => vercel_json,
                                        PlatformView::Netlify => netlify_toml,
                                        PlatformView::DockerNginx => dockerfile_nginx,
                                    };
                                    copy_to_clipboard(content);
                                }
                            >
                                {move || {
                                    if copied_text.get().is_some() {
                                        "✓ 클립보드 복사됨!"
                                    } else {
                                        "📋 코드 복사"
                                    }
                                }}
                            </button>
                        </div>

                        <pre class="code-block">
                            <code>
                                {move || match platform.get() {
                                    PlatformView::GitHubPages => gh_actions_yaml,
                                    PlatformView::Vercel => vercel_json,
                                    PlatformView::Netlify => netlify_toml,
                                    PlatformView::DockerNginx => dockerfile_nginx,
                                }}
                            </code>
                        </pre>
                    </div>
                </section>
            </div>
        </div>
    }
}


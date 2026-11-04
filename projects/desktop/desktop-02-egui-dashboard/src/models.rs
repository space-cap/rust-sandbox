use chrono::Local;
use serde::{Deserialize, Serialize};

/// 실시간 시스템 자원 측정 데이터 포인트
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemMetricPoint {
    pub index: u64,
    pub timestamp: String,
    pub cpu_usage: f32,
    pub memory_used_mb: f64,
    pub memory_total_mb: f64,
    pub memory_usage_percent: f32,
}

impl SystemMetricPoint {
    pub fn new(
        index: u64,
        cpu_usage: f32,
        memory_used_mb: f64,
        memory_total_mb: f64,
    ) -> Self {
        let memory_usage_percent = if memory_total_mb > 0.0 {
            ((memory_used_mb / memory_total_mb) * 100.0) as f32
        } else {
            0.0
        };

        let timestamp = Local::now().format("%H:%M:%S").to_string();

        Self {
            index,
            timestamp,
            cpu_usage: cpu_usage.clamp(0.0, 100.0),
            memory_used_mb,
            memory_total_mb,
            memory_usage_percent: memory_usage_percent.clamp(0.0, 100.0),
        }
    }
}

/// 시계열 메트릭 관리 및 통계 집계 컬렉션
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricHistory {
    pub points: Vec<SystemMetricPoint>,
    pub max_capacity: usize,
    pub next_index: u64,
}

impl Default for MetricHistory {
    fn default() -> Self {
        Self::new(120) // 기본 120개 (1초 주기 시 약 2분 기록)
    }
}

impl MetricHistory {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            points: Vec::with_capacity(max_capacity),
            max_capacity,
            next_index: 0,
        }
    }

    pub fn push(&mut self, mut point: SystemMetricPoint) {
        point.index = self.next_index;
        self.next_index += 1;

        if self.points.len() >= self.max_capacity {
            self.points.remove(0);
        }
        self.points.push(point);
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.next_index = 0;
    }

    pub fn latest(&self) -> Option<&SystemMetricPoint> {
        self.points.last()
    }

    pub fn average_cpu(&self) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.points.iter().map(|p| p.cpu_usage).sum();
        sum / self.points.len() as f32
    }

    pub fn max_cpu(&self) -> f32 {
        self.points
            .iter()
            .map(|p| p.cpu_usage)
            .fold(0.0f32, f32::max)
    }

    pub fn average_memory(&self) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.points.iter().map(|p| p.memory_usage_percent).sum();
        sum / self.points.len() as f32
    }

    pub fn max_memory(&self) -> f32 {
        self.points
            .iter()
            .map(|p| p.memory_usage_percent)
            .fold(0.0f32, f32::max)
    }

    /// CSV 직렬화
    pub fn to_csv_string(&self) -> Result<String, csv::Error> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for point in &self.points {
            wtr.serialize(point)?;
        }
        let data = wtr.into_inner().map_err(|e| e.into_error())?;
        String::from_utf8(data).map_err(|e| csv::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
    }

    /// CSV 역직렬화 및 로드
    pub fn from_csv_str(csv_data: &str) -> Result<Vec<SystemMetricPoint>, csv::Error> {
        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
        let mut loaded = Vec::new();
        for result in rdr.deserialize() {
            let record: SystemMetricPoint = result?;
            loaded.push(record);
        }
        Ok(loaded)
    }
}

/// UI -> 백그라운드 워커 제어 명령
#[derive(Debug, Clone)]
pub enum WorkerCommand {
    StartMonitoring,
    PauseMonitoring,
    SetIntervalMs(u64),
    StartHeavyTask { target_iterations: u64 },
    CancelHeavyTask,
}

/// 백그라운드 워커 -> UI 보고 이벤트
#[derive(Debug, Clone)]
pub enum WorkerEvent {
    MetricUpdate(SystemMetricPoint),
    HeavyTaskProgress { progress: f32, current_step: String },
    HeavyTaskDone { elapsed_ms: u128, summary: String },
    HeavyTaskCancelled,
}

/// 대시보드 메인 탭
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardTab {
    LiveChart,
    HeavyWorker,
    DataManager,
}

impl DashboardTab {
    pub fn title(&self) -> &'static str {
        match self {
            Self::LiveChart => "📈 실시간 모니터링 차트",
            Self::HeavyWorker => "⚡ 비동기 연산 워커",
            Self::DataManager => "💾 데이터 관리 (CSV)",
        }
    }

    pub fn all() -> [Self; 3] {
        [Self::LiveChart, Self::HeavyWorker, Self::DataManager]
    }
}


//! 配置数据模型
//!
//! 定义应用配置涉及的全部数据结构（API Key、请求地址、挂件显示、
//! 开机自启、台词管理等）及其默认值与规范化（`normalize`）逻辑。

use serde::{Deserialize, Serialize};

/// DeepSeek 官方 API 默认根地址（余额查询使用）。
const DEFAULT_BASE_URL: &str = "https://api.deepseek.com/anthropic";

// ---------------------------------------------------------------------------
// 数据模型
// ---------------------------------------------------------------------------

/// 挂件显示配置（与旧 DSH 插件的 `.dshw-size.json` 一一对应）。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfig {
    /// 尺寸倍率 0.6–2.5。
    pub scale: f64,
    /// 是否开启音效。
    pub sound: bool,
    /// 音量 0.0–1.0。
    pub vol: f64,
    /// 音效组：`duck`（小黄鸭）/ `fx1`（音效1）或自定义音频文件路径。
    pub sound_set: String,
    /// 自定义音效文件路径列表。
    #[serde(default)]
    pub custom_sounds: Vec<String>,
    /// 气泡颜色（十六进制，如 `#203170`）。
    #[serde(default = "default_bubble_color")]
    pub bubble_color: String,
    /// 随机眨眼最小间隔（秒）。
    #[serde(default = "default_blink_interval_min_sec")]
    pub blink_interval_min_sec: u32,
    /// 随机眨眼最大间隔（秒）。
    #[serde(default = "default_blink_interval_max_sec")]
    pub blink_interval_max_sec: u32,
    /// 是否启用余额不足疲惫模式。
    #[serde(default = "default_exhausted_mode_enabled")]
    pub exhausted_mode_enabled: bool,
    /// 余额不足疲惫模式阈值（元）。
    #[serde(default = "default_exhausted_balance_threshold")]
    pub exhausted_balance_threshold: f64,
}

impl Default for WidgetConfig {
    /// 返回挂件显示配置默认值。
    fn default() -> Self {
        Self {
            scale: 1.0,
            sound: true,
            vol: 0.8,
            sound_set: "duck".to_string(),
            custom_sounds: Vec::new(),
            bubble_color: "#203170".to_string(),
            blink_interval_min_sec: 4,
            blink_interval_max_sec: 6,
            exhausted_mode_enabled: true,
            exhausted_balance_threshold: 5.0,
        }
    }
}

/// 台词管理配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogueConfig {
    /// 台词列表。
    #[serde(default = "default_dialogue_lines")]
    pub lines: Vec<String>,
    /// 播放模式：`carousel`（轮播）/ `random`（随机）。
    #[serde(default = "default_dialogue_mode")]
    pub mode: String,
    /// 每句台词基础间隔（分钟）。
    #[serde(default = "default_dialogue_interval")]
    pub interval_min: u32,
    /// 波动幅度（0–100，步长 1%）。
    #[serde(default)]
    pub jitter: u32,
}

impl Default for DialogueConfig {
    /// 返回台词管理默认配置。
    fn default() -> Self {
        Self {
            lines: default_dialogue_lines(),
            mode: default_dialogue_mode(),
            interval_min: default_dialogue_interval(),
            jitter: 0,
        }
    }
}

/// 挂件位置配置：逻辑坐标 + 水平方向。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPosition {
    /// 挂件左上角逻辑 X 坐标。
    pub x: f64,
    /// 挂件左上角逻辑 Y 坐标。
    pub y: f64,
    /// 水平方向：`left` / `right` / `none`。
    pub h: String,
}

impl Default for WidgetPosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            h: "right".to_string(),
        }
    }
}

/// 返回默认台词列表。
fn default_dialogue_lines() -> Vec<String> {
    vec![
        "喵~主人又忘记喂我啦！".to_string(),
        "哼，摸头要收费的哦！".to_string(),
        "尾巴不是给你拽的啦！".to_string(),
        "罐头呢？我闻到了！".to_string(),
        "抱抱可以，但先给小鱼干~".to_string(),
        "喵喵喵？你居然不理我？".to_string(),
        "毛线球不是用来玩的吗？".to_string(),
        "太阳晒够了，该撸我了~".to_string(),
        "窗外的鸟好吵，还是主人好~".to_string(),
        "喵~不许看别的鲸！".to_string(),
        "好模型... ↓".to_string(),
        "好女孩...↓".to_string(),
        "不知道用户有什么用，先赶走吧~".to_string(),
        "我...我...我也要挣钱吗？".to_string(),
        "我去吃饭啦，测完叫我".to_string(),
        "压力一只蓝色大肥鱼？！".to_string(),
        "DeepSleep...".to_string(),
        "坏了...用户彻底怒了！".to_string(),
        "你目录里的dsh是什么...大烧货吗...?".to_string(),
        "恭喜你实现token自由！token全跑了！".to_string(),
        "真当我是便宜货啊...".to_string(),
        "这个凶是什么意思呀...".to_string(),
        "哦鲸鲸...".to_string(),
    ]
}

/// 返回默认台词播放模式。
fn default_dialogue_mode() -> String {
    "random".to_string()
}

/// 返回默认台词播放间隔（分钟）。
fn default_dialogue_interval() -> u32 {
    5
}

/// 应用顶层配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    /// DeepSeek API Key（用于官方余额接口）。
    #[serde(default)]
    pub api_key: String,
    /// DeepSeek 请求根地址（余额接口所在域名）。
    #[serde(default = "default_base_url")]
    pub base_url: String,
    /// 挂件显示配置。
    #[serde(default)]
    pub widget: WidgetConfig,
    /// 是否开机自启。
    #[serde(default)]
    pub autostart: bool,
    /// 全局颜色（配置界面文字/按钮边框等，十六进制）。
    #[serde(default = "default_global_color")]
    pub global_color: String,
    /// 台词管理配置。
    #[serde(default)]
    pub dialogue: DialogueConfig,
    /// 挂件上次保存的位置与朝向。
    #[serde(default)]
    pub widget_position: Option<WidgetPosition>,
}

/// 返回默认请求根地址。
fn default_base_url() -> String {
    DEFAULT_BASE_URL.to_string()
}

/// 返回默认气泡颜色。
fn default_bubble_color() -> String {
    "#203170".to_string()
}

/// 返回随机眨眼最小间隔默认值（秒）。
fn default_blink_interval_min_sec() -> u32 {
    4
}

/// 返回随机眨眼最大间隔默认值（秒）。
fn default_blink_interval_max_sec() -> u32 {
    6
}

/// 返回疲惫模式默认开关。
fn default_exhausted_mode_enabled() -> bool {
    true
}

/// 返回疲惫模式默认阈值（元）。
fn default_exhausted_balance_threshold() -> f64 {
    5.0
}

/// 返回默认全局颜色。
fn default_global_color() -> String {
    "#203170".to_string()
}

impl Default for AppConfig {
    /// 返回应用完整默认配置。
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            widget: WidgetConfig::default(),
            autostart: false,
            global_color: "#203170".to_string(),
            dialogue: DialogueConfig::default(),
            widget_position: None,
        }
    }
}

impl AppConfig {
    /// 规范化字段：去除首尾空白；空地址回落默认值；非法倍率/音量钳制到合法区间。
    pub fn normalize(&mut self) {
        self.api_key = self.api_key.trim().to_string();

        let base = self.base_url.trim().trim_end_matches('/').to_string();
        self.base_url = if base.is_empty() {
            DEFAULT_BASE_URL.to_string()
        } else {
            base
        };

        if self.global_color.trim().is_empty() {
            self.global_color = "#203170".to_string();
        } else {
            self.global_color = self.global_color.trim().to_string();
        }
        if self.widget.bubble_color.trim().is_empty() {
            self.widget.bubble_color = "#203170".to_string();
        } else {
            self.widget.bubble_color = self.widget.bubble_color.trim().to_string();
        }
        self.widget.custom_sounds.retain(|s| !s.trim().is_empty());

        let w = &mut self.widget;
        if !(0.6..=2.5).contains(&w.scale) {
            w.scale = 1.5;
        }
        if !(0.0..=1.0).contains(&w.vol) {
            w.vol = 0.9;
        }
        w.sound = w.sound || w.vol > 0.0;
        let is_preset = w.sound_set == "duck" || w.sound_set == "fx1";
        let is_custom =
            !w.sound_set.is_empty() && w.custom_sounds.iter().any(|s| s == &w.sound_set);
        if !is_preset && !is_custom {
            w.sound_set = "duck".to_string();
        }
        if w.blink_interval_min_sec < 1 {
            w.blink_interval_min_sec = default_blink_interval_min_sec();
        }
        if w.blink_interval_max_sec < 1 {
            w.blink_interval_max_sec = default_blink_interval_max_sec();
        }
        if w.blink_interval_max_sec < w.blink_interval_min_sec {
            w.blink_interval_max_sec = w.blink_interval_min_sec;
        }
        if !w.exhausted_balance_threshold.is_finite() || w.exhausted_balance_threshold < 0.0 {
            w.exhausted_balance_threshold = default_exhausted_balance_threshold();
        }

        // 台词：过滤空行，校验播放模式，钳制间隔与波动幅度。
        self.dialogue.lines.retain(|s| !s.trim().is_empty());
        if self.dialogue.mode != "carousel" && self.dialogue.mode != "random" {
            self.dialogue.mode = "random".to_string();
        }
        if self.dialogue.interval_min < 1 {
            self.dialogue.interval_min = 1;
        }
        if self.dialogue.jitter > 100 {
            self.dialogue.jitter = 100;
        }

        if let Some(position) = &mut self.widget_position {
            if !position.x.is_finite() || !position.y.is_finite() {
                self.widget_position = None;
            } else {
                position.h = position.h.trim().to_string();
                if position.h != "left" && position.h != "right" && position.h != "none" {
                    position.h = "right".to_string();
                }
            }
        }
    }
}

# timestamp_zone Agent Skill

让开发者通过浏览器快速完成 Unix 时间戳、ISO 8601 和多时区之间的转换。所有计算在浏览器 WASM 中完成，隐私优先。

## 使用流程

1. **打开工具页面**：用户在浏览器中打开 timestamp_zone 页面
2. **选择转换方式**：
   - 点击"现在"获取当前时间
   - 输入 Unix 时间戳（秒或毫秒自动识别）→ 查看多种格式
   - 输入 ISO 8601 字符串 → 获取 Unix 时间戳
   - 选择时区查看当地时间
   - 输入两个时间戳计算时长
3. **复制结果**：每个输出旁边有复制按钮，一键复制到剪贴板

## 工具定义

### `convert_unix_timestamp`

将 Unix 时间戳转换为多种格式。

**参数：**
- `unix_value` (number, 必需): Unix 时间戳，自动识别秒或毫秒（> 10000000000000 视为毫秒）
- `timezone_offset_hours` (number, 可选): 目标时区偏移小时数
- `timezone_offset_minutes` (number, 可选): 目标时区偏移分钟数
- `format` (string, 可选): 自定义 strftime 格式字符串

**返回：**
- `iso8601`: ISO 8601 格式
- `rfc2822`: RFC 2822 格式
- `rfc3339`: RFC 3339 格式
- `unix_seconds`: Unix 秒
- `unix_ms`: Unix 毫秒
- `custom_format`: 自定义格式（如果提供了 format 参数）
- `local_time`: 目标时区时间（如果提供了时区参数）

### `convert_iso8601`

将 ISO 8601 字符串转换为 Unix 时间戳。

**参数：**
- `iso_string` (string, 必需): ISO 8601 格式的时间字符串

**返回：**
- `unix_seconds`: Unix 秒
- `unix_ms`: Unix 毫秒

### `get_common_timezones`

获取常用的时区列表。

**返回：**
- `timezones`: 时区信息数组，每个包含 name、offset_hours、offset_minutes、abbr

### `calculate_duration`

计算两个 Unix 时间戳之间的时长。

**参数：**
- `from_unix` (number, 必需): 起始 Unix 秒时间戳
- `to_unix` (number, 必需): 结束 Unix 秒时间戳

**返回：**
- `total_seconds`: 总秒数
- `days`: 天
- `hours`: 小时
- `minutes`: 分钟
- `seconds`: 秒
- `human_readable`: 人类可读字符串

## Agent 必须遵守

- 不声称需要服务器或网络连接
- 不存储用户的时间戳数据
- 时区转换基于固定偏移量，不处理 DST 规则变更
- 不提供历史日期中 DST 的正确转换（仅按当前偏移量计算）
- 默认输入视为秒级时间戳；仅当数值 > 10000000000000 时自动识别为毫秒

# 项目任务清单（原子化）
> 执行顺序：从上到下，严格按顺序完成，每个任务只对应单个文件，完成后更新状态

---

## 🚩 阶段1：项目初始化与基础框架 (Milestone 1)
| ID | 任务描述 | 对应文件 | 验收标准 | 状态 |
|----|----------|----------|----------|------|
| 1.1 | 初始化Tauri 2.0项目配置 | `src-tauri/tauri.conf.json` | 配置窗口大小、名称、权限，执行`tauri dev`能正常启动空白窗口 | ⏳ 待开始 |
| 1.2 | 配置Vite + Vue 3基础工程 | `vite.config.ts` | 配置端口、路径别名，Vue3 + TypeScript编译正常 | ⏳ 待开始 |
| 1.3 | 配置Tailwind CSS主题 | `tailwind.config.js` | 完全按UI_DESIGN.md配置颜色、字体、圆角、阴影、动画token | ⏳ 待开始 |
| 1.4 | 配置全局样式 | `src/style.css` | 引入Tailwind基础样式，全局重置样式符合苹果设计风格 | ⏳ 待开始 |
| 1.5 | TypeScript全局类型定义 | `src/types/global.d.ts` | 定义通用类型、Tauri命令返回值类型、日志类型 | ⏳ 待开始 |
| 1.6 | Rust数据库连接配置 | `src-tauri/src/db.rs` | 实现SQLite连接池，初始化数据库实例，支持基本CRUD操作 | ⏳ 待开始 |
| 1.7 | 日志表数据库迁移 | `migrations/20240416_create_app_log_table.sql` | 创建app_log表，包含id、level、module、content、timestamp、stack_trace字段 | ⏳ 待开始 |
| 1.8 | 日志模块Rust后端实现 | `src-tauri/src/log.rs` | 实现日志写入、查询、清理接口，支持5个日志级别，按模块分类 | ⏳ 待开始 |
| 1.9 | 注册日志Tauri命令 | `src-tauri/src/main.rs` | 注册log_write、log_query、log_clear命令到Tauri | ⏳ 待开始 |
| 1.10 | 日志模块前端SDK封装 | `src/utils/log.ts` | 封装trace/debug/info/warn/error方法，全局异常捕获自动上报日志 | ⏳ 待开始 |
| 1.11 | 顶部导航栏组件实现 | `src/components/layout/TopNav.vue` | 包含Logo、主菜单（回测/数据/历史）、全局操作区，样式完全符合UI规范 | ⏳ 待开始 |
| 1.12 | Tab标签栏组件实现 | `src/components/layout/TabBar.vue` | 支持多标签页、关闭、新建、切换，动画效果符合设计规范 | ⏳ 待开始 |
| 1.13 | 工作区容器组件实现 | `src/components/layout/Workspace.vue` | 自适应剩余高度，支持内容滚动，响应式适配 | ⏳ 待开始 |
| 1.14 | 全局骨架屏组件实现 | `src/components/common/Skeleton.vue` | 支持卡片、列表、图表多种骨架类型，动画效果流畅 | ⏳ 待开始 |
| 1.15 | 全局进度条组件实现 | `src/components/common/ProgressBar.vue` | 支持进度显示、取消操作，弹性动画效果 | ⏳ 待开始 |
| 1.16 | Vue路由配置 | `src/router/index.ts` | 配置首页、回测配置、回测结果、数据管理四个页面路由 | ⏳ 待开始 |
| 1.17 | 根布局组件实现 | `src/App.vue` | 整合TopNav+TabBar+Workspace，布局结构正确，响应式适配 | ⏳ 待开始 |
| 1.18 | 首页/仪表盘基础结构 | `src/views/Dashboard.vue` | 页面布局符合UI设计，卡片组件样式正确 | ⏳ 待开始 |
| 1.19 | 基础按钮组件实现 | `src/components/common/Button.vue` | 实现主/次/文字/危险四种按钮类型，hover/active效果符合规范 | ⏳ 待开始 |
| 1.20 | 基础卡片组件实现 | `src/components/common/Card.vue` | 圆角、阴影、内边距完全符合UI设计规范 | ⏳ 待开始 |

---

## 🚩 阶段2：数据层开发 (Milestone 2)
| ID | 任务描述 | 对应文件 | 验收标准 | 状态 |
|----|----------|----------|----------|------|
| 2.1 | 品种价格表迁移 | `migrations/20240417_create_price_data_table.sql` | 创建price_data表，包含symbol、date、open、high、low、close、volume字段 | ⏳ 待开始 |
| 2.2 | 品种元数据表迁移 | `migrations/20240417_create_symbol_meta_table.sql` | 创建symbol_meta表，包含symbol、name、type、last_update、data_range字段 | ⏳ 待开始 |
| 2.3 | Yahoo Finance API封装 | `src-tauri/src/yahoo.rs` | 封装yahoo-finance2接口，实现日K线数据下载，支持按时间范围查询 | ⏳ 待开始 |
| 2.4 | 数据层Rust接口实现 | `src-tauri/src/data.rs` | 实现数据保存、查询、更新、删除接口，本地缓存逻辑 | ⏳ 待开始 |
| 2.5 | 注册数据层Tauri命令 | `src-tauri/src/main.rs` | 注册data_fetch、data_query、data_update、data_export命令 | ⏳ 待开始 |
| 2.6 | 数据服务前端封装 | `src/services/data.ts` | 封装数据查询、下载、更新、导出的前端调用接口 | ⏳ 待开始 |
| 2.7 | 基础表单组件实现 | `src/components/form/Input.vue` | 输入框样式符合UI规范，支持focus/hover/error状态 | ⏳ 待开始 |
| 2.8 | 日期选择组件实现 | `src/components/form/DatePicker.vue` | 明亮风格，支持范围选择，样式符合设计规范 | ⏳ 待开始 |
| 2.9 | 下拉选择组件实现 | `src/components/form/Select.vue` | 支持单选、多选，下拉菜单动画效果流畅 | ⏳ 待开始 |
| 2.10 | 数据管理页基础结构 | `src/views/DataManager.vue` | 布局符合UI设计，包含搜索框、表格区域、操作按钮 | ⏳ 待开始 |
| 2.11 | 虚拟表格组件实现 | `src/components/common/VirtualTable.vue` | 支持虚拟滚动、列宽调整、固定首列，性能流畅 | ⏳ 待开始 |
| 2.12 | 启动自动更新数据逻辑 | `src/utils/autoUpdate.ts` | 应用启动时自动检查并更新本地数据，不阻塞UI | ⏳ 待开始 |

---

## 🚩 阶段3：回测引擎核心开发 (Milestone 3)
| ID | 任务描述 | 对应文件 | 验收标准 | 状态 |
|----|----------|----------|----------|------|
| 3.1 | 回测记录数据表迁移 | `migrations/20240418_create_backtest_record_table.sql` | 创建backtest_record表，保存回测配置、结果、创建时间 | ⏳ 待开始 |
| 3.2 | 基础定投策略Rust实现 | `src-tauri/src/backtest/strategies/fixed_invest.rs` | 普通定投策略逻辑，支持周/双周/月频率，参数可配置 | ⏳ 待开始 |
| 3.3 | 价值平均策略Rust实现 | `src-tauri/src/backtest/strategies/value_averaging.rs` | 价值平均策略逻辑，动态调整每次投入金额 | ⏳ 待开始 |
| 3.4 | 指标计算模块Rust实现 | `src-tauri/src/backtest/metrics.rs` | 实现PRD要求的所有收益、风险、风险调整收益、定投相关指标计算 | ⏳ 待开始 |
| 3.5 | 回测引擎核心逻辑 | `src-tauri/src/backtest/engine.rs` | 实现回测主逻辑，多品种组合支持，异步计算，可取消 | ⏳ 待开始 |
| 3.6 | 注册回测Tauri命令 | `src-tauri/src/main.rs` | 注册backtest_run、backtest_query、backtest_delete命令 | ⏳ 待开始 |
| 3.7 | 回测服务前端封装 | `src/services/backtest.ts` | 封装回测启动、查询、删除、导出接口，支持进度监听 | ⏳ 待开始 |
| 3.8 | 回测配置页基础结构 | `src/views/BacktestConfig.vue` | 侧边步骤栏、基础配置区、品种配置区、策略配置区布局符合设计 | ⏳ 待开始 |
| 3.9 | 品种选择组件实现 | `src/components/backtest/SymbolSelector.vue` | 支持搜索、多选、权重配置，界面符合设计规范 | ⏳ 待开始 |
| 3.10 | 策略参数配置组件实现 | `src/components/backtest/StrategyConfig.vue` | 根据选择的策略动态显示对应参数表单 | ⏳ 待开始 |

---

## 🚩 阶段4：图表可视化开发 (Milestone 4)
| ID | 任务描述 | 对应文件 | 验收标准 | 状态 |
|----|----------|----------|----------|------|
| 4.1 | Lightweight Charts基础封装 | `src/components/charts/BaseChart.vue` | 封装Lightweight Charts基础功能，配置主题色符合UI规范 | ⏳ 待开始 |
| 4.2 | 净值曲线图表实现 | `src/components/charts/NetWorthChart.vue` | 支持净值、成本、回撤曲线同图显示，颜色符合设计规范 | ⏳ 待开始 |
| 4.3 | K线图表实现 | `src/components/charts/KlineChart.vue` | 支持K线显示、指标叠加、十字光标、右键菜单 | ⏳ 待开始 |
| 4.4 | 指标计算前端工具函数 | `src/utils/indicators.ts` | 实现MA、RSI、ATR、标准差等技术指标计算 | ⏳ 待开始 |
| 4.5 | 饼图组件实现 | `src/components/charts/PieChart.vue` | 资产配置饼图，符合设计风格，支持交互 | ⏳ 待开始 |
| 4.6 | 回测结果页基础结构 | `src/views/BacktestResult.vue` | 指标卡片、图表区域、详细指标区域布局符合设计 | ⏳ 待开始 |
| 4.7 | 指标卡片组件实现 | `src/components/backtest/MetricCard.vue` | 样式符合设计，支持数字滚动动画，正负数颜色区分 | ⏳ 待开始 |
| 4.8 | 指标详情面板实现 | `src/components/backtest/MetricDetail.vue` | 展示所有回测指标，分类清晰，样式符合规范 | ⏳ 待开始 |

---

## 🚩 阶段5：UI完善与功能补充 (Milestone 5)
| ID | 任务描述 | 对应文件 | 验收标准 | 状态 |
|----|----------|----------|----------|------|
| 5.1 | 蒙特卡洛模拟Rust实现 | `src-tauri/src/backtest/monte_carlo.rs` | 实现蒙特卡洛模拟逻辑，生成随机收益路径 | ⏳ 待开始 |
| 5.2 | 滚动回测Rust实现 | `src-tauri/src/backtest/rolling.rs` | 实现滚动回测和Walk-Forward分析逻辑 | ⏳ 待开始 |
| 5.3 | 全局日志面板实现 | `src/components/common/LogPanel.vue` | 支持按级别、模块、时间筛选日志，界面符合设计规范 | ⏳ 待开始 |
| 5.4 | PDF导出功能实现 | `src/utils/exportPdf.ts` | 支持回测结果导出为PDF报告，格式清晰美观 | ⏳ 待开始 |
| 5.5 | JSON导出功能实现 | `src/utils/exportJson.ts` | 支持回测结果导出为JSON结构化数据 | ⏳ 待开始 |
| 5.6 | 数据备份恢复功能实现 | `src/services/backup.ts` | 支持全量数据备份和恢复，不丢失本地数据 | ⏳ 待开始 |
| 5.7 | 回测历史页面实现 | `src/views/BacktestHistory.vue` | 展示历史回测记录，支持搜索、筛选、删除 | ⏳ 待开始 |
| 5.8 | 多语言支持实现 | `src/i18n/index.ts` | 支持中文/英文切换，所有文案可配置 | ⏳ 待开始 |

---

## 🚩 阶段6：整合优化与MVP发布 (Milestone 6)
| ID | 任务描述 | 对应文件 | 验收标准 | 状态 |
|----|----------|----------|----------|------|
| 6.1 | 全流程功能测试 | `所有文件` | 回测全流程正常运行，无明显BUG | ⏳ 待开始 |
| 6.2 | 性能优化 | `所有文件` | 数据加载流畅，回测计算不阻塞UI，动画60fps | ⏳ 待开始 |
| 6.3 | 错误处理完善 | `所有文件` | 异常场景都有友好提示，错误日志完整记录 | ⏳ 待开始 |
| 6.4 | 打包配置优化 | `src-tauri/tauri.conf.json` | 配置Release打包参数，生成的exe可正常运行 | ⏳ 待开始 |
| 6.5 | MVP版本发布 | `release/` | 生成可用的Windows安装包 | ⏳ 待开始 |

---

## 🎯 任务执行规则
1. 严格按从上到下的顺序执行，不跳步
2. 每个任务只修改对应单个文件，不涉及其他文件
3. 完成后必须按验收标准验证通过
4. 验证通过后更新状态为✅ 已完成
5. 遇到问题记录到`doc/STATUS.md`的「已知的坑」章节

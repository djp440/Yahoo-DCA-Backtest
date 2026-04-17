//! 数据层 - 品种数据与价格数据的CRUD操作

use crate::db::{DbManager, DbResult};
use crate::yahoo::{PriceBar, SymbolType, YahooQuote};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// 品种元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolMeta {
    pub symbol: String,
    pub name: String,
    #[serde(rename = "type")]
    pub symbol_type: String,
    pub exchange: Option<String>,
    pub currency: String,
    pub first_date: Option<String>,
    pub last_date: Option<String>,
    pub total_records: i32,
    pub last_update: Option<String>,
}

/// 价格查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct PriceQueryParams {
    pub symbol: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// 品种列表查询参数
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SymbolListParams {
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// 数据服务
pub struct DataService {
    db: DbManager,
}

impl DataService {
    /// 创建新的数据服务
    pub fn new(db: DbManager) -> Self {
        Self { db }
    }

    // ==================== 品种元数据操作 ====================

    /// 保存或更新品种元数据
    pub fn upsert_symbol(&mut self, quote: &YahooQuote) -> DbResult<()> {
        let symbol_type = SymbolType::from_str(&quote.quote_type);
        let name = quote.long_name.clone().unwrap_or_else(|| quote.short_name.clone());
        let now = Utc::now().to_rfc3339();

        self.db.conn().execute(
            "INSERT INTO symbol_meta (symbol, name, type, exchange, currency, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(symbol) DO UPDATE SET
                 name = excluded.name,
                 type = excluded.type,
                 exchange = excluded.exchange,
                 currency = excluded.currency,
                 updated_at = excluded.updated_at",
            params![
                quote.symbol,
                name,
                symbol_type.to_label(),
                quote.exchange,
                quote.currency,
                now,
            ],
        )?;
        Ok(())
    }

    /// 更新品种数据统计信息
    pub fn update_symbol_stats(
        &mut self,
        symbol: &str,
        first_date: &str,
        last_date: &str,
        total_records: i32,
    ) -> DbResult<()> {
        let now = Utc::now().to_rfc3339();

        self.db.conn().execute(
            "UPDATE symbol_meta
             SET first_date = ?1, last_date = ?2,
                 total_records = ?3, last_update = ?4, updated_at = ?5
             WHERE symbol = ?6",
            params![first_date, last_date, total_records, now, now, symbol],
        )?;
        Ok(())
    }

    /// 查询所有品种列表
    pub fn list_symbols(&self, params: SymbolListParams) -> DbResult<Vec<SymbolMeta>> {
        let mut sql = String::from(
            "SELECT symbol, name, type, exchange, currency,
                    first_date, last_date, total_records, last_update
             FROM symbol_meta WHERE 1=1",
        );
        let mut sql_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        let mut param_index = 1;

        if let Some(ref symbol_type) = params.symbol_type {
            sql.push_str(&format!(" AND type = ?{}", param_index));
            sql_params.push(Box::new(symbol_type.clone()));
            param_index += 1;
        }

        if let Some(ref search) = params.search {
            sql.push_str(&format!(
                " AND (symbol LIKE ?{} OR name LIKE ?{})",
                param_index,
                param_index + 1
            ));
            let pattern = format!("%{}%", search);
            sql_params.push(Box::new(pattern.clone()));
            sql_params.push(Box::new(pattern));
        }

        sql.push_str(" ORDER BY last_update DESC");

        if let Some(limit) = params.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        } else {
            sql.push_str(" LIMIT 1000");
        }

        if let Some(offset) = params.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = self.db.conn().prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> =
            sql_params.iter().map(|p| p.as_ref()).collect();

        let symbols = stmt.query_map(&param_refs[..], |row| {
            Ok(SymbolMeta {
                symbol: row.get(0)?,
                name: row.get(1)?,
                symbol_type: row.get(2)?,
                exchange: row.get(3)?,
                currency: row.get(4)?,
                first_date: row.get(5)?,
                last_date: row.get(6)?,
                total_records: row.get(7)?,
                last_update: row.get(8)?,
            })
        })?;

        symbols.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// 查询单个品种详情
    pub fn get_symbol(&self, symbol: &str) -> DbResult<Option<SymbolMeta>> {
        let mut stmt = self.db.conn().prepare(
            "SELECT symbol, name, type, exchange, currency,
                    first_date, last_date, total_records, last_update
             FROM symbol_meta WHERE symbol = ?1",
        )?;

        let result = stmt.query_row(params![symbol], |row| {
            Ok(SymbolMeta {
                symbol: row.get(0)?,
                name: row.get(1)?,
                symbol_type: row.get(2)?,
                exchange: row.get(3)?,
                currency: row.get(4)?,
                first_date: row.get(5)?,
                last_date: row.get(6)?,
                total_records: row.get(7)?,
                last_update: row.get(8)?,
            })
        });

        match result {
            Ok(meta) => Ok(Some(meta)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// 删除品种及其所有价格数据
    pub fn delete_symbol(&mut self, symbol: &str) -> DbResult<usize> {
        let tx = self.db.conn().unchecked_transaction()?;
        let price_deleted = tx.execute("DELETE FROM price_data WHERE symbol = ?1", params![symbol])?;
        let meta_deleted = tx.execute("DELETE FROM symbol_meta WHERE symbol = ?1", params![symbol])?;
        tx.commit()?;
        Ok(price_deleted + meta_deleted)
    }

    // ==================== 价格数据操作 ====================

    /// 批量保存价格数据（INSERT OR IGNORE，不覆盖已有数据）
    pub fn save_price_data(
        &mut self,
        symbol: &str,
        bars: &[PriceBar],
    ) -> DbResult<usize> {
        if bars.is_empty() {
            return Ok(0);
        }

        let tx = self.db.conn().unchecked_transaction()?;
        let mut inserted = 0;

        {
            let mut stmt = tx.prepare(
                "INSERT OR IGNORE INTO price_data
                 (symbol, date, open, high, low, close, volume, adj_close)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            )?;

            for bar in bars {
                stmt.execute(params![
                    symbol, bar.date, bar.open, bar.high,
                    bar.low, bar.close, bar.volume, bar.adj_close,
                ])?;
                inserted += 1;
            }
        }

        tx.commit()?;

        // 更新品种统计信息
        if inserted > 0 {
            let stats: Option<(String, String, i32)> = self.db.conn().query_row(
                "SELECT MIN(date), MAX(date), COUNT(*)
                 FROM price_data WHERE symbol = ?1",
                params![symbol],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            ).ok();

            if let Some((first_date, last_date, total)) = stats {
                self.update_symbol_stats(symbol, &first_date, &last_date, total)?;
            }
        }

        Ok(inserted)
    }

    /// 查询价格数据
    pub fn query_prices(&self, params: PriceQueryParams) -> DbResult<Vec<PriceBar>> {
        let mut sql = String::from(
            "SELECT date, open, high, low, close, volume, adj_close
             FROM price_data WHERE symbol = ?1",
        );
        let mut sql_params: Vec<Box<dyn rusqlite::ToSql>> =
            vec![Box::new(params.symbol.clone())];
        let mut param_index = 2;

        if let Some(ref start_date) = params.start_date {
            sql.push_str(&format!(" AND date >= ?{}", param_index));
            sql_params.push(Box::new(start_date.clone()));
            param_index += 1;
        }

        if let Some(ref end_date) = params.end_date {
            sql.push_str(&format!(" AND date <= ?{}", param_index));
            sql_params.push(Box::new(end_date.clone()));
        }

        sql.push_str(" ORDER BY date ASC");

        if let Some(limit) = params.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = params.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = self.db.conn().prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> =
            sql_params.iter().map(|p| p.as_ref()).collect();

        let bars = stmt.query_map(&param_refs[..], |row| {
            Ok(PriceBar {
                date: row.get(0)?,
                open: row.get(1)?,
                high: row.get(2)?,
                low: row.get(3)?,
                close: row.get(4)?,
                volume: row.get(5)?,
                adj_close: row.get(6)?,
            })
        })?;

        bars.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// 查询品种的数据范围
    pub fn get_data_range(&self, symbol: &str) -> DbResult<Option<(String, String, i32)>> {
        let result = self.db.conn().query_row(
            "SELECT MIN(date), MAX(date), COUNT(*)
             FROM price_data WHERE symbol = ?1",
            params![symbol],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        );

        match result {
            Ok((first, last, count)) => Ok(Some((first, last, count))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// 删除指定时间范围的价格数据
    pub fn delete_price_data(
        &mut self,
        symbol: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> DbResult<usize> {
        let mut sql = String::from("DELETE FROM price_data WHERE symbol = ?1");
        let mut sql_params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(symbol.to_string())];
        let mut param_index = 2;

        if let Some(s) = start_date {
            sql.push_str(&format!(" AND date >= ?{}", param_index));
            sql_params.push(Box::new(s.to_string()));
            param_index += 1;
        }

        if let Some(e) = end_date {
            sql.push_str(&format!(" AND date <= ?{}", param_index));
            sql_params.push(Box::new(e.to_string()));
        }

        let param_refs: Vec<&dyn rusqlite::ToSql> = sql_params.iter().map(|p| p.as_ref()).collect();
        let deleted = self.db.conn().execute(&sql, &param_refs[..])?;

        // 更新统计信息
        if deleted > 0 {
            let stats: Option<(String, String, i32)> = self.db.conn().query_row(
                "SELECT MIN(date), MAX(date), COUNT(*)
                 FROM price_data WHERE symbol = ?1",
                params![symbol],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            ).ok();

            if let Some((first, last, total)) = stats {
                self.update_symbol_stats(symbol, &first, &last, total)?;
            } else {
                self.update_symbol_stats(symbol, "", "", 0)?;
            }
        }

        Ok(deleted)
    }
}

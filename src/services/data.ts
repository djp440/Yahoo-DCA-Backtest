/**
 * 数据服务 - 封装数据查询、下载、更新、导出的前端调用接口
 */

import { invoke } from '@tauri-apps/api/core'
import type {
  SymbolMeta,
  PriceBar,
  YahooQuote,
  PriceQueryParams,
  SymbolListParams,
  TauriCommandResult,
} from '@/types/global'

// ==================== 类型定义 ====================

interface StoreDataOptions {
  symbol: string
  quote: YahooQuote
  prices: PriceBar[]
}

interface FetchDataResult {
  symbol: string
  count: number
  first_date: string
  last_date: string
}

// ==================== 数据服务类 ====================

class DataService {
  /**
   * 存储品种数据（价格数据 + 元数据）
   */
  async storeData(options: StoreDataOptions): Promise<FetchDataResult> {
    const result = await invoke<TauriCommandResult<FetchDataResult>>('data_store', {
      symbol: options.symbol,
      quote: options.quote,
      prices: options.prices,
    })

    if (!result.ok) {
      throw new Error(result.error || '存储数据失败')
    }
    return result.data!
  }

  /**
   * 查询品种的价格数据
   */
  async queryPrices(params: PriceQueryParams): Promise<PriceBar[]> {
    const result = await invoke<TauriCommandResult<PriceBar[]>>('data_query', {
      params,
    })

    if (!result.ok) {
      throw new Error(result.error || '查询价格数据失败')
    }
    return result.data || []
  }

  /**
   * 查询本地已缓存的品种列表
   */
  async listSymbols(params?: SymbolListParams): Promise<SymbolMeta[]> {
    const result = await invoke<TauriCommandResult<SymbolMeta[]>>(
      'data_list_symbols',
      { params: params || {} }
    )

    if (!result.ok) {
      throw new Error(result.error || '查询品种列表失败')
    }
    return result.data || []
  }

  /**
   * 删除品种及其所有数据
   */
  async deleteSymbol(symbol: string): Promise<number> {
    const result = await invoke<TauriCommandResult<number>>('data_delete_symbol', {
      symbol,
    })

    if (!result.ok) {
      throw new Error(result.error || '删除品种失败')
    }
    return result.data || 0
  }

  /**
   * 从Yahoo Finance下载并存储品种数据
   */
  async fetchAndStore(
    symbol: string,
    startDate: string,
    endDate: string
  ): Promise<FetchDataResult> {
    const yf: typeof import('yahoo-finance2') = await import('yahoo-finance2')

    const quotes = await yf.default.historical(symbol, {
      start: startDate,
      end: endDate,
    })

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const quotesArray = quotes as any[]
    if (!quotesArray || quotesArray.length === 0) {
      throw new Error(`未找到 ${symbol} 的数据`)
    }

    const prices: PriceBar[] = quotesArray.map((q) => ({
      date: q.date instanceof Date
        ? q.date.toISOString().split('T')[0]
        : String(q.date).split('T')[0],
      open: Number(q.open) || 0,
      high: Number(q.high) || 0,
      low: Number(q.low) || 0,
      close: Number(q.close) || 0,
      volume: Number(q.volume) || 0,
      adj_close: Number(q.adjClose) || Number(q.close) || 0,
    }))

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const quoteData = await yf.default.quote(symbol) as any

    const yahooQuote: YahooQuote = {
      symbol: String(quoteData?.symbol || symbol),
      short_name: String(quoteData?.shortName || symbol),
      long_name: quoteData?.longName ? String(quoteData.longName) : null,
      quote_type: String(quoteData?.quoteType || 'EQUITY'),
      currency: String(quoteData?.currency || 'USD'),
      exchange: String(quoteData?.exchange || ''),
      regular_market_price: quoteData?.regularMarketPrice
        ? Number(quoteData.regularMarketPrice)
        : null,
      regular_market_previous_close: quoteData?.regularMarketPreviousClose
        ? Number(quoteData.regularMarketPreviousClose)
        : null,
      fifty_two_week_high: quoteData?.fiftyTwoWeekHigh
        ? Number(quoteData.fiftyTwoWeekHigh)
        : null,
      fifty_two_week_low: quoteData?.fiftyTwoWeekLow
        ? Number(quoteData.fiftyTwoWeekLow)
        : null,
    }

    return this.storeData({
      symbol,
      quote: yahooQuote,
      prices,
    })
  }
}

// ==================== 全局实例 ====================

const dataService = new DataService()

export { dataService }
export default dataService

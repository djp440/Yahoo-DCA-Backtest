/**
 * 技术指标计算工具函数
 * 实现MA、RSI、ATR、标准差等指标计算
 */

/**
 * 计算简单移动平均线 (SMA)
 */
export function calculateSMA(data: number[], period: number): (number | null)[] {
  const result: (number | null)[] = []

  for (let i = 0; i < data.length; i++) {
    if (i < period - 1) {
      result.push(null)
    } else {
      const sum = data.slice(i - period + 1, i + 1).reduce((a, b) => a + b, 0)
      result.push(sum / period)
    }
  }

  return result
}

/**
 * 计算指数移动平均线 (EMA)
 */
export function calculateEMA(data: number[], period: number): (number | null)[] {
  const result: (number | null)[] = []
  const multiplier = 2 / (period + 1)

  for (let i = 0; i < data.length; i++) {
    if (i < period - 1) {
      result.push(null)
    } else if (i === period - 1) {
      const sum = data.slice(0, period).reduce((a, b) => a + b, 0)
      result.push(sum / period)
    } else {
      const prevEMA = result[i - 1] as number
      const ema = (data[i] - prevEMA) * multiplier + prevEMA
      result.push(ema)
    }
  }

  return result
}

/**
 * 计算RSI (相对强弱指数)
 */
export function calculateRSI(prices: number[], period: number = 14): (number | null)[] {
  const result: (number | null)[] = []
  const gains: number[] = []
  const losses: number[] = []

  for (let i = 1; i < prices.length; i++) {
    const change = prices[i] - prices[i - 1]
    gains.push(change > 0 ? change : 0)
    losses.push(change < 0 ? Math.abs(change) : 0)
  }

  for (let i = 0; i < prices.length; i++) {
    if (i < period) {
      result.push(null)
    } else {
      const avgGain = gains.slice(i - period, i).reduce((a, b) => a + b, 0) / period
      const avgLoss = losses.slice(i - period, i).reduce((a, b) => a + b, 0) / period

      if (avgLoss === 0) {
        result.push(100)
      } else {
        const rs = avgGain / avgLoss
        result.push(100 - 100 / (1 + rs))
      }
    }
  }

  return result
}

/**
 * 计算ATR (平均真实波幅)
 */
export function calculateATR(
  highs: number[],
  lows: number[],
  closes: number[],
  period: number = 14
): (number | null)[] {
  const result: (number | null)[] = []
  const trueRanges: number[] = []

  for (let i = 0; i < highs.length; i++) {
    if (i === 0) {
      trueRanges.push(highs[i] - lows[i])
    } else {
      const tr = Math.max(
        highs[i] - lows[i],
        Math.abs(highs[i] - closes[i - 1]),
        Math.abs(lows[i] - closes[i - 1])
      )
      trueRanges.push(tr)
    }
  }

  for (let i = 0; i < highs.length; i++) {
    if (i < period - 1) {
      result.push(null)
    } else if (i === period - 1) {
      const atr = trueRanges.slice(0, period).reduce((a, b) => a + b, 0) / period
      result.push(atr)
    } else {
      const prevATR = result[i - 1] as number
      const atr = (prevATR * (period - 1) + trueRanges[i]) / period
      result.push(atr)
    }
  }

  return result
}

/**
 * 计算标准差
 */
export function calculateStdDev(data: number[], period: number): (number | null)[] {
  const result: (number | null)[] = []

  for (let i = 0; i < data.length; i++) {
    if (i < period - 1) {
      result.push(null)
    } else {
      const slice = data.slice(i - period + 1, i + 1)
      const mean = slice.reduce((a, b) => a + b, 0) / period
      const variance = slice.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / period
      result.push(Math.sqrt(variance))
    }
  }

  return result
}

/**
 * 计算加权移动平均线 (WMA)
 */
export function calculateWMA(data: number[], period: number): (number | null)[] {
  const result: (number | null)[] = []
  const weightSum = (period * (period + 1)) / 2

  for (let i = 0; i < data.length; i++) {
    if (i < period - 1) {
      result.push(null)
    } else {
      let weightedSum = 0
      for (let j = 0; j < period; j++) {
        weightedSum += data[i - j] * (period - j)
      }
      result.push(weightedSum / weightSum)
    }
  }

  return result
}
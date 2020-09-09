const _ = require('lodash')
const tty = require('tty')

//  ------ 级别 ------
const Levels = {
  NO_LOG: -1, // 无日志
  FATAL: 0, // 致命报错
  ERROR: 1, // 错误日志
  WARN: 2, // 警报日志
  NOTICE: 3, // 普通日志，但须有所注意
  INFO: 4, // 普通日志
  DEBUG: 5 // 调试日志
}

let inspectOpts = {}
const loggers = new Map()

/**
 * @returns {Number}
 */
function getLoggerLevel () {
  return process.env.NODE_ENV === 'production' ? Levels['ERROR'] : Levels['DEBUG']
}
function setLoggerLevel (val) {
  inspectOpts['level'] = val
}

//  ------ 颜色 ------
let colors = [ 6, 2, 3, 4, 5, 1 ]
try {
  var supportsColor = require('supports-color').stdout
  if (supportsColor && supportsColor.level >= 2) {
    colors = [
      20, 21, 26, 27, 32, 33, 38, 39, 40, 41, 42, 43, 44, 45, 56, 57, 62, 63, 68,
      69, 74, 75, 76, 77, 78, 79, 80, 81, 92, 93, 98, 99, 112, 113, 128, 129, 134,
      135, 148, 149, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171,
      172, 173, 178, 179, 184, 185, 196, 197, 198, 199, 200, 201, 202, 203, 204,
      205, 206, 207, 208, 209, 214, 215, 220, 221
    ]
  }
} catch (err) {
  // swallow - we only care if `supports-color` is available; it doesn't have to be.
}

/**
 * Is stdout a TTY? Colored output is enabled when `true`.
 * @returns {Boolean}
 */
function useColors () {
  return 'colors' in inspectOpts
    ? Boolean(inspectOpts.colors)
    : tty.isatty(process.stderr.fd)
}

function hash2Num (namespace) {
  let hash = 0
  for (let i in namespace) {
    hash = ((hash << 5) - hash) + namespace.charCodeAt(i)
    hash |= 0 // Convert to 32bit integer
  }
  return Math.abs(hash)
}

/**
 * Select a color.
 * @param {String} namespace
 * @return {Number}
 * @api private
 */
function selectColor (namespace) {
  return colors[hash2Num(namespace) % colors.length]
}

// 通用私有方法
/**
 * @private
 */
function _error (message, err, tags) {
  if (!tags) {
    if (message instanceof Error) {
      let args2 = err
      err = message
      tags = args2 || []
      message = null
    } else {
      tags = []
    }
  }
  if (!(err instanceof Error)) {
    err = err ? new Error(err.message || err) : new Error()
  }
  let errStr = `message=${err.message},name=${err.name}`
  if (process.env.NODE_ENV !== 'production') {
    errStr += `,stack=${err.stack}}`
  }
  if (message !== '' && message !== null) {
    errStr = `info=${message},${errStr}`
  }
  // Response Error特殊处理
  if (err.response) {
    const resError = err.response
    const resData = typeof resError.data === 'string' ? resError.data : JSON.stringify(resError.data || {})
    errStr = `res=[${resError.status}|${resError.statusText}|${resData}],${errStr}`
  }
  // 构建message
  const msg = _formatRecord.call(this, errStr, tags)
  console.error(msg)
  return this
}

/**
 * @param {String} message 日志记录
 * @param {String[]} tags 各类日志标签
 * @private
 */
function _log (message, tags) {
  const msg = _formatRecord.call(this, message || 'EMPTY', tags)
  console.log(msg)
  return this
}

/**
 * 标准化输出
 * @returns {String}
 */
function _formatRecord (message, tags) {
  const datestr = getDate()
  const tagstr = _buildTags.call(this, tags)
  // 构建日志
  let prefix = datestr + '|' + tagstr + '|'
  message = prefix + message.split('\n').join('\n' + prefix)
  return message
}

/**
 * 生成Log Tags
 * @param {String[]} tags 各类日志标签
 * @returns {String}
 * @private
 */
function _buildTags (tags) {
  if (this._nextTags) {
    tags.unshift(...this._nextTags)
    this._nextTags = null
  }

  // 添加主类型
  let categoryTag = !this._sub ? this._category : `${this._category} - ${this._sub}`
  tags.unshift(categoryTag)
  // 构建tags
  tags = (tags || []).map(t => `[${t}]`)

  const tagstr = tags.join('')
  const c = this._color
  return this._useColors
    ? `\u001b[3${c < 8 ? c : '8;5;' + c};1m${tagstr}\u001b[0m`
    : tagstr
}

/**
 * 获取日期时间
 */
function getDate () {
  return inspectOpts.hideDate ? '' : new Date().toLocaleString()
}

class Logger {
  /**
   * 标准化日志
   * @param {String} category 主类别
   * @param {String} sub 子类别
   */
  constructor (category, sub) {
    this['_category'] = category || 'Default'
    this['_sub'] = sub || ''
    this['_color'] = selectColor(this._category + sub)
    this['_useColors'] = useColors()
  }
  // 链式调用辅助功能
  /**
   * 添加之后第一个log中的标签
   * @param  {...String} tags
   */
  tag (...tags) {
    this['_nextTags'] = tags
    return this
  }
  // 功能性日志记录
  // ----------------- Error型 --------------------
  /**
   * 错误日志
   * @param {String} message info记录
   * @param {Error} err 错误对象
   * @param {String[]} tags 各类日志标签
   */
  error (message, err, tags = undefined) {
    if (getLoggerLevel() < Levels.ERROR) return this
    return _error.call(this, message, err, tags)
  }
  // ------------------- Log型 ----------------------
  /**
   * 警报日志
   * @param {String} message info记录
   * @param {String[]} tags 各类日志标签
   */
  warn (message, ...args) {
    if (getLoggerLevel() < Levels.WARN) return this
    const tags = _.isArray(args[args.length - 1]) ? args.pop() : []
    tags.unshift('WARN')
    args.unshift(message)
    return _log.call(this, args.join(','), tags)
  }
  /**
   * 普通日志，但须有所注意
   * @param {String} message info记录
   * @param {String[]} tags 各类日志标签
   */
  notice (message, ...args) {
    if (getLoggerLevel() < Levels.NOTICE) return this
    const tags = _.isArray(args[args.length - 1]) ? args.pop() : []
    tags.unshift('NOTICE')
    args.unshift(message)
    return _log.call(this, args.join(','), tags)
  }
  /**
   * 普通日志，常用方法，使用NOTICE级别
   * @param {String} message info记录
   * @param {String[]} tags 各类日志标签
   */
  log (message, ...args) {
    if (getLoggerLevel() < Levels.NOTICE) return this
    const tags = _.isArray(args[args.length - 1]) ? args.pop() : []
    args.unshift(message)
    return _log.call(this, args.join(','), tags)
  }
  /**
   * 普通日志，带'INFO' tag
   * @param {String} message info记录
   * @param {String[]} tags 各类日志标签
   */
  info (message, ...args) {
    if (getLoggerLevel() < Levels.INFO) return this
    const tags = _.isArray(args[args.length - 1]) ? args.pop() : []
    tags.unshift('INFO')
    args.unshift(message)
    return _log.call(this, args.join(','), tags)
  }
  /**
   * 调试日志
   * @param {String} message info记录
   * @param {String[]} tags 各类日志标签
   */
  debug (message, ...args) {
    if (getLoggerLevel() < Levels.DEBUG) return this
    const tags = _.isArray(args[args.length - 1]) ? args.pop() : []
    tags.unshift('DEBUG')
    args.unshift(message)
    return _log.call(this, args.join(','), tags)
  }
  /**
   * 平铺记录对象，使用log方法
   * @param {Object} msgObj 日志对象
   * @param {String[]} tags
   */
  logObj (msgObj, tags) {
    let args = []
    for (const k in msgObj) {
      if (msgObj.hasOwnProperty(k)) {
        args.push(`${k}=${msgObj[k]}`)
      }
    }
    if (tags) args.push(tags)
    return this.log(...args)
  }
}

module.exports = Object.assign(Logger, {
  // Methods
  setLoggerLevel,
  /**
   * 返回Logger
   * @param {String} name 主名称
   * @param {String} sub 子名称
   * @returns {Logger}
   */
  of (name, sub) {
    let key = name + (sub || '')
    let logger = loggers.get(key)
    if (!logger) {
      logger = new Logger(name, sub)
      loggers.set(key, logger)
    }
    return logger
  }
})

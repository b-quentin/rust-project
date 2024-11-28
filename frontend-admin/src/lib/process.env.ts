const logLevelFromEnv = process.env.LOG_LEVEL_FRONTEND_ADMIN;

enum LogLevel {
  Silly = 0,
  Trace = 1,
  Debug = 2,
  Info = 3,
  Warn = 4,
  Error = 5,
  Fatal = 6
}

function getLogLevel(level: string | undefined): LogLevel {
  if (!level) {
    return LogLevel.Error;
  }

  switch (level.toLowerCase()) {
    case "silly":
      return LogLevel.Silly;
    case "trace":
      return LogLevel.Trace;
    case "debug":
      return LogLevel.Debug;
    case "info":
      return LogLevel.Info;
    case "warn":
      return LogLevel.Warn;
    case "error":
      return LogLevel.Error;
    case "fatal":
      return LogLevel.Fatal;
    default:
      return LogLevel.Error;
  }
}

const logLevel = getLogLevel(logLevelFromEnv);

export { logLevel };


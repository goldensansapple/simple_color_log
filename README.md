# Simple Color Log

This is a simple logger that uses standard log levels and prints the file name, line number, and timestamp for each log message.
The log levels are as follows:

- FATAL
- ERROR
- WARN
- INFO
- DEBUG
- TRACE

The logger's level is set through the feature flags. These are the allowed feature flags:

- `error`
- `warn`
- `info`
- `debug`
- `trace`

The logger will only print messages that are at or above the level set by the feature flag. The default level is `info`.

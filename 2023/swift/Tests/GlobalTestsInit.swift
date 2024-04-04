import Logging
import XCTest

/*--------------------------------------------------------------------------------------
  Initialize the Logging System
--------------------------------------------------------------------------------------*/

let testLogLevel = Logger.Level.notice

let loggingSystemBootstrapped: Bool = {
    LoggingSystem.bootstrap { label in
        var logHandler = StreamLogHandler.standardError(label: label)
        logHandler.logLevel = testLogLevel
        return logHandler
    }
    print("Logging system bootstrapped with log level: \(testLogLevel)")
    return true
}()

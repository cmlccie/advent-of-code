import Foundation
import Logging

/*--------------------------------------------------------------------------------------
  Utility Functions
--------------------------------------------------------------------------------------*/

internal func getInput(from path: String) throws -> String {
    let logger = Logger(label: "utils.getInput")

    guard let fileData = FileManager.default.contents(atPath: path) else {
        let message = "Failed to read file at path: \(path)"
        logger.error(Logger.Message(stringLiteral: message))
        throw InputError.fileSystemError(message)
    }

    guard let input = String(data: fileData, encoding: .utf8) else {
        let message = "Failed to decode file as UTF-8 text"
        logger.error(Logger.Message(stringLiteral: message))
        throw InputError.DecodingError(message)
    }

    return input
}

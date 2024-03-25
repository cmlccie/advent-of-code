import Foundation

/*--------------------------------------------------------------------------------------
  Utility Functions
--------------------------------------------------------------------------------------*/

internal func getInput(from path: String) -> String {
    let fileData = FileManager.default.contents(atPath: path)!
    return String(data: fileData, encoding: .utf8)!
}

import Foundation
import OSLog

func fetchXProtectLogs() {
    do {
        // Open the system log store (which includes kernel and system logs)
        let logStore = try OSLogStore(scope: .system)
        
        // Define the time window: last 1 hour
        let startDate = Date().addingTimeInterval(-3600)  // 1 hour ago
        let startPosition = logStore.position(date: startDate)
        
        // Predicate to match any log entry whose subsystem contains "xprotect" (case-insensitive)
        let predicate = NSPredicate(format: "subsystem CONTAINS[c] %@", "xprotect")
        
        // Query the log store for entries since the start position matching the predicate
        let entries = try logStore.getEntries(at: startPosition, matching: predicate)
        
        // For each log entry, if it is of type OSLogEntryLog, extract key properties and convert to JSON
        for entry in entries {
            if let logEntry = entry as? OSLogEntryLog {
                let logDict: [String: Any] = [
                    "date": logEntry.date.description,
                    "process": logEntry.process,
                    "subsystem": logEntry.subsystem,
                    "category": logEntry.category,
                    "message": logEntry.composedMessage
                ]
                
                if let jsonData = try? JSONSerialization.data(withJSONObject: logDict, options: [.prettyPrinted]),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    print(jsonString)
                }
            }
        }
    } catch {
        print("Error retrieving logs: \(error)")
    }
}

fetchXProtectLogs()

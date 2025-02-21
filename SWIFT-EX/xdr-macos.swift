import Foundation
import OSLog

func pollXProtectLogs() {
    do {
        // Open the system-wide log store.
        let logStore = try OSLogStore(scope: .system)
        // Start by looking back one hour.
        var lastDate = Date().addingTimeInterval(-3600)
        
        while true {
            // Get a position in the log store corresponding to lastDate.
            let startPosition = logStore.position(date: lastDate)
            // Define a predicate to capture any log entry whose subsystem contains "xprotect" (case-insensitive)
            let predicate = NSPredicate(format: "subsystem CONTAINS[c] %@", "xprotect")
            // Query log entries from startPosition matching our predicate.
            let entries = try logStore.getEntries(at: startPosition, matching: predicate)
            
            for entry in entries {
                if let logEntry = entry as? OSLogEntryLog {
                    // Create a dictionary representing the log entry.
                    let logDict: [String: Any] = [
                        "date": logEntry.date.description,
                        "process": logEntry.process,
                        "subsystem": logEntry.subsystem,
                        "category": logEntry.category,
                        "message": logEntry.composedMessage
                    ]
                    // Convert dictionary to JSON string for easier ingestion.
                    if let jsonData = try? JSONSerialization.data(withJSONObject: logDict, options: .prettyPrinted),
                       let jsonString = String(data: jsonData, encoding: .utf8) {
                        print(jsonString)
                    }
                }
            }
            
            // Update lastDate to current time so we only fetch new entries next time.
            lastDate = Date()
            // Sleep for 10 seconds before polling again.
            sleep(10)
        }
    } catch {
        print("Error retrieving logs: \(error)")
    }
}

pollXProtectLogs()

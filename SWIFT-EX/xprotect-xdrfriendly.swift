import OSLog

do {
    // Open the system-wide log store
    let logStore = try OSLogStore(scope: .system)
    
    // Define the time window: last 1 hour
    let startDate = Date().addingTimeInterval(-3600) // 1 hour ago
    let startPosition = logStore.position(date: startDate)
    
    // Use a predicate to capture any log entry whose subsystem contains "xprotect" (case-insensitive)
    let predicate = NSPredicate(format: "subsystem CONTAINS[c] %@", "xprotect")
    
    // Query the log store for matching entries
    let entries = try logStore.getEntries(at: startPosition, matching: predicate)
    
    // Loop through each entry and print its composed message (if available)
    for entry in entries {
        if let logEntry = entry as? OSLogEntryLog {
            print(logEntry.composedMessage)
        } else {
            // For other types of entries, print the default description
            print(entry)
        }
    }
} catch {
    print("Error retrieving logs: \(error)")
}

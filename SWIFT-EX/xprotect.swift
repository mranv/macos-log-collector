import OSLog

do {
    // Open the system-wide log store.
    let logStore = try OSLogStore(scope: .system)
    
    // Set the timeframe to the last 1 hour.
    let startDate = Date().addingTimeInterval(-3600) // 1 hour ago
    let startPosition = logStore.position(date: startDate)
    
    // Use a predicate to capture any log whose subsystem contains "xprotect" (case-insensitive)
    let predicate = NSPredicate(format: "subsystem CONTAINS[c] %@", "xprotect")
    
    let entries = try logStore.getEntries(at: startPosition, matching: predicate)
    
    // Loop through and print the composed log message for each entry.
    for entry in entries {
        if let logEntry = entry as? OSLogEntryLog {
            print(logEntry.composedMessage)
        }
    }
} catch {
    print("Error retrieving logs: \(error)")
}

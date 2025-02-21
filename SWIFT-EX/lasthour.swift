import OSLog

do {
    // Open the system-wide log store
    let logStore = try OSLogStore(scope: .system)
    
    // Set the timeframe to the last 1 hour
    let startDate = Date().addingTimeInterval(-3600) // 3600 seconds = 1 hour
    let startPosition = logStore.position(date: startDate)
    
    // Filter for kernel messages (processID == 0)
    let predicate = NSPredicate(format: "processID == 0")
    let entries = try logStore.getEntries(at: startPosition, matching: predicate)
    
    // Loop through and print the actual log messages
    for entry in entries {
        if let logEntry = entry as? OSLogEntryLog {
            print(logEntry.composedMessage)
        } else {
            print(entry)
        }
    }
} catch {
    print("Error retrieving logs: \(error)")
}

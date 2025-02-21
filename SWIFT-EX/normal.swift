import OSLog

do {
    // Create a log store that covers the last hour.
    let logStore = try OSLogStore(scope: .currentProcessIdentifier)
    let oneHourAgo = Date().addingTimeInterval(-3600)
    let position = logStore.position(date: oneHourAgo)
    
    // Retrieve entries since that position.
    let entries = try logStore.getEntries(at: position)
    for entry in entries {
        print(entry)
    }
} catch {
    print("Error retrieving logs: \(error)")
}

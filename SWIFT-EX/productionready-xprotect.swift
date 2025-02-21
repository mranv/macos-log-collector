import Foundation
import OSLog

/// Production-ready XProtect log collector for EDR/XDR integration.
class XProtectRiskCollector {
    
    // MARK: - Configuration
    
    /// How often (in seconds) to poll for new log entries.
    let pollInterval: TimeInterval = 60.0  // every 60 seconds
    
    /// Look-back window for each poll (in seconds). Default: last 1 hour.
    let lookBackInterval: TimeInterval = 3600
    
    /// File path to store risk events as JSON.
    let outputFileURL: URL
    
    /// Predicate to capture any log entry whose subsystem contains "xprotect" (case-insensitive).
    let basePredicate = NSPredicate(format: "subsystem CONTAINS[c] %@", "xprotect")
    
    /// A list of substrings that, if found in the composed message, indicate routine messages to skip.
    let benignPhrases: [String] = [
        "Already up to date",
        "Using XProtect rules location",
        "Forwarding detection succeeded",
        "JETSAM_REASON_MEMORY_IDLE_EXIT",
        "shutting down",
        "cleaning up",
        "XPC connection invalidated",
        "Waiting for launchd to call us"
    ]
    
    /// Last time the log was polled.
    var lastPollDate: Date
    
    /// The OSLogStore for system logs.
    let logStore: OSLogStore
    
    // MARK: - Initialization
    
    init(outputFilePath: String) throws {
        self.logStore = try OSLogStore(scope: .system)
        self.lastPollDate = Date().addingTimeInterval(-lookBackInterval)
        self.outputFileURL = URL(fileURLWithPath: outputFilePath)
    }
    
    // MARK: - Helper Functions
    
    /// Checks if the given message should be considered benign.
    func isBenign(message: String) -> Bool {
        for phrase in benignPhrases {
            if message.contains(phrase) {
                return true
            }
        }
        return false
    }
    
    /// Processes log entries: filters risk events and returns an array of dictionaries.
    func processEntries(entries: [OSLogEntry]) -> [[String: Any]] {
        var riskEvents: [[String: Any]] = []
        let isoFormatter = ISO8601DateFormatter()
        
        for entry in entries {
            // We care only about OSLogEntryLog entries
            if let logEntry = entry as? OSLogEntryLog {
                let message = logEntry.composedMessage
                // Skip entries that contain any known benign phrases.
                if isBenign(message: message) {
                    continue
                }
                let event: [String: Any] = [
                    "date": isoFormatter.string(from: logEntry.date),
                    "process": logEntry.process,
                    "subsystem": logEntry.subsystem,
                    "category": logEntry.category,
                    "message": message
                ]
                riskEvents.append(event)
            }
        }
        return riskEvents
    }
    
    /// Appends the given JSON data to the output file.
    func appendToFile(jsonData: Data) {
        if FileManager.default.fileExists(atPath: outputFileURL.path) {
            if let handle = try? FileHandle(forWritingTo: outputFileURL) {
                handle.seekToEndOfFile()
                handle.write(jsonData)
                handle.write("\n".data(using: .utf8)!)
                handle.closeFile()
            }
        } else {
            try? jsonData.write(to: outputFileURL)
            try? "\n".data(using: .utf8)?.write(to: outputFileURL, options: .atomic)
        }
    }
    
    // MARK: - Polling Function
    
    /// Polls the system log for new XProtect entries and processes risk events.
    func pollLogs() {
        do {
            let startPosition = logStore.position(date: lastPollDate)
            // Convert the AnySequence to an Array.
            let entriesSequence = try logStore.getEntries(at: startPosition, matching: basePredicate)
            let entries = Array(entriesSequence)
            
            let riskEvents = processEntries(entries: entries)
            
            if !riskEvents.isEmpty {
                let jsonData = try JSONSerialization.data(withJSONObject: riskEvents, options: [.prettyPrinted])
                appendToFile(jsonData: jsonData)
                if let jsonString = String(data: jsonData, encoding: .utf8) {
                    print(jsonString)
                }
            }
            
            lastPollDate = Date()
        } catch {
            fputs("Error polling logs: \(error)\n", stderr)
        }
    }
    
    // MARK: - Start Collector
    
    /// Starts continuous log collection.
    func startCollecting() {
        let timer = DispatchSource.makeTimerSource(queue: DispatchQueue.global(qos: .background))
        timer.schedule(deadline: .now(), repeating: pollInterval)
        timer.setEventHandler { [weak self] in
            self?.pollLogs()
        }
        timer.resume()
        dispatchMain()
    }
}

// MARK: - Main Execution

do {
    let collector = try XProtectRiskCollector(outputFilePath: "/var/log/xprotect_risk_events.json")
    collector.startCollecting()
} catch {
    fputs("Failed to initialize XProtect risk collector: \(error)\n", stderr)
    exit(1)
}

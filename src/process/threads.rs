use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, Thread32First, Thread32Next, THREADENTRY32};
use windows::Win32::System::Threading::{OpenThread, ResumeThread, SuspendThread, THREAD_ALL_ACCESS};
#[derive(Debug, Clone)]
pub struct ProcessThreads {
    process_id: u32
}
impl ProcessThreads {
    /// Constructor in order to take a process id and allow us to get thread information about the process
    pub fn new(process_id: u32) -> Self {
        Self { process_id }
    }
    /// Suspend all threads
    pub unsafe fn suspend_threads(&self) {
        // Get thread snapshot
        let thread_snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, self.process_id).expect("Couldn't get a snapshot of threads");
        // Create struct
        let mut thread_entry = THREADENTRY32::default();
        // Initialise size
        thread_entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;
        // Get first entry
        Thread32First(thread_snap, &mut thread_entry).expect("Couldn't get first thread");
        // Loop through threads
        loop {
            if thread_entry.th32OwnerProcessID == self.process_id {
                // Open thread
                let thread_handle = OpenThread(THREAD_ALL_ACCESS, false, thread_entry.th32ThreadID).unwrap();
                // Suspend thread
                SuspendThread(thread_handle);
            }
            if Thread32Next(thread_snap, &mut thread_entry).is_err() { break }
        }
        CloseHandle(thread_snap).expect("Failed to close handle");
    }
    /// Resume all threads
    pub unsafe fn resume_threads(&self) {
        // Get thread snapshot
        let thread_snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, self.process_id).expect("Couldn't get a snapshot of threads");
        // Create struct
        let mut thread_entry = THREADENTRY32::default();
        // Initialise size
        thread_entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;
        // Get first entry
        Thread32First(thread_snap, &mut thread_entry).expect("Couldn't get first thread");
        // Loop through threads
        loop {
            if thread_entry.th32OwnerProcessID == self.process_id {
                // Open thread
                let thread_handle = OpenThread(THREAD_ALL_ACCESS, false, thread_entry.th32ThreadID).unwrap();
                // Resume thread
                ResumeThread(thread_handle);
            }
            if Thread32Next(thread_snap, &mut thread_entry).is_err() { break }
        }
        CloseHandle(thread_snap).expect("Failed to close handle");
    }
}
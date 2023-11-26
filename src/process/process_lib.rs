use dynasmrt::DynasmApi;
use std::collections::HashMap;
use std::ffi::c_void;
use dynasmrt::dynasm;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS};
use windows::Win32::System::Memory::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAllocEx};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use crate::process::module::ModuleInfo;
use crate::process::threads::ProcessThreads;

#[derive(Debug, Clone)]
pub struct Process {
    process_handle: HANDLE,
    process_id: u32,
    process_threads: ProcessThreads,
}

impl Process {
    /// Constructor to find a process by name and extract all the key information we'll need to perform analysis on said process
    pub unsafe fn find(name_of_process: &str) -> Self {
        // Create a process snapshot
        let proc_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).expect("Failed to get process snapshot");
        // Setup process entry
        let mut proc_entry = PROCESSENTRY32::default();
        // Set struct size
        proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        // Check first process
        Process32First(proc_snapshot, &mut proc_entry).expect("Failed to get initial process");
        // Loop through all processes
        loop {
            // Get process name
            let proc_name: String = String::from_utf8_lossy(&proc_entry.szExeFile[..]).trim_end_matches('\0').to_string();
            // Check name
            if &proc_name == name_of_process {
                CloseHandle(proc_snapshot).expect("Failed to close handle");
                return Self {
                    // Set Process ID
                    process_id: proc_entry.th32ProcessID,
                    // Open a handle and set it to our field
                    process_handle: OpenProcess(PROCESS_ALL_ACCESS, false, proc_entry.th32ProcessID).unwrap(),
                    // Create thread entry
                    process_threads: ProcessThreads::new(proc_entry.th32ProcessID)
                };
            }
            // Clear buffer
            proc_entry.szExeFile.iter_mut().for_each(|e_byte| *e_byte = 0x0);
            // Onto next entry
            if Process32Next(proc_snapshot, &mut proc_entry).is_err() { break; }
        }
        CloseHandle(proc_snapshot).expect("Failed to close handle");
        return Self {
            // Set Process ID
            process_id: u32::default(),
            // Open a handle and set it to our field
            process_handle: HANDLE::default(),
            // Create thread entry
            process_threads: ProcessThreads::new(0)
        };
    }
    /// Write value of type T to the given process at location addr_to_write
    pub unsafe fn write<T>(&self, addr_to_write: usize, value_to_write: T) -> Result<(), String> {
        if WriteProcessMemory(self.process_handle,
                               addr_to_write as *const c_void,
                               &value_to_write as *const T as *const c_void,
                               std::mem::size_of::<T>(),
                               None
        ).is_err() {
            return Err(String::from("Failed to write to process' memory"));
        }
        Ok(())
    }
    /// Write an array of bytes to the given process at location addr_to_write
    pub unsafe fn write_bytes(&self, addr_to_write: usize, value_to_write: &[u8]) -> Result<(), String> {
        let val_ptr = value_to_write.as_ptr() as *const c_void;
        if WriteProcessMemory(self.process_handle,
                               addr_to_write as *const c_void,
                               val_ptr,
                               std::mem::size_of_val(value_to_write),
                               None
        ).is_err() {
            return Err(String::from("Failed to write to process' memory"));
        }
        Ok(())
    }
    /// Return a rust vector of modules loaded by given process
    unsafe fn vec_modules(&self) -> Vec<ModuleInfo> {
        let mut module_vec = vec![];
        // Get module snapshot
        let module_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE32 | TH32CS_SNAPMODULE, self.process_id).expect("Failed to get module snapshot");
        // Setup module struct
        let mut module_entry = MODULEENTRY32::default();
        // Set struct size
        module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
        // Check first module
        Module32First(module_snapshot, &mut module_entry).expect("Failed to get first module");
        loop {
            let module_name = std::str::from_utf8(&module_entry.szModule[..]).unwrap().trim_end_matches('\0');
            module_vec.push(
                ModuleInfo::new(
                    module_name,
                    module_entry.modBaseAddr as usize,
                )
            );
            // Clear buffer
            module_entry.szModule.iter_mut().for_each(|e_byte| *e_byte = 0x0);
            // Onto next entry
            if Module32Next(module_snapshot, &mut module_entry).is_err() { break; }
        }
        CloseHandle(module_snapshot).expect("Failed to close handle");
        module_vec
    }
    /// Create a hashmap of modules loaded by given process
    pub unsafe fn get_modules(&self) -> HashMap<String, ModuleInfo> {
        let mut hashmap_modules = HashMap::new();
        // Loop through modules and append it to the hashmap
        for curr_module in self.vec_modules() {

            hashmap_modules.insert(curr_module.get_mod_name().to_lowercase(), curr_module);
        }
        hashmap_modules
    }
    /// Read memory of type T from the process at the given location addr_to_read
    pub unsafe fn read<T/*: std::fmt::Display*/>(&self, addr_to_read: usize) -> Result<T, String> {
        let mut buffer_vec : Vec<u8> = vec![0;std::mem::size_of::<T>()];
        if ReadProcessMemory(
            self.process_handle,
            addr_to_read as *const c_void,
            buffer_vec.as_mut_ptr() as _,
            std::mem::size_of::<T>(),
            None
        ).is_err() {
            return Err(String::from("Failed to read process' memory"));
        }
        // Create an uninitialized value of type T
        let mut result_value: std::mem::MaybeUninit<T> = std::mem::MaybeUninit::uninit();
        // Use copy_nonoverlapping to copy the bytes from the buffer vector to the target type
        std::ptr::copy_nonoverlapping(buffer_vec.as_ptr(), result_value.as_mut_ptr() as *mut u8, std::mem::size_of::<T>());
        // Convert from MaybeUninit<T> to T
        Ok(result_value.assume_init())
    }
    /// Read bytes from the process at the given location addr_to_read
    pub unsafe fn read_bytes(&self, addr_to_read: usize, size_to_read: usize) -> Result<Vec<u8>, String> {
        let mut buffer_vec : Vec<u8> = vec![0;size_to_read];
        if ReadProcessMemory(
            self.process_handle,
            addr_to_read as *const c_void,
            buffer_vec.as_mut_ptr() as _,
            size_to_read,
            None
        ).is_err() {
            return Err(String::from("Failed to read process' memory"));
        }
        Ok(buffer_vec)
    }
    /// Create allocation with size passed
    pub unsafe fn create_alloc_ex(&self, size: usize) -> Result<usize, String> {
        let alloc_ptr = VirtualAllocEx(self.process_handle,None, size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
        match alloc_ptr as usize != 0 {
            true => Ok(alloc_ptr as usize),
            false => Err(String::from("Failed to create allocation"))
        }
    }
    /// Inline hook a function and return the original bytes
    pub unsafe fn hook_fn(&self, address_to_hook: usize, detour_address: usize, size_to_rep: usize) -> Vec<u8> {
        // Suspend threads
        self.process_threads.suspend_threads();
        // Read old bytes & store it
        let old_bytes = self.read_bytes(address_to_hook, size_to_rep).unwrap();
        // Replace bytes with nop
        let nop_bytes : Vec<u8> = vec![0x90, size_to_rep as u8];
        self.write_bytes(address_to_hook, nop_bytes.as_slice()).unwrap();
        // Setup shellcode
        let mut hook_shellcode = dynasmrt::x86::Assembler::new().unwrap();
        dynasm!(hook_shellcode
            ; .arch x86
            ; mov eax, detour_address as _
            ; jmp eax
        );
        // Turn it into a byte array
        let shellcode_bytes = hook_shellcode.finalize().unwrap().to_vec();
        // Write shellcode to func
        self.write_bytes(address_to_hook, shellcode_bytes.as_slice()).unwrap();
        // Resume threads
        self.process_threads.resume_threads();
        // Return old bytes :-)
        old_bytes
    }
    /// Attempt to parse a function into bytes
    pub unsafe fn get_func_bytes(&self, address_to_parse: usize) -> Vec<u8> {
        let mut return_bytes: Vec<u8> = Vec::new();
        // Read every byte until 0xC3
        let mut inc = 0;
        loop {
            let curr_byte_to_read = address_to_parse + inc;
            let curr_byte = self.read::<u8>(curr_byte_to_read).unwrap();
            // Push to return vec
            //return_bytes.push(curr_byte);
            match curr_byte.clone() {
                0xE8 => {
                    // Check if previous byte is in ignore list
                    if return_bytes.last().unwrap().clone() == 0x83 {
                        return_bytes.push(curr_byte);
                        inc+=1;
                    }
                    else {
                        // Relative jmp/call
                        // Read four bytes after first inst
                        let call_rva = self.read::<usize>(curr_byte_to_read + 0x1).unwrap();
                        // Get the absolute by using wrapping since we're on x86
                        let call_absolute = call_rva.wrapping_add(curr_byte_to_read.wrapping_add(0x5));
                        // Assemble an absolute call
                        let mut new_call = dynasmrt::x86::Assembler::new().unwrap();
                        dynasm!(new_call
                            ; .arch x86
                            ; mov edx, call_absolute as _
                            ; call edx
                        );
                        // Push absolute call bytes onto the return bytes vector
                        return_bytes.extend_from_slice(&new_call.finalize().unwrap().to_vec());
                        // Skip the E8 00000000
                        inc += 5;
                    }
                }
                0xC3 => {
                    return_bytes.push(curr_byte.clone());
                    return return_bytes;
                },
                _ => {
                    return_bytes.push(curr_byte.clone());
                    inc+=1;
                },
            }
        }
    }
    /// Clone a func from bytes to a new allocation
    pub unsafe fn clone_func(&self, address_to_clone: usize) -> usize {
        // Create allocation
        let cloned_func_alloc = self.create_alloc_ex(0x1000).unwrap();
        println!("{cloned_func_alloc:#X}");
        // Get func bytes
        let func_bytes = self.get_func_bytes(address_to_clone);
        // Write bytes to alloc
        self.write_bytes(cloned_func_alloc, func_bytes.as_slice()).unwrap();
        // Return func to user
        cloned_func_alloc
    }
}
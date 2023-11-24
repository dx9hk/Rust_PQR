use dynasmrt::dynasm;
use dynasmrt::DynasmApi;
use crate::Process;

#[derive(Debug, Clone)]
pub struct WowCheats {
    wow_process: Process,
    frame_script_execute: usize,
    end_scene_ptr: usize,
}

impl WowCheats {
    pub unsafe fn new(end_scene_ptr: usize) -> Self {
        Self { wow_process: Process::find("Wow.exe"), frame_script_execute: 0x819210, end_scene_ptr}
    }
    pub unsafe fn second_run_string(&self, old_script_to_run: &str) {
        let script_to_run = old_script_to_run.replace("PQR", "dx9");
        // Allocate an area in memory for our return value
        let did_func_run_alloc = self.wow_process.create_alloc_ex(0x1000).unwrap();
        // Allocate memory for our shellcode
        let shellcode_alloc = self.wow_process.create_alloc_ex(0x200).unwrap();
        // Allocate memory for our string
        let string_alloc = self.wow_process.create_alloc_ex(script_to_run.len()).unwrap();
        // Write our string to our alloc
        self.wow_process.write_bytes(string_alloc, script_to_run.as_bytes()).unwrap();
        // Create our shellcode
        let mut exec_shellcode = dynasmrt::x86::Assembler::new().unwrap();
        dynasm!(exec_shellcode
            ; .arch x86
            ; mov eax, string_alloc as i32
            ; push 0
            ; push eax
            ; push eax
            ; mov eax, self.frame_script_execute as i32
            ; call eax
            ; add esp, 0xC
        );
        // finalise our shellcode
        let mut shellcode_bytes = exec_shellcode.finalize().unwrap().to_vec();
        // Read old bytes
        let old_bytes = self.wow_process.read_bytes(self.end_scene_ptr, 7).unwrap();
        // Add old bytes to our shellcode
        old_bytes.iter().for_each(|curr_byte| shellcode_bytes.push(*curr_byte));
        // Create another shellcode to jmp back to original function + 7 bytes
        let mut jmp_back_shellcode = dynasmrt::x86::Assembler::new().unwrap();
        dynasm!(jmp_back_shellcode
            ; .arch x86
            ; mov eax, (self.end_scene_ptr + 0x7) as i32
            ; mov DWORD [did_func_run_alloc as i32], 1
            ; jmp eax
        );
        jmp_back_shellcode.finalize().unwrap().to_vec().iter().for_each(|curr_byte| shellcode_bytes.push(*curr_byte));
        // Write this to our shellcode alloc
        self.wow_process.write_bytes(shellcode_alloc, shellcode_bytes.as_slice()).unwrap();
        // Hook func
        self.wow_process.hook_fn(self.end_scene_ptr,shellcode_alloc, 7);
        // Restore func
        while self.wow_process.read::<u8>(did_func_run_alloc).unwrap() != 1 {
            std::thread::yield_now();
        }
        self.wow_process.write_bytes(self.end_scene_ptr, old_bytes.as_slice()).unwrap();
    }

}
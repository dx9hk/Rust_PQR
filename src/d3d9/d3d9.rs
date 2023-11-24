use windows::core::s;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D9::{D3D_SDK_VERSION, D3DADAPTER_DEFAULT, D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3DDEVTYPE_HAL, D3DPRESENT_PARAMETERS, D3DSWAPEFFECT_DISCARD, Direct3DCreate9, IDirect3DDevice9};
use windows::Win32::UI::WindowsAndMessaging::FindWindowA;

#[derive(Debug)]
pub struct D3d9 {
    d3d9_vtable: Vec<*const usize>,
}

impl D3d9 {
    /// Constructor to create a dummy device and assign vtable
    pub unsafe fn new() -> Self {
        let d3d9 = Direct3DCreate9(D3D_SDK_VERSION).unwrap();
        let p_dummy_device: *mut IDirect3DDevice9 = std::ptr::null_mut();
        let window_hwnd = FindWindowA(None, s!("World of Warcraft"));
        if window_hwnd.0 <= HWND(0).0 {
            panic!("Error: Couldn't use FindWindow")
        }
        let mut d3dpp = D3DPRESENT_PARAMETERS {
            hDeviceWindow: window_hwnd,
            SwapEffect: D3DSWAPEFFECT_DISCARD,
            BackBufferCount: 1,
            ..Default::default()
        };


        let mut dummy_device_created = d3d9.CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL,
                                                             d3dpp.hDeviceWindow, D3DCREATE_SOFTWARE_VERTEXPROCESSING as u32,
                                                             std::mem::transmute(&d3dpp),
                                                             std::mem::transmute(&p_dummy_device));

        if let Err(e) = dummy_device_created {
            d3dpp.Windowed = !d3dpp.Windowed;
            dummy_device_created = d3d9.CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL,
                                                     d3dpp.hDeviceWindow, D3DCREATE_SOFTWARE_VERTEXPROCESSING as u32,
                                                     std::mem::transmute(&d3dpp),
                                                     std::mem::transmute(&p_dummy_device));
            if let Err(e) = dummy_device_created {
                panic!("Error: {e}");
            }
        }


        let v = std::slice::from_raw_parts((p_dummy_device as *const *const *const usize).read(), 119).to_vec();
        match v.is_empty() {
            true => panic!("Failed to dump d3d9 device addresses"),
            false => Self {d3d9_vtable: v}
        }
    }
    /// Return endscene via index
    pub fn get_endscene(&self) -> *const usize {
        self.d3d9_vtable[42]
    }
}



//! Barebones baseview egui plugin

#[macro_use]
extern crate vst;

use egui;
use egui::CtxRef;

use baseview::{Size, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Info, Plugin, PluginParameters};

use egui_baseview::{EguiWindow, Queue, RenderSettings, Settings};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use glicol::Engine;

use std::boxed::Box;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicPtr, Ordering};

const WINDOW_WIDTH: usize = 600;
const WINDOW_HEIGHT: usize = 800;

struct GlicolVSTPluginEditor {
    params: Arc<GlicolParams>,
    window_handle: Option<WindowHandle>,
    is_open: bool,
}

struct GlicolParams {
    code_ptr: AtomicPtr<u8>,
    code_len: AtomicUsize,
    has_update: AtomicBool,
}

struct GlicolVSTPlugin {
    params: Arc<GlicolParams>,
    engine: Engine<128>,
    editor: Option<GlicolVSTPluginEditor>,
}

struct VstParent(*mut ::std::ffi::c_void);

impl Editor for GlicolVSTPluginEditor {
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn size(&self) -> (i32, i32) {
        (WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
    }

    fn open(&mut self, parent: *mut ::std::ffi::c_void) -> bool {
        ::log::info!("Editor open");
        if self.is_open {
            return false;
        }
        
        self.is_open = true;

        let settings = Settings {
            window: WindowOpenOptions {
                title: String::from("Glicol VST"),
                size: Size::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
            },
            render_settings: RenderSettings::default(),
        };

        let mut code: String = "o: ~input >> mul 0.1;\n\n// o: sin 440;".to_owned();
        let window_handle = EguiWindow::open_parented(
            &VstParent(parent),
            settings,
            self.params.clone(),
            |_egui_ctx: &CtxRef, _queue: &mut Queue, _state: &mut Arc<GlicolParams>,| {},
            move |egui_ctx: &CtxRef, _queue: &mut Queue, state: &mut Arc<GlicolParams>,| {
                egui::Window::new("Glicol VST").show(egui_ctx, |ui| {

                    // if ui.input_mut()
                    // .consume_key(egui::Modifiers::default(), egui::Key::Enter)
                    // {
                    //     let hello_utf8;
                    //     unsafe {
                    //         hello_utf8 = code.as_bytes_mut();
                    //     }
                    //     // let mtlen = code.len();
                        
                    //     state.code_ptr.store(hello_utf8.as_mut_ptr(), Ordering::SeqCst);
                    //     state.code_len.store(code.len(), Ordering::SeqCst);
                    //     // std::mem::forget(code);
                    //     state.has_update.store(true, Ordering::SeqCst);
                    // };
                    ui.add(
                        egui::TextEdit::multiline(&mut code).code_editor()
                        .desired_rows(50)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                    );
                    
                    if ui.button("Run").clicked() {
                        let hello_utf8;
                        unsafe {
                            hello_utf8 = code.as_bytes_mut();
                        }
                        // let mtlen = code.len();
                        
                        state.code_ptr.store(hello_utf8.as_mut_ptr(), Ordering::SeqCst);
                        state.code_len.store(code.len(), Ordering::SeqCst);
                        // std::mem::forget(code);
                        state.has_update.store(true, Ordering::SeqCst);
                    }
                        
                });
            },
        );

        self.window_handle = Some(window_handle);
        true
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }

    fn close(&mut self) {
        self.is_open = false;
        if let Some(mut window_handle) = self.window_handle.take() {
            window_handle.close();
        }
    }
}

impl Default for GlicolParams {
    fn default() -> Self {
        let mut dummy = String::from("");
        let ptr;
        unsafe {
            ptr = dummy.as_bytes_mut().as_mut_ptr();
        }
        Self {
            code_ptr: AtomicPtr::<u8>::new(ptr),
            code_len: AtomicUsize::new(0),
            has_update: AtomicBool::new(false)
        }
    }
}

impl Default for GlicolVSTPlugin {
    fn default() -> Self {
        let params = Arc::new(GlicolParams::default());
        let mut engine = Engine::<128>::new();
        engine.update_with_code("o: ~input >> mul 0.1;");
        Self {
            params: params.clone(),
            engine: engine,
            editor: Some(GlicolVSTPluginEditor {
                params: params.clone(),
                window_handle: None,
                is_open: false,
            }),
        }
    }
}

impl Plugin for GlicolVSTPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Glicol VST".to_string(),
            vendor: "chaosprint".to_string(),
            unique_id: 88886666,
            version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            category: Category::Effect,
            ..Default::default()
        }
    }

    fn init(&mut self) {
        let log_folder = ::dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

        let log_file = ::std::fs::File::create(log_folder.join("GlicolVST.log")).unwrap();

        let log_config = ::simplelog::ConfigBuilder::new()
            .set_time_to_local(true)
            .build();

        let _ = ::simplelog::WriteLogger::init(simplelog::LevelFilter::Info, log_config, log_file);

        ::log_panics::init();

        ::log::info!("init");
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if self.params.has_update.load(Ordering::Acquire) {
            let ptr = self.params.code_ptr.load(Ordering::Acquire);
            let len = self.params.code_len.load(Ordering::Acquire);
            let encoded:&[u8] = unsafe { std::slice::from_raw_parts(ptr, len) };
            // std::mem::forget(ptr);
            let code = std::str::from_utf8(encoded.clone()).unwrap().to_owned();
            self.engine.update_with_code(&code);
            self.params.has_update.store(false, Ordering::Release);
        }

        let block_size: usize = buffer.samples();

        let (input, mut outputs) = buffer.split();
        let output_channels = outputs.len();
        let process_times = block_size / 128;

        // let mut out = vec![0.0; block_size];
        // let mut index = 0;

        for b in 0..process_times {
            let inp = vec![
                &input.get(0)[b*128..(b+1)*128], 
                &input.get(1)[b*128..(b+1)*128]
            ];

            let engine_out = self.engine.next_block(inp).0;
            // [0].clone();

            // for i in 0..128 {
            //     out[index] = o[i];
            //     index += 1;
            // }

            for chan_idx in 0..output_channels {
                let buff = outputs.get_mut(chan_idx);
                for n in 0..128 {
                    buff[b*128+n] = engine_out[chan_idx][n];
                }
            }
        }

        // for sample_idx in 0..block_size {
        //     for buf_idx in 0..output_channels {
        //         let buff = outputs.get_mut(buf_idx);
        //         buff[sample_idx] = out[sample_idx];
        //     }
        // }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for GlicolParams {
}

#[cfg(target_os = "macos")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::macos::MacOSHandle;

        RawWindowHandle::MacOS(MacOSHandle {
            ns_view: self.0 as *mut ::std::ffi::c_void,
            ..MacOSHandle::empty()
        })
    }
}

#[cfg(target_os = "windows")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::windows::WindowsHandle;

        RawWindowHandle::Windows(WindowsHandle {
            hwnd: self.0,
            ..WindowsHandle::empty()
        })
    }
}

#[cfg(target_os = "linux")]
unsafe impl HasRawWindowHandle for VstParent {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use raw_window_handle::unix::XcbHandle;

        RawWindowHandle::Xcb(XcbHandle {
            window: self.0 as u32,
            ..XcbHandle::empty()
        })
    }
}

plugin_main!(GlicolVSTPlugin);
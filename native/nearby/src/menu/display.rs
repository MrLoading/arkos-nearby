use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{ChildStdin, Command, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use serde_json::{Value, json};

use super::{Event, OwnedChild, log};
use crate::context::INSTALL;
use crate::system::{mkdir, monotonic, private_write, random_id};
use crate::{Result, require};

const KEYS: &str = "a = enter\nb = esc\nback = esc\nx = r\nstart = enter\nup = up\ndown = down\nleft = left\nright = right\nl1 = pageup\nr1 = pagedown\nl2 = pageup\nr2 = pagedown\nleft_analog_up = up\nleft_analog_down = down\nleft_analog_left = left\nleft_analog_right = right\nright_analog_up = up\nright_analog_down = down\nright_analog_left = left\nright_analog_right = right\ndeadzone_triggers = 32767\n";

struct Temporary(PathBuf);
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Delivery {
    screen: Option<u64>,
    event: Event,
}

#[derive(Clone)]
pub(super) struct Replies {
    screen: u64,
    sender: mpsc::Sender<Delivery>,
}

impl Replies {
    pub(super) fn send(&self, event: Event) {
        let _ = self.sender.send(Delivery {
            screen: Some(self.screen),
            event,
        });
    }
}

pub(super) struct Display {
    stdin: Option<ChildStdin>,
    gui: Option<OwnedChild>,
    mapper: OwnedChild,
    reader: Option<JoinHandle<()>>,
    sender: mpsc::Sender<Delivery>,
    receiver: mpsc::Receiver<Delivery>,
    stopping: Arc<AtomicBool>,
    signals: Vec<signal_hook::SigId>,
    screen: u64,
    input_epoch: f64,
    previous: Value,
    _directory: Option<Temporary>,
}

impl Display {
    pub(super) fn open() -> Result<Self> {
        let root = Path::new("/run/arkos-nearby-rust/ui");
        mkdir(root)?;
        let directory = Temporary(root.join(random_id()?));
        mkdir(&directory.0)?;
        let keys = directory.0.join("keys.gptk");
        private_write(&keys, KEYS.as_bytes())?;
        let logs = Path::new(INSTALL).join("logs");
        mkdir(&logs)?;
        log(json!({"phase":"gui-spawn"}))?;
        let mapper = OwnedChild(
            Command::new("/opt/inttools/gptokeyb")
                .args(["-c", keys.to_str().ok_or("Invalid key mapping path")?])
                .env("TERM", "linux")
                .env("LANG", "C.UTF-8")
                .env("LC_ALL", "C.UTF-8")
                .env(
                    "SDL_GAMECONTROLLERCONFIG_FILE",
                    "/opt/inttools/gamecontrollerdb.txt",
                )
                .stdout(Stdio::from(File::create(logs.join("native-mapper.log"))?))
                .stderr(Stdio::null())
                .spawn()?,
        );
        let gui = OwnedChild(
            Command::new(format!("{INSTALL}/arkos-nearby-gui"))
                .env("SDL_INPUT_LINUX_KEEP_KBD", "1")
                .env("TERM", "linux")
                .env("LANG", "C.UTF-8")
                .env("LC_ALL", "C.UTF-8")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::from(File::create(logs.join("native-gui.log"))?))
                .spawn()?,
        );
        let mut display = Self::connect(gui, mapper)?;
        display._directory = Some(directory);
        for signal in [libc::SIGTERM, libc::SIGHUP] {
            display.signals.push(signal_hook::flag::register(
                signal,
                display.stopping.clone(),
            )?);
        }
        Ok(display)
    }

    fn connect(mut gui: OwnedChild, mapper: OwnedChild) -> Result<Self> {
        let stdin = gui.0.stdin.take().ok_or("GUI has no input channel")?;
        let stdout = gui.0.stdout.take().ok_or("GUI has no output channel")?;
        let (sender, receiver) = mpsc::channel();
        let input_sender = sender.clone();
        let reader = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = Vec::new();
                let result = (&mut reader).take(16385).read_until(b'\n', &mut line);
                if !matches!(result, Ok(count) if count > 0 && count <= 16384) {
                    break;
                }
                if let Ok(value) = serde_json::from_slice::<Value>(&line) {
                    let event = if value == json!({"event":"frame_ready"}) {
                        Some(Event::FrameReady)
                    } else {
                        value
                            .as_str()
                            .filter(|s| {
                                matches!(
                                    *s,
                                    "up" | "down"
                                        | "left"
                                        | "right"
                                        | "confirm"
                                        | "back"
                                        | "refresh"
                                        | "page_up"
                                        | "page_down"
                                )
                            })
                            .map(|s| Event::Input(s.into(), monotonic()))
                    };
                    if let Some(event) = event {
                        if input_sender
                            .send(Delivery {
                                screen: None,
                                event,
                            })
                            .is_err()
                        {
                            return;
                        }
                    }
                }
            }
            let _ = input_sender.send(Delivery {
                screen: None,
                event: Event::Eof,
            });
        });
        Ok(Self {
            stdin: Some(stdin),
            gui: Some(gui),
            mapper,
            reader: Some(reader),
            sender,
            receiver,
            stopping: Arc::new(AtomicBool::new(false)),
            signals: Vec::new(),
            screen: 0,
            input_epoch: monotonic(),
            previous: Value::Null,
            _directory: None,
        })
    }

    pub(super) fn begin_screen(&mut self) -> Replies {
        self.screen += 1;
        self.input_epoch = monotonic();
        Replies {
            screen: self.screen,
            sender: self.sender.clone(),
        }
    }

    pub(super) fn stopped(&self) -> bool {
        self.stopping.load(Ordering::Relaxed)
    }

    pub(super) fn present(&mut self, view: Value) -> Result<()> {
        require(
            self.mapper.0.try_wait()?.is_none(),
            "The owned gamepad mapper stopped",
        )?;
        let gui = self.gui.as_mut().ok_or("The graphical interface stopped")?;
        require(
            gui.0.try_wait()?.is_none(),
            "The graphical interface stopped",
        )?;
        if view != self.previous {
            let stdin = self.stdin.as_mut().ok_or("GUI has no input channel")?;
            serde_json::to_writer(&mut *stdin, &view)?;
            stdin.write_all(b"\n")?;
            stdin.flush()?;
            self.previous = view;
        }
        Ok(())
    }

    pub(super) fn receive(&self) -> std::result::Result<Option<Event>, mpsc::RecvTimeoutError> {
        let delivery = self.receiver.recv_timeout(Duration::from_millis(20))?;
        if delivery.screen.is_some_and(|screen| screen != self.screen)
            || matches!(&delivery.event, Event::Input(_, at) if *at < self.input_epoch)
        {
            return Ok(None);
        }
        Ok(Some(delivery.event))
    }

    pub(super) fn previous(&self) -> &Value {
        &self.previous
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        for signal in self.signals.drain(..) {
            signal_hook::low_level::unregister(signal);
        }
        drop(self.stdin.take());
        drop(self.gui.take());
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn display() -> Display {
        let gui = OwnedChild(Command::new("sh").args(["-c", "printf '{\"event\":\"frame_ready\"}\\n'; while IFS= read -r line; do printf '\"confirm\"\\n'; done"])
            .stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap());
        let mapper = OwnedChild(Command::new("sleep").arg("30").spawn().unwrap());
        let display = Display::connect(gui, mapper).unwrap();
        assert!(matches!(next(&display), Event::FrameReady));
        display
    }

    fn next(display: &Display) -> Event {
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            match display.receive() {
                Ok(Some(event)) => return event,
                Ok(None) | Err(mpsc::RecvTimeoutError::Timeout) if Instant::now() < deadline => {}
                _ => panic!("The GUI did not deliver an event"),
            }
        }
    }

    #[test]
    fn screen_handoffs_keep_both_processes_and_close_them_at_session_end() {
        let mut display = display();
        let gui = display.gui.as_ref().unwrap().0.id();
        let mapper = display.mapper.0.id();
        for page in ["mode", "games", "choice", "host", "rooms", "mode"] {
            display.begin_screen();
            display.present(json!({"page":page})).unwrap();
            assert!(matches!(next(&display), Event::Input(action, _) if action == "confirm"));
            assert_eq!(display.gui.as_ref().unwrap().0.id(), gui);
            assert_eq!(display.mapper.0.id(), mapper);
        }
        drop(display);
        for pid in [gui, mapper] {
            assert_eq!(unsafe { libc::kill(pid as libc::pid_t, 0) }, -1);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::ESRCH)
            );
        }
    }

    #[test]
    fn late_workers_and_queued_old_input_do_not_cross_screen_boundaries() {
        let mut display = display();
        let old = display.begin_screen();
        let old_input = display.input_epoch;
        let current = display.begin_screen();
        old.send(Event::Status(0, json!({"ready":true})));
        old.send(Event::Scan(0, json!({"rooms":[]})));
        old.send(Event::Prepared(Err("stale".into())));
        old.send(Event::Action(
            crate::flow::Request {
                token: 1,
                operation: crate::flow::Operation::Create,
                payload: json!({}),
            },
            json!({"ready":true}),
            false,
        ));
        display
            .sender
            .send(Delivery {
                screen: None,
                event: Event::Input("back".into(), old_input - 1.0),
            })
            .unwrap();
        for _ in 0..5 {
            assert!(display.receive().unwrap().is_none());
        }
        current.send(Event::Status(0, json!({"ready":false})));
        assert!(matches!(next(&display), Event::Status(0, value) if value["ready"] == false));
    }
}

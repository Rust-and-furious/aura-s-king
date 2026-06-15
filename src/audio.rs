use std::process::Command;
use std::sync::{Arc, Mutex};
use std::process::Child;

pub fn play_sound(file_path: &str) {
    if cfg!(target_os = "windows") {
        if let Ok(abs_path) = std::fs::canonicalize(file_path) {
            let path_str = abs_path.to_string_lossy().replace("\\\\?\\", "");
            let cmd = format!(
                "Add-Type -AssemblyName PresentationCore; \
                 $player = New-Object System.Windows.Media.MediaPlayer; \
                 $player.Open('{}'); \
                 $player.Volume = 0.5; \
                 $player.Play(); \
                 Start-Sleep -Seconds 4",
                path_str
            );
            let _ = Command::new("powershell").args(["-c", &cmd]).spawn();
        } else {
            let cmd = format!(
                "Add-Type -AssemblyName PresentationCore; \
                 $player = New-Object System.Windows.Media.MediaPlayer; \
                 $player.Open('{}'); \
                 $player.Volume = 0.5; \
                 $player.Play(); \
                 Start-Sleep -Seconds 4",
                file_path
            );
            let _ = Command::new("powershell").args(["-c", &cmd]).spawn();
        }
    } else if cfg!(target_os = "linux") {
        let cmd = format!(
            "if command -v paplay >/dev/null 2>&1; then paplay \"{}\"; elif command -v aplay >/dev/null 2>&1; then aplay -q \"{}\"; fi",
            file_path, file_path
        );
        let _ = Command::new("sh").args(["-c", &cmd]).spawn();
    }
}

pub struct MusicHandle {
    child: Arc<Mutex<Option<Child>>>,
}

impl Drop for MusicHandle {
    fn drop(&mut self) {
        if let Ok(mut child_lock) = self.child.lock() {
            if let Some(mut child) = child_lock.take() {
                let pid = child.id();
                if cfg!(target_os = "linux") {
                    // Tuer les sous-processus (paplay, sleep) lancés par le shell
                    let _ = std::process::Command::new("pkill").args(["-P", &pid.to_string()]).status();
                }
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

pub fn play_music_loop(file_path: &str) -> MusicHandle {
    let mut child_opt = None;
    if cfg!(target_os = "windows") {
        if let Ok(abs_path) = std::fs::canonicalize(file_path) {
            let path_str = abs_path.to_string_lossy().replace("\\\\?\\", "");
            // On utilise MediaPlayer au lieu de SoundPlayer pour pouvoir gérer le volume (0.2 = 20%)
            let cmd = format!(
                "Add-Type -AssemblyName PresentationCore; \
                 $player = New-Object System.Windows.Media.MediaPlayer; \
                 $player.Open('{}'); \
                 $player.Volume = 0.2; \
                 $player.Play(); \
                 while ($true) {{ \
                     Start-Sleep -Milliseconds 500; \
                     if ($player.Position -ge $player.NaturalDuration.TimeSpan -and $player.NaturalDuration.TimeSpan.TotalMilliseconds -gt 0) {{ \
                         $player.Position = [TimeSpan]::Zero; \
                         $player.Play(); \
                     }} \
                 }}",
                path_str
            );
            child_opt = Command::new("powershell").args(["-c", &cmd]).spawn().ok();
        } else {
            let cmd = format!(
                "Add-Type -AssemblyName PresentationCore; \
                 $player = New-Object System.Windows.Media.MediaPlayer; \
                 $player.Open('{}'); \
                 $player.Volume = 0.2; \
                 $player.Play(); \
                 while ($true) {{ \
                     Start-Sleep -Milliseconds 500; \
                     if ($player.Position -ge $player.NaturalDuration.TimeSpan -and $player.NaturalDuration.TimeSpan.TotalMilliseconds -gt 0) {{ \
                         $player.Position = [TimeSpan]::Zero; \
                         $player.Play(); \
                     }} \
                 }}",
                file_path
            );
            child_opt = Command::new("powershell").args(["-c", &cmd]).spawn().ok();
        }
    } else if cfg!(target_os = "linux") {
        // paplay gère le volume : 65536 = 100%, 16384 = 25%
        let cmd = format!(
            "while true; do \
                if command -v paplay >/dev/null 2>&1; then paplay --volume=45000 \"{}\"; \
                elif command -v aplay >/dev/null 2>&1; then aplay -q \"{}\"; \
                else sleep 5; fi; \
                sleep 1; \
            done",
            file_path, file_path
        );
        child_opt = Command::new("sh").args(["-c", &cmd]).spawn().ok();
    }
    
    MusicHandle {
        child: Arc::new(Mutex::new(child_opt)),
    }
}

use std::process::Command;

pub fn play_sound(file_path: &str) {
    if cfg!(target_os = "windows") {
        if let Ok(abs_path) = std::fs::canonicalize(file_path) {
            let path_str = abs_path.to_string_lossy().replace("\\\\?\\", "");
            // Utilise MediaPlayer de PresentationCore pour pouvoir régler le volume à 50% (0.5).
            // Le Start-Sleep maintient le processus PowerShell ouvert le temps de jouer l'effet (4 secondes).
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
    }
}

pub fn play_music_loop(file_path: &str) {
    if cfg!(target_os = "windows") {
        if let Ok(abs_path) = std::fs::canonicalize(file_path) {
            let path_str = abs_path.to_string_lossy().replace("\\\\?\\", "");
            let cmd = format!(
                "$player = New-Object Media.SoundPlayer '{}'; $player.PlayLooping(); while ($true) {{ Start-Sleep 5 }}",
                path_str
            );
            let _ = Command::new("powershell").args(["-c", &cmd]).spawn();
        } else {
            let cmd = format!(
                "$player = New-Object Media.SoundPlayer '{}'; $player.PlayLooping(); while ($true) {{ Start-Sleep 5 }}",
                file_path
            );
            let _ = Command::new("powershell").args(["-c", &cmd]).spawn();
        }
    }
}

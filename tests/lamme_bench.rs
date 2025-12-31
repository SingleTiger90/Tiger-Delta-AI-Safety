let mut count: u64 = 0;

// ... в середині основного циклу

while start.elapsed() < Duration::from_secs(profile.duration_secs) {
    for _ in 0..profile.burst_size {
        // ... генерація та відправка пакету ...

        count += 1;

        // Кожні 4096 пакетів — мікропауза для хвильового резонансу
        if count % 4096 == 0 {
            std::thread::sleep(Duration::from_micros(50));
        }
    }

    std::thread::sleep(Duration::from_millis(profile.burst_pause_ms));
}

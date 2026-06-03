#!/usr/bin/env bash

# This script generates WAV files of different lengths (1s, 5s, 20s, 30s, 1min, 3min)
# that conform to the project requirements:
# - Mono (1 channel)
# - 48000 Hz sample rate
# - 16-bit PCM (2 bytes per sample)

# Output directory
OUT_DIR="test_wavs"
mkdir -p "$OUT_DIR"

echo "Generating WAV files in directory '$OUT_DIR'..."

python3 -c "
import wave
import struct
import math
import os

def create_wav(filename, duration_sec, sample_rate=48000, frequency=440.0):
    filepath = os.path.join('$OUT_DIR', filename)
    num_samples = int(duration_sec * sample_rate)
    
    with wave.open(filepath, 'wb') as wav_file:
        wav_file.setnchannels(1)      # Mono
        wav_file.setsampwidth(2)      # 16-bit PCM (2 bytes)
        wav_file.setframerate(sample_rate)
        
        # Write a simple 440Hz sine wave
        for i in range(num_samples):
            t = float(i) / sample_rate
            value = int(16384.0 * math.sin(2.0 * math.pi * frequency * t))
            data = struct.pack('<h', value)
            wav_file.writeframesraw(data)
            
    print(f'Created {filename}: {duration_sec}s ({num_samples} samples, {os.path.getsize(filepath)} bytes)')

# Generate the requested durations
create_wav('1s.wav', 1)
create_wav('5s.wav', 5)
create_wav('20s.wav', 20)
create_wav('30s.wav', 30)
create_wav('1min.wav', 60)
create_wav('3min.wav', 180)
"

echo "All WAV files generated successfully."

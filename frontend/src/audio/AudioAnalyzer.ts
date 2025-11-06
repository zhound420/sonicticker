import type { AudioFrame } from '../types';

export class AudioAnalyzer {
  private context: AudioContext;
  private gain: GainNode;
  private analyser: AnalyserNode;
  private waveformBuffer: Float32Array<ArrayBuffer>;
  private frequencyBuffer: Uint8Array<ArrayBuffer>;
  private nextPlaybackTime = 0;

  constructor(sampleRate = 44_100) {
    this.context = new AudioContext({ sampleRate });
    this.gain = this.context.createGain();
    this.analyser = this.context.createAnalyser();
    this.analyser.fftSize = 2048;
    this.gain.connect(this.analyser);
    this.analyser.connect(this.context.destination);

    this.waveformBuffer = new Float32Array<ArrayBuffer>(
      new ArrayBuffer(
        this.analyser.fftSize * Float32Array.BYTES_PER_ELEMENT,
      ),
    );
    this.frequencyBuffer = new Uint8Array<ArrayBuffer>(
      new ArrayBuffer(this.analyser.frequencyBinCount),
    );
  }

  setVolume(level: number) {
    this.gain.gain.value = Math.min(Math.max(level, 0.0), 1.0);
  }

  getWaveform(): Float32Array {
    this.analyser.getFloatTimeDomainData(this.waveformBuffer);
    return this.waveformBuffer;
  }

  getFrequencies(): Uint8Array {
    this.analyser.getByteFrequencyData(this.frequencyBuffer);
    return this.frequencyBuffer;
  }

  async playFrame(frame: AudioFrame) {
    if (this.context.state === 'suspended') {
      await this.context.resume();
    }

    const { payload, metadata } = frame;
    const interleaved = new Float32Array(payload);
    const channels = metadata.channels;
    const frameCount = interleaved.length / channels;
    const audioBuffer = this.context.createBuffer(
      channels,
      frameCount,
      metadata.sample_rate,
    );

    for (let channel = 0; channel < channels; channel += 1) {
      const channelData = audioBuffer.getChannelData(channel);
      let writeIndex = 0;
      for (let i = channel; i < interleaved.length; i += channels) {
        channelData[writeIndex] = interleaved[i];
        writeIndex += 1;
      }
    }

    const source = this.context.createBufferSource();
    source.buffer = audioBuffer;
    source.connect(this.gain);

    const now = this.context.currentTime;
    const startAt = Math.max(now, this.nextPlaybackTime);
    source.start(startAt);

    const durationSeconds = metadata.frames / metadata.sample_rate;
    this.nextPlaybackTime = startAt + durationSeconds;
  }

  get audioContext() {
    return this.context;
  }
}

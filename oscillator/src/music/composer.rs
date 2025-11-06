use chrono::Utc;
use tunes::prelude::*;

use crate::models::{AudioChunk, MusicalParams};

use super::styles::CompositionStyle;

pub struct MarketComposer {
    sample_rate: u32,
    bars_per_chunk: usize,
}

impl MarketComposer {
    pub fn new(sample_rate: u32, bars_per_chunk: usize) -> Self {
        Self {
            sample_rate,
            bars_per_chunk,
        }
    }

    pub fn render_chunk(
        &self,
        params: &MusicalParams,
        style: CompositionStyle,
    ) -> anyhow::Result<AudioChunk> {
        let tempo = Tempo::new(params.tempo as f32);
        let mut comp = Composition::new(tempo);

        match style {
            CompositionStyle::Electronic => self.compose_electronic(&mut comp, params),
            CompositionStyle::Ambient => self.compose_ambient(&mut comp, params),
            CompositionStyle::Orchestral => self.compose_orchestral(&mut comp, params),
            CompositionStyle::Rock => self.compose_rock(&mut comp, params),
        }

        let mut mixer = comp.into_mixer();
        let buffer = mixer.render_to_buffer(self.sample_rate as f32);
        let frames = buffer.len() / 2;
        let mut samples = Vec::with_capacity(buffer.len() * std::mem::size_of::<f32>());
        for sample in buffer {
            samples.extend_from_slice(&sample.to_le_bytes());
        }

        Ok(AudioChunk {
            samples,
            frames,
            channels: 2,
            sample_rate: self.sample_rate,
            timestamp: Utc::now(),
        })
    }

    fn compose_electronic(&self, comp: &mut Composition, params: &MusicalParams) {
        let quarter = comp.tempo().quarter_note();
        let sixteenth = comp.tempo().sixteenth_note();

        let mut lead = comp.instrument("lead", &Instrument::synth_lead());
        lead = lead
            .filter(Filter::low_pass(2400.0, 0.7))
            .reverb(Reverb::new(0.4, 0.6, params.reverb_mix))
            .delay(Delay::new(0.375, 0.3, 0.4));

        for _ in 0..self.bars_per_chunk {
            for note in &params.melody_notes {
                lead = lead.note(&[*note], quarter * 0.5).wait(0.05);
            }
        }

        let mut bass = comp.instrument("bass", &Instrument::sub_bass());
        bass = bass.distortion(Distortion::new(
            1.0 + params.distortion * 4.0,
            params.distortion,
        ));

        for _ in 0..self.bars_per_chunk {
            bass = bass.note(&[params.bass_note], quarter);
        }

        let kick_pattern = self.kick_pattern(params.volume_intensity);
        comp.track("drums")
            .drum_grid(16, sixteenth)
            .kick(&kick_pattern)
            .snare(&[4, 12])
            .hihat(&[2, 6, 10, 14]);
    }

    fn compose_orchestral(&self, comp: &mut Composition, params: &MusicalParams) {
        let quarter = comp.tempo().quarter_note();
        let half = quarter * 2.0;

        let mut strings = comp.instrument("strings", &Instrument::strings());
        strings = strings
            .filter(Filter::low_pass(2200.0, 0.8))
            .reverb(Reverb::new(0.6, 0.5, (params.reverb_mix * 1.2).min(1.0)));

        for _ in 0..self.bars_per_chunk {
            for note in &params.melody_notes {
                strings = strings.notes(&[*note, *note * 1.25], half);
            }
        }

        let mut brass = comp.instrument("brass", &Instrument::brass());
        brass = brass.tremolo(Tremolo::new(
            quarter.recip(),
            (params.distortion + 0.2).min(0.8),
        ));

        for _ in 0..self.bars_per_chunk {
            brass = brass.note(&[params.bass_note * 0.5], quarter * 2.0);
        }

        comp.track("timpani")
            .drum_grid(8, quarter)
            .kick(&[0, 4])
            .snare(&[2, 6]);
    }

    fn compose_ambient(&self, comp: &mut Composition, params: &MusicalParams) {
        let whole = comp.tempo().whole_note();

        let mut pad = comp.instrument("pad", &Instrument::ambient_pad());
        pad = pad
            .filter(Filter::low_pass(1800.0, 0.9))
            .reverb(Reverb::new(0.8, 0.7, (params.reverb_mix + 0.2).min(1.0)))
            .chorus(Chorus::new(0.3, 0.002, 0.4));

        for _ in 0..self.bars_per_chunk {
            pad = pad.notes(&self.pad_chord(params), whole);
        }
    }

    fn compose_rock(&self, comp: &mut Composition, params: &MusicalParams) {
        let quarter = comp.tempo().quarter_note();

        let mut guitar = comp.instrument("guitar", &Instrument::electric_guitar_distorted());
        guitar = guitar.distortion(Distortion::new(
            1.0 + (params.distortion + 0.2) * 5.0,
            (params.distortion + 0.2).min(1.0),
        ));

        for _ in 0..self.bars_per_chunk {
            for note in &params.melody_notes {
                guitar = guitar.note(&[*note], quarter * 0.5);
            }
        }

        let mut bass = comp.instrument("bass_guitar", &Instrument::funk_bass());
        for _ in 0..self.bars_per_chunk {
            bass = bass.note(&[params.bass_note], quarter);
        }

        let sixteen = comp.tempo().sixteenth_note();
        let hats: Vec<usize> = (0..16).step_by(2).collect();
        comp.track("rock_drums")
            .drum_grid(16, sixteen)
            .kick(&[0, 8])
            .snare(&[4, 12])
            .hihat(&hats);
    }

    fn pad_chord(&self, params: &MusicalParams) -> Vec<f32> {
        if params.melody_notes.is_empty() {
            return vec![C4, E4, G4];
        }
        let root = params.melody_notes[0];
        vec![root, root * 1.25, root * 1.5]
    }

    fn kick_pattern(&self, volume_ratio: f64) -> Vec<usize> {
        if volume_ratio < 0.7 {
            vec![0, 8]
        } else if volume_ratio < 1.3 {
            vec![0, 4, 8, 12]
        } else {
            vec![0, 3, 4, 7, 8, 11, 12, 15]
        }
    }
}

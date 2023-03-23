enum SampleState {
    Trigger{sample: usize, pitch: u8},
    Continue,
    Off,
}

enum Modulator {
    Nil,
}

struct TrackStep {
    state: SampleState,
    modulate: Modulator,
}

struct SeqBlock {
    seq: Vec<Vec<TrackStep>> //1st dim = list of tracks; 2nd dim = steps w/n a track
}

struct ProgramSeq {
    seq_choices: Vec<SeqBlock>,
    song_seq: Vec<usize>,
    inter_onset: std::time::Duration,
}
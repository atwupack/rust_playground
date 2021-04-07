

pub enum Interval {
    PerfectUnison,
    DiminishedSecond,
    MinorSecond,
    AugmentedUnison,
    Semitone,
    MajorSecond,
    DiminishedThird,
    Tone,
    MinorThird,
    AugmentedSecond,
    MajorThird,
    DiminishedFourth,
    PerfectFourth,
    AugmentedThird,
    DiminishedFifth,
    AugmentedFourth,
    Tritone,
    PerfectFifth,
    DiminishedSixth,
    MinorSixth,
    AugmentedFifth,
    MajorSixth,
    DiminishedSeventh,
    MinorSeventh,
    AugmentedSixth,
    MajorSeventh,
    DiminishedOctave,
    PerfectOctave,
    AugmentedSeventh,
}

impl Interval {
    pub fn semitones(&self) -> u8 {
        match self {
            Interval::PerfectUnison | Interval::DiminishedSecond => 0,
            Interval::MinorSecond | Interval::AugmentedUnison | Interval::Semitone  => 1,
            Interval::MajorSecond | Interval::DiminishedThird | Interval::Tone => 2,
            Interval::MinorThird | Interval::AugmentedSecond => 3,
            Interval::MajorThird | Interval::DiminishedFourth => 4,
            Interval::PerfectFourth | Interval::AugmentedThird => 5,
            Interval::DiminishedFifth | Interval::AugmentedFourth | Interval::Tritone => 6,
            Interval::PerfectFifth | Interval::DiminishedSixth => 7,
            Interval::MinorSixth | Interval::AugmentedFifth => 8,
            Interval::MajorSixth | Interval::DiminishedSeventh => 9,
            Interval::MinorSeventh | Interval::AugmentedSixth => 10,
            Interval::MajorSeventh | Interval::DiminishedOctave => 11,
            Interval::PerfectOctave | Interval::AugmentedSeventh => 12,
        }
    }
}
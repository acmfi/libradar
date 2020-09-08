use crate::apk::{Apk, DexFile};

pub enum AvailableAnalysis {
    LibradarExact,
    LibradarFuzzy,
}

pub fn create_analysis(analysis_type: AvailableAnalysis) -> Box<dyn Analysis> {
    match analysis_type {
        AvailableAnalysis::LibradarExact => Box::new(LibradarExactAnalysis {}),
        AvailableAnalysis::LibradarFuzzy => Box::new(LibradarFuzzyAnalysis {}),
    }
}

pub trait Analysis {
    /// Return type if a todo
    fn analyze_apk(&self, apk: Apk) {
        for dex in apk.dex_files {
            self.analyze_dex(dex);
        }
    }

    fn analyze_dex(&self, dex: DexFile);
}

pub struct LibradarExactAnalysis {}

impl Analysis for LibradarExactAnalysis {
    fn analyze_dex(&self, dex: DexFile) {
        todo!();
    }
}

pub struct LibradarFuzzyAnalysis {}

impl Analysis for LibradarFuzzyAnalysis {
    fn analyze_dex(&self, dex: DexFile) {
        todo!();
    }
}

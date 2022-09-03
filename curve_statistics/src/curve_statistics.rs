use incremental_statistics::incremental_statistics;

pub struct CurveStatistics {
    points: Vec<incremental_statistics::IncrementalStatistics>,
    count: usize,
}

impl CurveStatistics {
    pub fn new(length: usize) -> CurveStatistics {
        CurveStatistics {
            points: vec![incremental_statistics::IncrementalStatistics::new(); length],
            count: 0usize,
        }
    }

    pub fn mean(&self, index: usize) -> f64 {
        self.points[index].mean()
    }

    pub fn means(&self) -> Vec<f64> {
        let mut means: Vec<f64> = vec![0.0; points.len()];
        for i in 0..self.points.len() {
            means[i] = self.points[i].mean();
        }
        means
    }

    pub fn variance(&self, index: usize) -> f64 {
        self.points[index].variance()
    }

    pub fn variances(&self) -> Vec<f64> {
        let mut variances: Vec<f64> = vec![0.0; points.len()];
        for i in 0..self.points.len() {
            variances[i] = self.points[i].variance();
        }
        variances
    }

    pub fn un_variance(&self, index: usize) -> f64 {
        self.points[index].un_variance()
    }

    pub fn un_variances(&self) -> Vec<f64> {
        let mut un_variances: Vec<f64> = vec![0.0; points.len()];
        for i in 0..self.points.len() {
            un_variances[i] = self.points[i].un_variance();
        }
        un_variances
    }

    pub fn standard_deviation(&mut self, index: usize) -> f64 {
        self.points[i].standard_deviation()
    }

    pub fn standard_deviations(&mut self) -> Vec<f64> {
        let mut standard_deviations: Vec<f64> = vec![0.0; points.len()];
        for i in 0..self.points.len() {
            standard_deviations[i] = self.points[i].un_standard_deviation();
        }
        standard_deviations
    }

    pub fn un_standard_deviation(&mut self, index: usize) -> f64 {
        self.points[index].un_standard_deviation()
    }

    pub fn un_standard_deviations(&mut self) -> Vec<f64> {
        let mut un_standard_deviations: Vec<f64> = vec![0.0; points.len()];
        for i in 0..self.points.len() {
            un_standard_deviations[i] = self.points[i].un_standard_deviation();
        }
        un_standard_deviations
    }

    pub fn upper(&self, index: usize) -> f64 {
        self.points[index].upper()
    }

    pub fn uppers(&mut self) -> Vec<f64> {
        let mut uppers: Vec<f64> = vec![0.0; points.len()];
        for i in 0..self.points.len() {
            uppers[i] = self.points[i].upper();
        }
        uppers
    }

    pub fn lower(&mut self, index: usize) {
        self.points[i].lower()
    }

    pub fn lowers(&mut self) -> Vec<f64> {
        let mut lowers: Vec<f64> = vec![0.0; points.len()];
        for i in 0..self.points.len() {
            lowers[i] = self.points[i].lower();
        }
        lowers
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn clear(&mut self) {
        for i in 0..self.points.len() {
            self.points[i].clear();
        }
    }
}

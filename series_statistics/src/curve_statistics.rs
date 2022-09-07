use incremental_statistics::incremental_statistics;

pub struct CurveStatistics {
    series: Vec<incremental_statistics::IncrementalStatistics>,
    count: usize,
}

impl CurveStatistics {
    pub fn new(length: usize) -> CurveStatistics {
        CurveStatistics {
            series: vec![incremental_statistics::IncrementalStatistics::new(); length],
            count: 0usize,
        }
    }

    pub fn mean(&self, index: usize) -> f64 {
        self.series[index].mean()
    }

    pub fn means(&self) -> Vec<f64> {
        let mut means: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            means[i] = self.series[i].mean();
        }
        means
    }

    pub fn variance(&self, index: usize) -> f64 {
        self.series[index].variance()
    }

    pub fn variances(&self) -> Vec<f64> {
        let mut variances: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            variances[i] = self.series[i].variance();
        }
        variances
    }

    pub fn un_variance(&self, index: usize) -> f64 {
        self.series[index].un_variance()
    }

    pub fn un_variances(&self) -> Vec<f64> {
        let mut un_variances: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            un_variances[i] = self.series[i].un_variance();
        }
        un_variances
    }

    pub fn standard_deviation(&mut self, index: usize) -> f64 {
        self.series[index].standard_deviation()
    }

    pub fn standard_deviations(&mut self) -> Vec<f64> {
        let mut standard_deviations: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            standard_deviations[i] = self.series[i].standard_deviation();
        }
        standard_deviations
    }

    pub fn un_standard_deviation(&mut self, index: usize) -> f64 {
        self.series[index].un_standard_deviation()
    }

    pub fn un_standard_deviations(&mut self) -> Vec<f64> {
        let mut un_standard_deviations: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            un_standard_deviations[i] = self.series[i].un_standard_deviation();
        }
        un_standard_deviations
    }

    pub fn upper(&mut self, index: usize) -> f64 {
        self.series[index].upper()
    }

    pub fn uppers(&mut self) -> Vec<f64> {
        let mut uppers: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            uppers[i] = self.series[i].upper();
        }
        uppers
    }

    pub fn un_upper(&mut self, index: usize) -> f64 {
        self.series[index].un_upper()
    }

    pub fn un_uppers(&mut self) -> Vec<f64> {
        let mut un_uppers: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            un_uppers[i] = self.series[i].un_upper();
        }
        un_uppers
    }

    pub fn lower(&mut self, index: usize) -> f64 {
        self.series[index].lower()
    }

    pub fn lowers(&mut self) -> Vec<f64> {
        let mut lowers: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            lowers[i] = self.series[i].lower();
        }
        lowers
    }

    pub fn un_lower(&mut self, index: usize) -> f64 {
        self.series[index].un_lower()
    }

    pub fn un_lowers(&mut self) -> Vec<f64> {
        let mut un_lowers: Vec<f64> = vec![0.0; self.series.len()];
        for i in 0..self.series.len() {
            un_lowers[i] = self.series[i].un_lower();
        }
        un_lowers
    }

    pub fn len(&self) -> usize {
        self.series.len()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn add(&mut self, series: &Vec<f64>) {
        for i in 0..series.len() {
            self.series[i].add(series[i]);
        }
        self.count += 1;
    }

    pub fn add_bulk(&mut self, series_bulk: &Vec<Vec<f64>>) {
        for i in series_bulk {
            self.add(i);
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.series.len() {
            self.series[i].clear();
        }
        self.count = 0;
    }
}

pub mod series_statistics;

#[cfg(test)]
mod tests {
    use crate::series_statistics;

    #[test]
    fn test_add() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);
        assert_eq!(cur.len(), length);
    }

    #[test]
    fn test_add_bulk() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        let matrix: Vec<Vec<f64>> = vec![
            vec![1.0f64; length],
            vec![2.0f64; length],
            vec![3.0f64; length],
            vec![4.0f64; length],
            vec![5.0f64; length],
        ];

        cur.add_bulk(&matrix);
        assert_eq!(cur.count(), length);
        assert_eq!(cur.len(), length);
        for i in 0..length {
            assert!((cur.mean(i) - 3.0) < 1e-6);
        }
    }

    #[test]
    fn test_mean() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let means = cur.means();
        for i in 0..length {
            assert!((cur.mean(i) - 3.0).abs() < 1e-6);
            assert!((means[i] - 3.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_sum() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);
        let sums = cur.sums();
        for i in 0..length {
            assert!((cur.sum(i) - 3.0 * length as f64).abs() < 1e-6);
            assert!((sums[i] - 3.0 * length as f64).abs() < 1e-6);
        }
    }

    #[test]
    fn test_variance() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let variances = cur.variances();
        for i in 0..length {
            assert!((cur.variance(i) - 2.0f64).abs() < 1e-6);
            assert!((variances[i] - 2.0f64).abs() < 1e-6);
        }
    }

    #[test]
    fn test_un_variance() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let un_variances = cur.un_variances();
        for i in 0..length {
            assert!((cur.un_variance(i) - 2.5f64).abs() < 1e-6);
            assert!((un_variances[i] - 2.5f64).abs() < 1e-6);
        }
    }

    #[test]
    fn test_standard_deviation() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let standard_deviations = cur.standard_deviations();
        for i in 0..length {
            assert!((cur.standard_deviation(i) - 2.0f64.sqrt()).abs() < 1e-6);
            assert!((standard_deviations[i] - 2.0f64.sqrt()).abs() < 1e-6);
        }
    }

    #[test]
    fn test_un_standard_deviation() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let un_standard_deviations = cur.un_standard_deviations();
        for i in 0..length {
            assert!((cur.un_standard_deviation(i) - 2.5f64.sqrt()).abs() < 1e-6);
            assert!((un_standard_deviations[i] - 2.5f64.sqrt()).abs() < 1e-6);
        }
    }

    #[test]
    fn test_upper() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let uppers = cur.uppers();
        for i in 0..length {
            assert!((cur.upper(i) - (cur.mean(i) + 2.0f64.sqrt())).abs() < 1e-6);
            assert!((uppers[i] - (cur.mean(i) + 2.0f64.sqrt())).abs() < 1e-6);
        }
    }

    #[test]
    fn test_un_upper() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let un_uppers = cur.un_uppers();
        for i in 0..length {
            assert!((cur.un_upper(i) - (cur.mean(i) + 2.5f64.sqrt())).abs() < 1e-6);
            assert!((un_uppers[i] - (cur.mean(i) + 2.5f64.sqrt())).abs() < 1e-6);
        }
    }

    #[test]
    fn test_lower() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let lowers = cur.lowers();
        for i in 0..length {
            assert!((cur.lower(i) - (cur.mean(i) - 2.0f64.sqrt())).abs() < 1e-6);
            assert!((lowers[i] - (cur.mean(i) - 2.0f64.sqrt())).abs() < 1e-6);
        }
    }

    #[test]
    fn test_un_lower() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);

        let un_lowers = cur.un_lowers();
        for i in 0..length {
            assert!((cur.un_lower(i) - (cur.mean(i) - 2.5f64.sqrt())).abs() < 1e-6);
            assert!((un_lowers[i] - (cur.mean(i) - 2.5f64.sqrt())).abs() < 1e-6);
        }
    }

    #[test]
    fn test_clear() {
        let length: usize = 5;
        let mut cur = series_statistics::SeriesStatistics::new(length);

        for i in 1..=length {
            let arr: Vec<f64> = vec![i as f64; length];
            cur.add(&arr);
        }
        assert_eq!(cur.count(), length);
        cur.clear();
        assert_eq!(cur.count(), 0usize);
        for i in 0..length {
            assert!(cur.mean(i) < 1e-6);
        }
    }
}

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use polars::prelude::*;

mod backtest;
mod types;
mod utils;

use crate::backtest::WeightBacktest;
use crate::types::*;

#[pyclass]
struct PyWeightBacktest {
    inner: WeightBacktest,
}

#[pymethods]
impl PyWeightBacktest {
    #[new]
    fn new(df: &PyAny, digits: u32, weight_type: String, kwargs: Option<&PyDict>) -> PyResult<Self> {
        // Convert Python DataFrame to Polars DataFrame
        let df = utils::py_df_to_polars(df)?;

        // Process kwargs
        let fee_rate = kwargs.and_then(|k| k.get_item("fee_rate").expect("REASON"))
            .map(|f| f.extract::<f64>())
            .transpose()?
            .unwrap_or(0.0002);

        let yearly_days = kwargs.and_then(|k| k.get_item("yearly_days").expect("REASON"))
            .map(|f| f.extract::<u32>())
            .transpose()?
            .unwrap_or(252);

        let n_jobs = kwargs.and_then(|k| k.get_item("n_jobs").expect("REASON"))
            .map(|f| f.extract::<usize>())
            .transpose()?
            .unwrap_or(1);

        // Validate input
        if df.is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err("DataFrame is empty"));
        }

        // Check for null values
        if df.null_count().iter().any(|x| x.null_count() > 0) {
            return Err(pyo3::exceptions::PyValueError::new_err("DataFrame contains null values"));
        }

        // Create input
        let input = BacktestInput {
            df,
            digits,
            weight_type,
            fee_rate,
            yearly_days,
            n_jobs,
        };

        Ok(Self {
            inner: WeightBacktest::new(input),
        })
    }

    fn backtest(&self) -> PyResult<PyObject> {
        let result = self.inner.run();
        
        Python::with_gil(|py| {

            // Convert result to Python dict
            let dict = PyDict::new(py);

            // Convert symbol results
            let symbol_results = PyDict::new(py);
            for (symbol, res) in &result.symbol_results {
                let symbol_dict = PyDict::new(py);

                // Convert daily results
                let daily_list = PyList::new(py, res.daily.iter().map(|d| {
                    let daily_dict = PyDict::new(py);
                    daily_dict.set_item("date", &d.date)?;
                    daily_dict.set_item("symbol", &d.symbol)?;
                    daily_dict.set_item("edge", d.edge)?;
                    // ... set other fields ...
                    Ok(daily_dict.into_py(py))
                }).collect::<PyResult<Vec<PyObject>>>()?);
                symbol_dict.set_item("daily", daily_list)?;

                // Convert pairs
                let pairs_list = PyList::new(py, res.pairs.iter().map(|p| {
                    let pair_dict = PyDict::new(py);
                    pair_dict.set_item("symbol", &p.symbol)?;
                    pair_dict.set_item("direction", &p.direction)?;
                    // ... set other fields ...
                    Ok(pair_dict.into_py(py))
                }).collect::<PyResult<Vec<PyObject>>>()?);
                symbol_dict.set_item("pairs", pairs_list)?;

                symbol_results.set_item(symbol, symbol_dict)?;
            }
            dict.set_item("symbol_results", symbol_results)?;

            // Convert daily returns
            let daily_returns_list = PyList::new(py, result.daily_returns.iter().map(|dr| {
                let dr_dict = PyDict::new(py);
                dr_dict.set_item("date", &dr.date)?;
                dr_dict.set_item("symbol", &dr.symbol)?;
                dr_dict.set_item("return", dr.return_)?;
                Ok(dr_dict.into_py(py))
            }).collect::<PyResult<Vec<PyObject>>>()?);
            dict.set_item("daily_returns", daily_returns_list)?;

            // Convert stats
            let stats_dict = PyDict::new(py);
            for (k, v) in &result.stats {
                stats_dict.set_item(k, v)?;
            }
            dict.set_item("stats", stats_dict)?;

            Ok(dict.into())
        })

    }
    
    #[getter]
    fn stats(&self) -> PyResult<PyObject> {
        let result = self.inner.run();
        Python::with_gil(|py| {
            let stats_dict = PyDict::new(py);
            for (k, v) in &result.stats {
                stats_dict.set_item(k, v)?;
            }
            Ok(stats_dict.into())
        })
    }

    // Implement other properties and methods...
}

#[pymodule]
fn weight_backtest_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWeightBacktest>()?;
    m.add("__version__", "20241205")?;
    Ok(())
}
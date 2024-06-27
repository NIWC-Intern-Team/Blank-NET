use pyo3::prelude::*;
use serde_json::Result;

fn sniffer(
    function: Bound<PyAny>,
) -> std::io::Result<(i32, i32, Option<String>, f64, String, Option<f64>)> {
    let func_call = function.call1(("wlx8c3badbc5be7",))?;
    let json_str: &str = func_call.extract()?;
    let tup: (i32, i32, Option<String>, f64, String, Option<f64>) = serde_json::from_str(json_str)?;
    Ok(tup)
}

pub fn scapy_analyzer_import() -> String {
    include_str!("../network_analyzer.py").into()
}

pub fn radio_metrics(code: &str) -> Result<(i32, i32, Option<String>, f64, String, Option<f64>)> {
    Python::with_gil(
        |py| -> Result<(i32, i32, Option<String>, f64, String, Option<f64>)> {
            let scapy_analyzer =
                PyModule::from_code_bound(py, code, "network_analyzer.py", "network_analyzer")
                    .expect("hmm");
            let function = scapy_analyzer.getattr("radioSniffer").unwrap();
            let res = sniffer(function).unwrap();
            return Ok(res);
        },
    )
}

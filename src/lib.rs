use ::bio_seq as bs;
use bs::codec::dna::Dna;
use bs::seq as bseq;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(sequence)]
struct Seq(bseq::Seq<Dna>);

#[pymethods]
impl Seq {
    #[new]
    #[pyo3(signature = (seq))]
    fn new(seq: String) -> PyResult<Self> {
        match bseq::Seq::try_from(seq) {
            Ok(s) => Ok(Seq(s)),
            Err(_) => Err(PyValueError::new_err("invalid sequence")),
        }
    }

    fn __repr__(&self) -> String {
        format!("Seq[Dna]({})", self.0)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn revcomp(&self) -> Self {
        todo!()
    }
}

#[pymodule]
fn bio_seq(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Seq>()?;
    Ok(())
}

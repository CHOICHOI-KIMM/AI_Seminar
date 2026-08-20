use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Clone)]
pub enum SolverError {
    #[error("Invalid geometry: {0}")]
    InvalidGeometry(String),

    #[error("Convergence failure: {0}")]
    ConvergenceFailure(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

}

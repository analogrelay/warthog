use std::fmt;

use std::borrow::Cow;

#[derive(PartialEq, Clone)]
pub enum AssertionOutcome {
    Success,
    Failure(Cow<'static, str>),
}

#[derive(PartialEq, Clone)]
pub struct AssertionResult {
    index: usize,
    assertion: String,
    outcome: AssertionOutcome,
}

impl AssertionResult {
    pub fn success<S: Into<String>>(index: usize, assertion: S) -> AssertionResult {
        AssertionResult {
            index,
            assertion: assertion.into(),
            outcome: AssertionOutcome::Success,
        }
    }

    pub fn failure<S: Into<String>, T: Into<Cow<'static, str>>>(
        index: usize,
        assertion: S,
        failure: T,
    ) -> AssertionResult {
        AssertionResult {
            index,
            assertion: assertion.into(),
            outcome: AssertionOutcome::Failure(failure.into()),
        }
    }

    pub fn assertion(&self) -> &str {
        &self.assertion
    }

    pub fn outcome(&self) -> &AssertionOutcome {
        &self.outcome
    }

    pub fn is_success(&self) -> bool {
        match self.outcome {
            AssertionOutcome::Success => true,
            _ => false,
        }
    }
}

impl fmt::Display for AssertionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.outcome() {
            AssertionOutcome::Success => {
                write!(f, "{:04} [success] {}", self.index, self.assertion())
            }
            AssertionOutcome::Failure(s) => write!(
                f,
                "{:04} [failure - {}] {}",
                self.index,
                s,
                self.assertion()
            ),
        }
    }
}

impl fmt::Debug for AssertionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

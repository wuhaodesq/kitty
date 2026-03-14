use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptValue {
    Str(String),
    Number(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    InvalidCommand(String),
    MissingArgument(&'static str),
    VariableNotFound(String),
    ParseNumberFailed(String),
}

#[derive(Debug, Default)]
pub struct ScriptRuntime {
    vars: HashMap<String, ScriptValue>,
}

impl ScriptRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self, script: &str) -> Result<Option<ScriptValue>, ScriptError> {
        let mut last = None;

        for raw in script.lines() {
            let line = raw.trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split_whitespace();
            let cmd = parts
                .next()
                .ok_or_else(|| ScriptError::InvalidCommand(line.to_string()))?;

            match cmd {
                "set" => {
                    let key = parts.next().ok_or(ScriptError::MissingArgument("name"))?;
                    let value = parts.collect::<Vec<_>>().join(" ");
                    if value.is_empty() {
                        return Err(ScriptError::MissingArgument("value"));
                    }
                    self.vars
                        .insert(key.to_string(), ScriptValue::Str(value.to_string()));
                    last = None;
                }
                "add" => {
                    let key = parts.next().ok_or(ScriptError::MissingArgument("name"))?;
                    let delta_str = parts.next().ok_or(ScriptError::MissingArgument("delta"))?;
                    let delta = delta_str
                        .parse::<i64>()
                        .map_err(|_| ScriptError::ParseNumberFailed(delta_str.to_string()))?;

                    let current = match self.vars.get(key) {
                        Some(ScriptValue::Number(n)) => *n,
                        Some(ScriptValue::Str(_)) => 0,
                        None => 0,
                    };
                    self.vars
                        .insert(key.to_string(), ScriptValue::Number(current + delta));
                    last = None;
                }
                "get" => {
                    let key = parts.next().ok_or(ScriptError::MissingArgument("name"))?;
                    let value = self
                        .vars
                        .get(key)
                        .cloned()
                        .ok_or_else(|| ScriptError::VariableNotFound(key.to_string()))?;
                    last = Some(value);
                }
                _ => return Err(ScriptError::InvalidCommand(cmd.to_string())),
            }
        }

        Ok(last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_string_value() {
        let mut rt = ScriptRuntime::new();
        let out = rt.execute("set title Kitty Browser\nget title").unwrap();
        assert_eq!(out, Some(ScriptValue::Str("Kitty Browser".to_string())));
    }

    #[test]
    fn add_number_value() {
        let mut rt = ScriptRuntime::new();
        let out = rt.execute("add counter 2\nadd counter 3\nget counter").unwrap();
        assert_eq!(out, Some(ScriptValue::Number(5)));
    }

    #[test]
    fn unknown_variable_returns_error() {
        let mut rt = ScriptRuntime::new();
        let err = rt.execute("get missing").unwrap_err();
        assert_eq!(err, ScriptError::VariableNotFound("missing".to_string()));
    }
}

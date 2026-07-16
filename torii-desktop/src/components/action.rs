//! This module manages component actions.

use std::error::Error;

/// Represents an action that can be performed on a component.
pub enum ComponentAction<ReturnType> {
    /// The component does not implement the request action. This is not an error,
    /// instead it's a permission signal.
    Unimplemented { reason: String },

    /// The component implements the requested action and provides a callback, which
    /// is not yet evaluated.
    Action {
        action: Box<dyn FnOnce() -> Result<ReturnType, Box<dyn Error>>>,
    },
}

impl<T> ComponentAction<T> {
    /// Invokes the action and returns the result.
    pub fn invoke(self) -> Result<T, Box<dyn Error>> {
        match self {
            ComponentAction::Unimplemented { reason } => Err(reason.clone().into()),
            ComponentAction::Action { action } => action(),
        }
    }

    /// Tests if the action is implemented.
    /// 
    /// # Example
    /// 
    /// ```
    /// use app_lib::components::ComponentAction;
    /// 
    /// let action: ComponentAction<()> = ComponentAction::Action { action: Box::new(|| Ok(())) };
    /// assert!(action.is_implemented());
    /// ```
    pub fn is_implemented(&self) -> bool {
        match self {
            ComponentAction::Unimplemented { .. } => false,
            ComponentAction::Action { .. } => true,
        }
    }

    /// Creates a new unimplemented action.
    /// 
    /// # Example
    /// 
    /// ```
    /// use app_lib::components::ComponentAction;
    /// 
    /// let action: ComponentAction<()> = ComponentAction::unimplemented("Not implemented");
    /// assert!(!action.is_implemented());
    /// ```
    pub fn unimplemented(reason: impl AsRef<str>) -> Self {
        ComponentAction::Unimplemented {
            reason: reason.as_ref().into(),
        }
    }
}

// impl<T> From<Box<dyn FnOnce() -> Result<T, Box<dyn Error>>>> for ComponentAction<T> {
//     fn from(value: Box<dyn FnOnce() -> Result<T, Box<dyn Error>>>) -> Self {
//         ComponentAction::Action { action: value }
//     }
// }

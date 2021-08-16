use std::fmt::Debug;

/// An `ICommand` represents business API call.
///
/// The name of an `ICommand` should always be in the present
/// tense, e.g.,
/// - `ChangeEmailAddress`
/// - `AddDependency`
///
/// To simplify serialization, a command should be an enum, and each
/// element should have a payload. By convention, the payload has the
/// same name as the element, and elements that do not require
/// additional information use an empty payload.
///
/// Though the `ICommand` trait only has a single function, the
/// commands must also derive a number of standard traits.
/// - `Debug` and `PartialEq` - needed for effective testing
///
/// # Examples
/// ```rust
/// use std::fmt::Debug;
///
/// use cqrs_es2::ICommand;
///
/// #[derive(Debug, PartialEq)]
/// pub enum CustomerCommand {
///     AddCustomerName(AddCustomerName),
///     UpdateEmail(UpdateEmail),
/// }
///
/// #[derive(Debug, PartialEq)]
/// pub struct AddCustomerName {
///     changed_name: String,
/// }
///
/// #[derive(Debug, PartialEq)]
/// pub struct UpdateEmail {
///     new_email: String,
/// }
///
/// impl ICommand for CustomerCommand {};
/// ```
pub trait ICommand: Debug + PartialEq + Sync + Send {}

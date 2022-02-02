/// Struct for base todo item
/// 
/// # Examples
/// 
/// ```
/// use to_do::structs::pending::Pending;
/// 
/// let pending = Pending::new("Buy milk"); 
/// ```
/// 
/// # Fields
/// 
/// * `title` - Title of the todo item
/// * `status` - Status of the todo item
pub struct Base {
    pub title: String
    pub status: String
}

impl Base {
  
    pub fn new(input_title: &str, input_status: &str) -> Base {
      return  Base {
            title: title.to_string(),
            status: input_status.to_string()
        }
    }
}
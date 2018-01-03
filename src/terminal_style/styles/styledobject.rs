use terminal_style::{ObjectStyle, Color};

/// Struct that contains both the style and the content wits is styled.
pub struct StyledObject<V>
{
    pub object_style: ObjectStyle,
    pub content: V,
}

impl<D> StyledObject<D>
{
    /// Paints the foreground color with the passed `Color` 
    /// 
    /// #Example 
    /// 
    /// ```rust
    /// extern crate crossterm;
    /// 
    /// use self::crossterm::terminal_style::{paint,Color};
    ///
    /// fn main()
    /// {   
    ///    // create an styled object with the foreground color red.        
    ///    let styledobject = paint("I am colored red").with(Color::Red);
    ///    // create an styled object with the foreground color blue.
    ///    let styledobject1 = paint("I am colored blue").with(Color::Blue);
    /// 
    ///    // print the styled objects
    ///    println!("{}", styledobject);
    ///    println!("{}", styledobject1);
    ///    // or print an styled object directly.
    ///    println!("{}", paint("I am colored green").with(Color::Green))
    /// }
    /// ```
    pub fn with(mut self, foreground_color: Color) -> StyledObject<D>
    {
        self.object_style = self.object_style.fg(foreground_color);
        self
    }

    
    /// Paints the background color with the passed `Color`
    /// 
    /// #Example 
    /// 
    /// ```rust
    /// extern crate crossterm;
    /// 
    /// use self::crossterm::terminal_style::{paint,Color};
    ///
    /// fn main()
    /// {   
    ///    // create an styled object with the background color red.        
    ///    let styledobject = paint("I am colored red").on(Color::Red);
    ///    // create an styled object with the background color blue.
    ///    let styledobject1 = paint("I am colored blue").on(Color::Blue);
    /// 
    ///    // print the styled objects
    ///    println!("{}", styledobject);
    ///    println!("{}", styledobject1);
    ///    // or print an styled object directly.
    ///    println!("{}", paint("I am colored green").on(Color::Green))
    /// }
    /// ```
    pub fn on(mut self, background_color: Color) -> StyledObject<D>
    {
        self.object_style = self.object_style.bg(background_color);
        self
    }
}



use crate::{Align, Justify, Length};

/// A container that distributes its contents vertically.
///
/// A [`Column`] will try to fill the horizontal space of its container.
///
/// [`Column`]: struct.Column.html
pub struct Column<Element> {
    pub spacing: u16,
    pub padding: u16,
    pub width: Length,
    pub height: Length,
    pub max_width: Length,
    pub max_height: Length,
    pub align_self: Option<Align>,
    pub align_items: Align,
    pub justify_content: Justify,
    pub children: Vec<Element>,
}

impl<Element> Column<Element> {
    /// Creates an empty [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn new() -> Self {
        Column {
            spacing: 0,
            padding: 0,
            width: Length::Fill,
            height: Length::Shrink,
            max_width: Length::Shrink,
            max_height: Length::Shrink,
            align_self: None,
            align_items: Align::Start,
            justify_content: Justify::Start,
            children: Vec::new(),
        }
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in Iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the padding of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn max_width(mut self, max_width: Length) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`Column`] in pixels.
    ///
    /// [`Column`]: struct.Column.html
    pub fn max_height(mut self, max_height: Length) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the alignment of the [`Column`] itself.
    ///
    /// This is useful if you want to override the default alignment given by
    /// the parent container.
    ///
    /// [`Column`]: struct.Column.html
    pub fn align_self(mut self, align: Align) -> Self {
        self.align_self = Some(align);
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`] .
    ///
    /// [`Column`]: struct.Column.html
    pub fn align_items(mut self, align: Align) -> Self {
        self.align_items = align;
        self
    }

    /// Sets the vertical distribution strategy for the contents of the
    /// [`Column`] .
    ///
    /// [`Column`]: struct.Column.html
    pub fn justify_content(mut self, justify: Justify) -> Self {
        self.justify_content = justify;
        self
    }

    /// Adds an element to the [`Column`].
    ///
    /// [`Column`]: struct.Column.html
    pub fn push<E>(mut self, child: E) -> Column<Element>
    where
        E: Into<Element>,
    {
        self.children.push(child.into());
        self
    }
}

impl<Element> Default for Column<Element> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Element> std::fmt::Debug for Column<Element>
where
    Element: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Complete once stabilized
        f.debug_struct("Column")
            .field("spacing", &self.spacing)
            .field("children", &self.children)
            .finish()
    }
}

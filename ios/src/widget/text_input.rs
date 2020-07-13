//! Display fields that can be filled with text.
//!
//! A [`TextInput`] has some local [`State`].
//!
//! [`TextInput`]: struct.TextInput.html
//! [`State`]: struct.State.html
use crate::{
    event::{EventHandler, WidgetEvent},
    Element, Hasher, Length, Widget,
    widget::{
        WidgetType,
        WidgetNode,
    },
};

pub use iced_style::text_input::{Style, StyleSheet};

use std::{rc::Rc, u32};

use std::convert::TryInto;
use uikit_sys::{
    id, CGPoint, CGRect, CGSize, INSNotificationCenter, INSObject, IUITextView,
    NSNotificationCenter, NSString, NSString_NSStringExtensionMethods,
    UITextView, UITextViewTextDidChangeNotification, UIView,
    UIView_UIViewHierarchy,
};

/// A field that can be filled with text.
///
/// # Example
/// ```
/// # use iced_web::{text_input, TextInput};
/// #
/// enum Message {
///     TextInputChanged(String),
/// }
///
/// let mut state = text_input::State::new();
/// let value = "Some text";
///
/// let input = TextInput::new(
///     &mut state,
///     "This is the placeholder...",
///     value,
///     Message::TextInputChanged,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct TextInput<'a, Message> {
    _state: &'a mut State,
    placeholder: String,
    value: String,
    is_secure: bool,
    width: Length,
    max_width: u32,
    padding: u16,
    size: Option<u16>,
    on_change: Rc<Box<dyn Fn(String) -> Message>>,
    on_submit: Option<Message>,
    style_sheet: Box<dyn StyleSheet>,
    widget_id: u64,
}

impl<'a, Message> TextInput<'a, Message> {
    /// Creates a new [`TextInput`].
    ///
    /// It expects:
    /// - some [`State`]
    /// - a placeholder
    /// - the current value
    /// - a function that produces a message when the [`TextInput`] changes
    ///
    /// [`TextInput`]: struct.TextInput.html
    /// [`State`]: struct.State.html
    pub fn new<F>(
        state: &'a mut State,
        placeholder: &str,
        value: &str,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(String) -> Message,
    {
        debug!("CREATING NEW TEXT INPUT");
        Self {
            _state: state,
            placeholder: String::from(placeholder),
            value: String::from(value),
            is_secure: false,
            width: Length::Fill,
            max_width: u32::MAX,
            padding: 0,
            size: None,
            on_change: Rc::new(Box::new(on_change)),
            on_submit: None,
            style_sheet: Default::default(),
            widget_id: 0,
        }
    }

    /// Converts the [`TextInput`] into a secure password input.
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn password(mut self) -> Self {
        self.is_secure = true;
        self
    }

    /// Sets the width of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the maximum width of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the padding of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the text size of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the message that should be produced when the [`TextInput`] is
    /// focused and the enter key is pressed.
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets the style of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for TextInput<'a, Message>
where
    Message: 'static + Clone,
{
    fn hash_layout(&self, state: &mut Hasher) {
        use std::{any::TypeId, hash::Hash};
        struct Marker;
        TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.max_width.hash(state);
        self.padding.hash(state);
        self.size.hash(state);
    }
    fn get_widget_type(&self) -> WidgetType {
        WidgetType::TextInput
    }

    fn update_or_add(&mut self, parent: Option<UIView>, old_node: Option<WidgetNode>,) -> WidgetNode {
        debug!("TEXT WIDGET ADD OR UPDATE old_node :{:?}", old_node);
        if let Some(old_node) = old_node {
            old_node
        } else {
            let textview = unsafe {
                let ui_textview = {
                    if parent.is_none() {
                        UITextView(UITextView::alloc().init())
                    } else {
                        let input_rect = CGRect {
                            origin: CGPoint { x: 10.0, y: 10.0 },
                            size: CGSize {
                                width: 100.0,
                                height: 100.0,
                            },
                        };
                        UITextView(UITextView::alloc().initWithFrame_textContainer_(
                                input_rect,
                                0 as id,
                        ))
                    }
                };
                let on_change = EventHandler::new(ui_textview.0);
                self.widget_id = on_change.widget_id;
                /*
                   input.addTarget_action_forControlEvents_(
                   on_change.id,
                   sel!(sendEvent),
                   uikit_sys::UIControlEvents_UIControlEventValueChanged,
                   );
                   */
                // https://developer.apple.com/documentation/foundation/nsnotificationcenter/1415360-addobserver?language=objc
                let center =
                    NSNotificationCenter(NSNotificationCenter::defaultCenter());
                center.addObserver_selector_name_object_(
                    on_change.id,
                    sel!(sendEvent),
                    UITextViewTextDidChangeNotification,
                    ui_textview.0,
                );
                //parent.addSubview_(ui_textview.0);
                if let Some(parent) = parent {
                    parent.addSubview_(ui_textview.0);
                }
                ui_textview
            };


            WidgetNode::new(Some(textview.0), WidgetType::TextInput)
        }

    }

    fn on_widget_event(
        &mut self,
        widget_event: WidgetEvent,
        messages: &mut Vec<Message>,
        widget_node: &WidgetNode,
    ) {
        debug!(
            "on_widget_event for text input: widget_event.id: {:x} for widget_id: {:?}, self.widget_id: {:?} widget_node.view_id {:?}",
            widget_event.id,
            widget_event.widget_id,
            self.widget_id,
            widget_node.view_id,
            );
        if Some(widget_event.id as id) == widget_node.view_id {
            let ui_textview = UITextView(widget_event.id as id);
            let value = unsafe {
                let value = NSString(ui_textview.text());
                let len = value
                    .lengthOfBytesUsingEncoding_(uikit_sys::NSUTF8StringEncoding);
                let bytes = value.UTF8String() as *const u8;
                String::from_utf8(
                    std::slice::from_raw_parts(bytes, len.try_into().unwrap())
                    .to_vec(),
                )
                    .unwrap()
            };
            if value.ends_with("\n") {
                if let Some(on_submit) = self.on_submit.take() {
                    messages.push(on_submit);
                }
            } else {
                self.value = value;

                messages.push((self.on_change)(self.value.clone()));
            }
        }

        /*
        debug!("on_widget_event TEXT UIVIEW {:?}", self.ui_textview.is_some());
        if let Some(ui_textview) = self.ui_textview {
            if widget_event.widget_id == self.widget_id {
                let value = unsafe {
                    let value = NSString(ui_textview.text());
                    let len = value.lengthOfBytesUsingEncoding_(
                        uikit_sys::NSUTF8StringEncoding,
                    );
                    let bytes = value.UTF8String() as *const u8;
                    String::from_utf8(std::slice::from_raw_parts(bytes, len.try_into().unwrap()).to_vec()).unwrap()
                };
                self.value = value;

                messages.push((self.on_change)(self.value.clone()));
            }
        }
        */
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        todo!()
    }
}

impl<'a, Message> From<TextInput<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(text_input: TextInput<'a, Message>) -> Element<'a, Message> {
        Element::new(text_input)
    }
}
impl<'a, Message> From<TextInput<'a, Message>> for WidgetNode {
    fn from(_text_input: TextInput<'a, Message>) -> WidgetNode {
        WidgetNode::new(None, WidgetType::TextInput)
    }
}

/// The state of a [`TextInput`].
///
/// [`TextInput`]: struct.TextInput.html
#[derive(Debug, Clone, Copy, Default)]
pub struct State;

impl State {
    /// Creates a new [`State`], representing an unfocused [`TextInput`].
    ///
    /// [`State`]: struct.State.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`State`], representing a focused [`TextInput`].
    ///
    /// [`State`]: struct.State.html
    pub fn focused() -> Self {
        // TODO
        Self::default()
    }
}
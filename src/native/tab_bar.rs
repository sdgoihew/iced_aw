//! Displays a [`TabBar`](TabBar) to select the content to be displayed.
//!
//! You have to manage the logic to show the contend by yourself or you may want
//! to use the [`Tabs`](super::tabs::Tabs) widget instead.
//!
//! *This API requires the following crate features to be activated: `tab_bar`*
use iced_native::{
    alignment::{Horizontal, Vertical},
    event,
    layout::{Limits, Node},
    mouse, renderer, touch,
    widget::{Column, Row, Text},
    Alignment, Clipboard, Color, Element, Event, Font, Layout, Length, Point, Rectangle, Shell,
    Widget,
};

pub mod tab_label;
pub use tab_label::TabLabel;

use crate::{
    graphics::icons,
    style::tab_bar::{Style, StyleSheet},
};

const DEFAULT_ICON_SIZE: u16 = 0;
const DEFAULT_TEXT_SIZE: u16 = 0;
const DEFAULT_CLOSE_SIZE: u16 = 0;
const DEFAULT_PADDING: u16 = 0;
const DEFAULT_SPACING: u16 = 0;

/// A tab bar to show tabs.
///
/// # Example
/// ```
/// # use iced_aw::{TabLabel};
/// # use iced_native::{renderer::Null};
/// #
/// # pub type TabBar<Message> = iced_aw::native::TabBar<Message, Null>;
/// #[derive(Debug, Clone)]
/// enum Message {
///     TabSelected(usize),
/// }
///
/// let active_tab = 0;
///
/// let tab_bar = TabBar::new(
///     active_tab,
///     Message::TabSelected,
/// )
/// .push(TabLabel::Text(String::from("One")))
/// .push(TabLabel::Text(String::from("Two")))
/// .push(TabLabel::Text(String::from("Three")));
/// ```
#[allow(missing_debug_implementations)]
pub struct TabBar<Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    /// The currently active tab.
    active_tab: usize,
    /// The vector containing the labels of the tabs.
    tab_labels: Vec<TabLabel>,
    /// The function that produces the message when a tab is selected.
    on_select: Box<dyn Fn(usize) -> Message>,
    /// The function that produces the message when the close icon was pressed.
    on_close: Option<Box<dyn Fn(usize) -> Message>>,
    /// The width of the [`TabBar`](TabBar).
    width: Length,
    /// The width of the tabs of the [`TabBar`](TabBar).
    tab_width: Length,
    /// The width of the [`TabBar`](TabBar).
    height: Length,
    /// The maximum height of the [`TabBar`](TabBar).
    max_height: u32,
    /// The icon size.
    icon_size: u16,
    /// The text size.
    text_size: u16,
    /// The size of the close icon.
    close_size: u16,
    /// The padding of the tabs of the [`TabBar`](TabBar).
    padding: u16,
    /// The spacing of the tabs of the [`TabBar`](TabBar).
    spacing: u16,
    /// The optional icon font of the [`TabBar`](TabBar).
    icon_font: Option<Renderer::Font>,
    /// The optional text font of the [`TabBar`](TabBar).
    text_font: Option<Renderer::Font>,
    /// The style of the [`TabBar`](TabBar).
    style_sheet: Box<dyn StyleSheet>,
}

impl<Message, Renderer> TabBar<Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    /// Creates a new [`TabBar`](TabBar) with the index of the selected tab and a
    /// specified message which will be send when a tab is selected by the user.
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn new<F>(active_tab: usize, on_select: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        Self::width_tab_labels(active_tab, Vec::new(), on_select)
    }

    /// Similar to `new` but with a given Vector of the
    /// [`TabLabel`](tab_label::TabLabel)s.Alignment
    ///
    /// It expects:
    ///     * the index of the currently active tab.
    ///     * a vector containing the [`TabLabel`](TabLabel)s of the [`TabBar`](TabBar).
    ///     * the function that will be called if a tab is selected by the user.
    ///         It takes the index of the selected tab.
    pub fn width_tab_labels<F>(active_tab: usize, tab_labels: Vec<TabLabel>, on_select: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        Self {
            active_tab,
            tab_labels,
            on_select: Box::new(on_select),
            on_close: None,
            width: Length::Fill,
            tab_width: Length::Fill,
            height: Length::Shrink,
            max_height: u32::MAX,
            icon_size: DEFAULT_ICON_SIZE,
            text_size: DEFAULT_TEXT_SIZE,
            close_size: DEFAULT_CLOSE_SIZE,
            padding: DEFAULT_PADDING,
            spacing: DEFAULT_SPACING,
            icon_font: None,
            text_font: None,
            style_sheet: Default::default(),
        }
    }

    /// Gets the index of the currently active tab on the [`TabBar`](TabBar).
    pub fn get_active_tab(&self) -> usize {
        self.active_tab
    }

    /// Sets the message that will be produced when the close icon of a tab
    /// on the [`TabBar`](TabBar) is pressed.
    ///
    /// Setting this enables the drawing of a close icon on the tabs.
    pub fn on_close<F>(mut self, on_close: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        self.on_close = Some(Box::new(on_close));
        self
    }

    /// Sets the width of the [`TabBar`](TabBar).
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Gets the width of the [`TabBar`](TabBar).
    pub fn get_width(&self) -> Length {
        self.width
    }

    /// Sets the width of a tab on the [`TabBar`](TabBar).
    pub fn tab_width(mut self, width: Length) -> Self {
        self.tab_width = width;
        self
    }

    /// Sets the height of the [`TabBar`](TabBar).
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Gets the width of the [`TabBar`](TabBar).
    pub fn get_height(&self) -> Length {
        self.height
    }

    /// Sets the maximum height of the [`TabBar`](TabBar).
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the icon size of the [`TabLabel`](tab_label::TabLabel)s of
    /// the [`TabBar`](TabBar).
    pub fn icon_size(mut self, icon_size: u16) -> Self {
        self.icon_size = icon_size;
        self
    }

    /// Sets the text size of the [`TabLabel`](tab_label::TabLabel)s of the
    /// [`TabBar`](TabBar).
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = text_size;
        self
    }

    /// Sets the size of the close icon of the
    /// [`TabLabel`](tab_label::TabLabel)s of the [`TabBar`](TabBar).
    pub fn close_size(mut self, close_size: u16) -> Self {
        self.close_size = close_size;
        self
    }

    /// Sets the padding of the tabs of the [`TabBar`](TabBar).
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing between the tabs of the [`TabBar`](TabBar).
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the font of the icons of the
    /// [`TabLabel`](tab_label::TabLabel)s of the [`TabBar`](TabBar).
    pub fn icon_font(mut self, icon_font: Renderer::Font) -> Self {
        self.icon_font = Some(icon_font);
        self
    }

    /// Sets the font of the text of the
    /// [`TabLabel`](tab_label::TabLabel)s of the [`TabBar`](TabBar).
    pub fn text_font(mut self, text_font: Renderer::Font) -> Self {
        self.text_font = Some(text_font);
        self
    }

    /// Sets the style of the [`TabBar`](TabBar).
    pub fn style_sheet(mut self, style_sheet: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style_sheet.into();
        self
    }

    /// Pushes a [`TabLabel`](tab_label::TabLabel) to the [`TabBar`](TabBar).
    pub fn push(mut self, tab_label: TabLabel) -> Self {
        self.tab_labels.push(tab_label);
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for TabBar<Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        self.tab_labels
            .iter()
            .fold(Row::<Message, Renderer>::new(), |row, tab_label| {
                let label = match tab_label {
                    TabLabel::Icon(_icon) => Column::new().align_items(Alignment::Center).push(
                        Row::new()
                            .width(Length::Units(self.icon_size))
                            .height(Length::Units(self.icon_size)),
                    ),
                    TabLabel::Text(text) => Column::new()
                        .align_items(Alignment::Center)
                        .push(Text::new(text).size(self.text_size).width(self.tab_width)),
                    TabLabel::IconText(_icon, text) => Column::new()
                        .align_items(Alignment::Center)
                        .push(
                            Row::new()
                                .width(Length::Units(self.icon_size))
                                .height(Length::Units(self.icon_size)),
                        )
                        .push(Text::new(text).size(self.text_size).width(self.tab_width)),
                }
                .width(self.tab_width)
                .height(self.height);

                let mut label_row = Row::new()
                    .align_items(Alignment::Center)
                    .padding(self.padding)
                    .width(self.tab_width)
                    .push(label);

                if self.on_close.is_some() {
                    label_row = label_row.push(
                        Row::new()
                            .width(Length::Units(self.close_size))
                            .height(Length::Units(self.close_size))
                            .align_items(Alignment::Center),
                    );
                }

                row.push(label_row)
            })
            .width(self.width)
            .height(self.height)
            .spacing(self.spacing)
            .layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if layout.bounds().contains(cursor_position) {
                    let tabs_map: Vec<bool> = layout
                        .children()
                        .map(|layout| layout.bounds().contains(cursor_position))
                        .collect();

                    if let Some(new_selected) = tabs_map.iter().position(|b| *b) {
                        shell.publish(
                            self.on_close
                                .as_ref()
                                .filter(|_on_close| {
                                    let tab_layout = layout.children().nth(new_selected).expect("Native: Layout should have a tab layout at the selected index");
                                    let cross_layout = tab_layout.children().nth(1).expect("Native: Layout should have a close layout");

                                    cross_layout.bounds().contains(cursor_position)
                                })
                                .map_or_else(
                                    || (self.on_select)(new_selected),
                                    |on_close| (on_close)(new_selected),
                                ),
                        );
                        return event::Status::Captured;
                    }
                }
                event::Status::Ignored
            }
            _ => event::Status::Ignored,
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        todo!()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _style: &iced_native::renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let children = layout.children();
        let is_mouse_over = bounds.contains(cursor_position);
        let style_sheet = if is_mouse_over {
            self.style_sheet.hovered(false)
        } else {
            self.style_sheet.active(false)
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: 0.0,
                border_width: style_sheet.border_width,
                border_color: style_sheet.border_color.unwrap_or(Color::TRANSPARENT),
            },
            style_sheet
                .background
                .unwrap_or_else(|| Color::TRANSPARENT.into()),
        );

        for ((i, tab), layout) in self.tab_labels.iter().enumerate().zip(children) {
            draw_tab(
                renderer,
                tab,
                layout,
                &self.style_sheet,
                i == self.active_tab,
                cursor_position,
                &self.icon_font.unwrap_or(Renderer::ICON_FONT),
                &self.text_font.unwrap_or_default(),
            );
        }
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        use std::hash::Hash;
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.active_tab.hash(state);
        self.tab_labels.iter().for_each(|tab| tab.hash(state));
        self.width.hash(state);
        self.height.hash(state);
        self.max_height.hash(state);
        self.icon_size.hash(state);
        self.text_size.hash(state);
        self.close_size.hash(state);
    }
}

#[allow(clippy::borrowed_box, clippy::too_many_lines)]
fn draw_tab<Renderer>(
    renderer: &mut Renderer,
    tab: &TabLabel,
    layout: Layout<'_>,
    style_sheet: &Box<dyn StyleSheet>,
    is_selected: bool,
    cursor_position: iced_native::Point,
    icon_font: &Renderer::Font,
    text_font: &Renderer::Font,
) where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    let is_mouse_over = layout.bounds().contains(cursor_position);
    let style = if is_mouse_over {
        style_sheet.hovered(is_selected)
    } else {
        style_sheet.active(is_selected)
    };

    let bounds = layout.bounds();
    let mut children = layout.children();
    let label_layout = children
        .next()
        .expect("Graphics: Layout should have a label layout");
    let mut label_layout_children = label_layout.children();

    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border_radius: 0.0,
            border_width: style.tab_label_border_width,
            border_color: style.tab_label_border_color,
        },
        style.tab_label_background,
    );

    match tab {
        TabLabel::Icon(icon) => {
            let icon_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have an icon layout for an Icon")
                .bounds();

            let mut buffer = [0; 4];
            let icon = icon.encode_utf8(&mut buffer);

            renderer.fill_text(iced_native::text::Text {
                content: icon,
                font: icon_font.clone(),
                size: icon_bounds.height,
                bounds: Rectangle {
                    x: icon_bounds.center_x(),
                    y: icon_bounds.center_y(),
                    ..icon_bounds
                },
                color: style.icon_color,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
            });
        }
        TabLabel::Text(text) => {
            let text_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have a text layout for a Text")
                .bounds();

            renderer.fill_text(iced_native::text::Text {
                content: &text[..],
                font: text_font,
                size: text_bounds.height,
                bounds: Rectangle {
                    x: text_bounds.center_x(),
                    y: text_bounds.center_y(),
                    ..text_bounds
                },
                color: style.text_color,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
            });
        }
        TabLabel::IconText(icon, text) => {
            let icon_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have an icons layout for an IconText")
                .bounds();
            let text_bounds = label_layout_children
                .next()
                .expect("Graphics: Layout should have a text layout for an IconText")
                .bounds();

            let mut buffer = [0; 4];
            let icon = icon.encode_utf8(&mut buffer);

            renderer.fill_text(iced_native::text::Text {
                content: icon,
                font: icon_font.clone(),
                size: icon_bounds.height,
                bounds: Rectangle {
                    x: icon_bounds.center_x(),
                    y: icon_bounds.center_y(),
                    ..icon_bounds
                },
                color: style.icon_color,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
            });

            renderer.fill_text(iced_native::text::Text {
                content: &text[..],
                font: text_font,
                size: text_bounds.height,
                bounds: Rectangle {
                    x: text_bounds.center_x(),
                    y: text_bounds.center_y(),
                    ..text_bounds
                },
                color: style.text_color,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
            });
        }
    };

    if let Some(cross_layout) = children.next() {
        let cross_bounds = cross_layout.bounds();
        let is_mouse_over_cross = cross_bounds.contains(cursor_position);

        let mut buffer = [0; 4];
        let icon = icons::icon_to_char(icons::Icon::X).encode_utf8(&mut buffer);

        renderer.fill_text(iced_native::text::Text {
            content: icon,
            font: icon_font,
            size: cross_bounds.height + if is_mouse_over_cross { 5.0 } else { 0.0 },
            bounds: Rectangle {
                x: cross_bounds.center_x(),
                y: cross_bounds.center_y(),
                ..cross_bounds
            },
            color: style.icon_color,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        });
    };
}

impl<'a, Message, Renderer> From<TabBar<Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer,
    Message: 'a,
{
    fn from(tab_bar: TabBar<Message, Renderer>) -> Self {
        Element::new(tab_bar)
    }
}

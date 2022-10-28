use tty_interface::Style;

/// A segment of multi-part formatted text content.
pub type Segment = Vec<Text>;

/// A collection of text segments representing each item in the drawer and each rendered
/// vertically-separated.
pub type DrawerContents = Vec<Segment>;

/// A tuple of text content and optional styling.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text(String, Option<Style>);

impl Text {
    /// Create a new, unstyled text segment.
    pub fn new(content: String) -> Self {
        Self(content, None)
    }

    /// Create a new, styled text segment.
    pub fn new_styled(content: String, style: Style) -> Self {
        Self(content, Some(style))
    }

    /// This text's content.
    pub fn content(&self) -> &str {
        &self.0
    }

    /// This text's styling, if specified.
    pub fn style(&self) -> Option<&Style> {
        self.1.as_ref()
    }

    /// This text as a single-element vector.
    pub fn as_segment(self) -> Segment {
        vec![self]
    }
}

/// Update a segment's style for some subset of its graphemes.
pub(crate) fn set_segment_style(segment: &mut Segment, start: usize, end: usize, style: Style) {
    let mut index = 0;
    let mut i = 0;
    loop {
        if i == segment.len() {
            break;
        }

        let text = &segment[i];

        let start_intersects = start > index && start < index + text.content().len();
        let end_intersects = end > index && end < index + text.content().len();

        if start_intersects {
            let (first, second) = split_text(text, start - index);

            segment[i] = first;
            segment.insert(i + 1, second);
        } else if end_intersects {
            let (first, second) = split_text(text, end - index);

            segment[i] = first;
            segment.insert(i + 1, second);
        }

        index += segment[i].content().len();
        i += 1;
    }

    index = 0;
    for text in segment {
        if index >= start && index < end {
            text.1 = Some(style);
        }

        index += text.content().len();
    }
}

pub(crate) fn split_text(text: &Text, index: usize) -> (Text, Text) {
    let (prefix, suffix) = text.0.split_at(index);

    let first = Text(prefix.to_string(), text.1);
    let second = Text(suffix.to_string(), text.1);

    (first, second)
}

#[cfg(test)]
mod tests {
    use tty_interface::{Color, Style};

    use crate::Text;

    use super::set_segment_style;

    macro_rules! text {
        ($content: expr) => {
            Text::new($content.to_string())
        };
    }

    macro_rules! style {
        ($color: expr) => {
            Style::default().set_foreground($color)
        };
    }

    macro_rules! text_styled {
        ($content: expr, $color: expr) => {
            Text::new_styled($content.to_string(), style!($color))
        };
    }

    #[test]
    fn test_set_segment_style_entirely() {
        let mut segment = vec![
            text!("TEST1"),
            text_styled!("TEST2", Color::Red),
            text_styled!("TEST3", Color::Blue),
            text!("TEST4"),
        ];

        set_segment_style(&mut segment, 0, 20, style!(Color::Green));

        assert_eq!(
            vec![
                text_styled!("TEST1", Color::Green),
                text_styled!("TEST2", Color::Green),
                text_styled!("TEST3", Color::Green),
                text_styled!("TEST4", Color::Green),
            ],
            segment
        );
    }

    #[test]
    fn test_set_segment_style_neatly() {
        let mut segment = vec![
            text!("TEST1"),
            text_styled!("TEST2", Color::Red),
            text_styled!("TEST3", Color::Blue),
            text!("TEST4"),
        ];

        set_segment_style(&mut segment, 5, 15, style!(Color::Green));

        assert_eq!(
            vec![
                text!("TEST1"),
                text_styled!("TEST2", Color::Green),
                text_styled!("TEST3", Color::Green),
                text!("TEST4"),
            ],
            segment
        );
    }

    #[test]
    fn test_set_segment_style_split() {
        let mut segment = vec![
            text!("TEST1"),
            text_styled!("TEST2", Color::Red),
            text_styled!("TEST3", Color::Blue),
            text!("TEST4"),
        ];

        set_segment_style(&mut segment, 3, 7, style!(Color::Green));
        set_segment_style(&mut segment, 11, 14, style!(Color::Magenta));

        assert_eq!(
            vec![
                text!("TES"),
                text_styled!("T1", Color::Green),
                text_styled!("TE", Color::Green),
                text_styled!("ST2", Color::Red),
                text_styled!("T", Color::Blue),
                text_styled!("EST", Color::Magenta),
                text_styled!("3", Color::Blue),
                text!("TEST4"),
            ],
            segment
        );
    }
}

use yoga::{get_node_ref_context, MeasureMode, NodeRef, Size};

const WIDTH_PER_CHAR: f32 = 10.0;
const HEIGHT_PER_CHAR: f32 = 10.0;

#[allow(dead_code)]
pub extern "C" fn intrinsic_measure_function(
    raw_node: NodeRef,
    width: f32,
    width_mode: MeasureMode,
    height: f32,
    height_mode: MeasureMode,
) -> Size {
    let context = get_node_ref_context(&raw_node);

    let text: &str = context
        .and_then(|ctx| {
            let s: Option<&String> = ctx.downcast_ref();
            s.map(|s| s.as_ref())
        })
        .unwrap_or("");

    let measured_width = match width_mode {
        MeasureMode::Exactly => width,
        MeasureMode::AtMost => (text.len() as f32 * WIDTH_PER_CHAR).min(width),
        MeasureMode::Undefined => text.len() as f32 * WIDTH_PER_CHAR,
    };

    let measured_height = match height_mode {
        MeasureMode::Exactly => height,
        MeasureMode::AtMost => {
            calculate_height(text, measured_width.max(get_widest_word_width(text))).min(height)
        },
        MeasureMode::Undefined => {
            calculate_height(text, measured_width.max(get_widest_word_width(text)))
        },
    };

    Size { width: measured_width, height: measured_height }
}

fn get_widest_word_width(text: &str) -> f32 {
    let widest_word_length = text.split(" ").map(|part| part.len()).max().unwrap_or(0);
    widest_word_length as f32 * WIDTH_PER_CHAR
}

fn calculate_height(text: &str, measured_width: f32) -> f32 {
    if text.len() as f32 * WIDTH_PER_CHAR <= measured_width {
        return HEIGHT_PER_CHAR;
    }

    let mut lines = 1;
    let mut current_line_length = 0.0;
    for word in text.split(" ") {
        let word_width = word.len() as f32 * WIDTH_PER_CHAR;
        if word_width > measured_width {
            if current_line_length > 0.0 {
                lines += 1;
            }
            lines += 1;
            current_line_length = 0.0;
        } else if current_line_length + word_width <= measured_width {
            current_line_length += word_width + WIDTH_PER_CHAR;
        } else {
            lines += 1;
            current_line_length = word_width + WIDTH_PER_CHAR;
        }
    }

    let lines = if current_line_length == 0.0 { lines - 1 } else { lines };

    lines as f32 * HEIGHT_PER_CHAR
}
